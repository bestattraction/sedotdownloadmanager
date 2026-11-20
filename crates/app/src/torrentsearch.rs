//! Integrated Torrent Search: Search torrents from public indexers directly within SDM.

use egui::{Color32, Rect, pos2, vec2};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::dialogs::{Outcome, window};
use crate::winui::{self, BG, ButtonKind, MUTED, TEXT, r, text};

#[derive(Clone, Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApibayItem {
    pub id: String,
    pub name: String,
    pub info_hash: String,
    pub leechers: String,
    pub seeders: String,
    pub num_files: String,
    pub size: String,
    #[serde(default)]
    pub added: String,
    pub category: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BitsearchItem {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub infohash: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub seeders: u32,
    #[serde(default)]
    pub leechers: u32,
    #[serde(default, rename = "createdAt")]
    pub created_at: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BitsearchResponse {
    #[serde(default)]
    pub results: Vec<BitsearchItem>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TorrentsCsvItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub infohash: String,
    #[serde(default)]
    pub size_bytes: u64,
    #[serde(default)]
    pub seeders: u32,
    #[serde(default)]
    pub leechers: u32,
    #[serde(default)]
    pub created_unix: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TorrentsCsvResponse {
    #[serde(default)]
    pub torrents: Vec<TorrentsCsvItem>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortColumn {
    Name,
    Size,
    Seeds,
    Peers,
    Date,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Clone, Debug)]
pub struct TorrentSearchResult {
    pub name: String,
    pub info_hash: String,
    pub size: u64,
    pub seeders: u32,
    pub leechers: u32,
    pub added: u64,
}

impl TorrentSearchResult {
    pub fn compare(&self, other: &Self, col: SortColumn, dir: SortDirection) -> std::cmp::Ordering {
        let ord = match col {
            SortColumn::Name => self.name.to_lowercase().cmp(&other.name.to_lowercase()),
            SortColumn::Size => self.size.cmp(&other.size),
            SortColumn::Seeds => self.seeders.cmp(&other.seeders),
            SortColumn::Peers => self.leechers.cmp(&other.leechers),
            SortColumn::Date => self.added.cmp(&other.added),
        };
        match dir {
            SortDirection::Ascending => ord,
            SortDirection::Descending => ord.reverse(),
        }
    }
}

pub fn format_date(secs: u64) -> String {
    if secs == 0 {
        return "-".to_string();
    }
    if let Some(dt) = chrono::DateTime::from_timestamp(secs as i64, 0) {
        dt.format("%Y-%m-%d").to_string()
    } else {
        "-".to_string()
    }
}

pub fn parse_rfc3339_date(s: &str) -> u64 {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.timestamp().max(0) as u64)
        .unwrap_or(0)
}

pub fn parse_rfc2822_date(s: &str) -> u64 {
    chrono::DateTime::parse_from_rfc2822(s)
        .map(|dt| dt.timestamp().max(0) as u64)
        .unwrap_or(0)
}

pub fn parse_size_str(s: &str) -> u64 {
    let s = s.trim();
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.is_empty() {
        return 0;
    }
    let val: f64 = parts[0].parse().unwrap_or(0.0);
    let unit = parts
        .get(1)
        .map(|u| u.to_ascii_uppercase())
        .unwrap_or_default();
    let mult: f64 = match unit.as_str() {
        "B" => 1.0,
        "KIB" | "KB" => 1024.0,
        "MIB" | "MB" => 1024.0 * 1024.0,
        "GIB" | "GB" => 1024.0 * 1024.0 * 1024.0,
        "TIB" | "TB" => 1024.0 * 1024.0 * 1024.0 * 1024.0,
        _ => 1.0,
    };
    (val * mult) as u64
}

pub fn parse_nyaa_rss(xml: &str) -> Vec<TorrentSearchResult> {
    let mut list = Vec::new();
    let opts = roxmltree::ParsingOptions {
        allow_dtd: false,
        nodes_limit: 20_000,
    };
    let Ok(doc) = roxmltree::Document::parse_with_options(xml, opts) else {
        return list;
    };
    for item in doc
        .descendants()
        .filter(|n| n.is_element() && n.tag_name().name().eq_ignore_ascii_case("item"))
    {
        let mut name = String::new();
        let mut hash = String::new();
        let mut seeders = 0u32;
        let mut leechers = 0u32;
        let mut size = 0u64;
        let mut added = 0u64;

        for child in item.children().filter(|c| c.is_element()) {
            let tag = child.tag_name().name().to_ascii_lowercase();
            let text = child.text().unwrap_or("").trim();
            match tag.as_str() {
                "title" => name = text.to_string(),
                "infohash" => hash = text.to_string(),
                "seeders" => seeders = text.parse().unwrap_or(0),
                "leechers" => leechers = text.parse().unwrap_or(0),
                "size" => size = parse_size_str(text),
                "pubdate" => added = parse_rfc2822_date(text),
                _ => {}
            }
        }
        let clean_hash = hash.trim().to_ascii_lowercase();
        if clean_hash.len() == 40 && !name.is_empty() {
            list.push(TorrentSearchResult {
                name,
                info_hash: clean_hash,
                size,
                seeders,
                leechers,
                added,
            });
        }
    }
    list
}

fn build_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(6))
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        )
        .build()
        .unwrap_or_default()
}

async fn search_apibay(client: &reqwest::Client, query: &str) -> Vec<TorrentSearchResult> {
    let encoded: String = url::form_urlencoded::byte_serialize(query.as_bytes()).collect();
    let url = format!("https://apibay.org/q.php?q={encoded}");
    let Ok(resp) = client.get(&url).send().await else {
        return Vec::new();
    };
    if !resp.status().is_success() {
        return Vec::new();
    }
    let Ok(items) = resp.json::<Vec<ApibayItem>>().await else {
        return Vec::new();
    };

    let mut results = Vec::new();
    for it in items {
        if it.name == "No results returned"
            || it.info_hash == "0000000000000000000000000000000000000000"
        {
            continue;
        }
        let hash = it.info_hash.trim().to_ascii_lowercase();
        if hash.len() == 40 {
            results.push(TorrentSearchResult {
                name: it.name,
                info_hash: hash,
                size: it.size.parse().unwrap_or(0),
                seeders: it.seeders.parse().unwrap_or(0),
                leechers: it.leechers.parse().unwrap_or(0),
                added: it.added.parse().unwrap_or(0),
            });
        }
    }
    results
}

async fn search_bitsearch(client: &reqwest::Client, query: &str) -> Vec<TorrentSearchResult> {
    let encoded: String = url::form_urlencoded::byte_serialize(query.as_bytes()).collect();
    let url = format!("https://bitsearch.to/api/v1/search?q={encoded}&limit=100&sort=seeders");
    let Ok(resp) = client.get(&url).send().await else {
        return Vec::new();
    };
    if !resp.status().is_success() {
        return Vec::new();
    }
    let Ok(data) = resp.json::<BitsearchResponse>().await else {
        return Vec::new();
    };

    let mut results = Vec::new();
    for it in data.results {
        let hash = it.infohash.trim().to_ascii_lowercase();
        if hash.len() != 40 || it.title.is_empty() {
            continue;
        }
        let added = it
            .created_at
            .as_deref()
            .map(parse_rfc3339_date)
            .unwrap_or(0);
        results.push(TorrentSearchResult {
            name: it.title,
            info_hash: hash,
            size: it.size,
            seeders: it.seeders,
            leechers: it.leechers,
            added,
        });
    }
    results
}

async fn search_solidtorrents(client: &reqwest::Client, query: &str) -> Vec<TorrentSearchResult> {
    let encoded: String = url::form_urlencoded::byte_serialize(query.as_bytes()).collect();
    let url = format!("https://solidtorrents.to/api/v1/search?q={encoded}&sort=seeders");
    let Ok(resp) = client.get(&url).send().await else {
        return Vec::new();
    };
    if !resp.status().is_success() {
        return Vec::new();
    }
    let Ok(data) = resp.json::<BitsearchResponse>().await else {
        return Vec::new();
    };

    let mut results = Vec::new();
    for it in data.results {
        let hash = it.infohash.trim().to_ascii_lowercase();
        if hash.len() != 40 || it.title.is_empty() {
            continue;
        }
        let added = it
            .created_at
            .as_deref()
            .map(parse_rfc3339_date)
            .unwrap_or(0);
        results.push(TorrentSearchResult {
            name: it.title,
            info_hash: hash,
            size: it.size,
            seeders: it.seeders,
            leechers: it.leechers,
            added,
        });
    }
    results
}

async fn search_torrents_csv(client: &reqwest::Client, query: &str) -> Vec<TorrentSearchResult> {
    let encoded: String = url::form_urlencoded::byte_serialize(query.as_bytes()).collect();
    let url = format!("https://torrents-csv.com/service/search?q={encoded}&size=100");
    let Ok(resp) = client.get(&url).send().await else {
        return Vec::new();
    };
    if !resp.status().is_success() {
        return Vec::new();
    }
    let Ok(data) = resp.json::<TorrentsCsvResponse>().await else {
        return Vec::new();
    };

    let mut results = Vec::new();
    for it in data.torrents {
        let hash = it.infohash.trim().to_ascii_lowercase();
        if hash.len() != 40 || it.name.is_empty() {
            continue;
        }
        results.push(TorrentSearchResult {
            name: it.name,
            info_hash: hash,
            size: it.size_bytes,
            seeders: it.seeders,
            leechers: it.leechers,
            added: it.created_unix,
        });
    }
    results
}

async fn search_nyaa(client: &reqwest::Client, query: &str) -> Vec<TorrentSearchResult> {
    let encoded: String = url::form_urlencoded::byte_serialize(query.as_bytes()).collect();
    let url = format!("https://nyaa.si/?page=rss&q={encoded}&s=seeders&o=desc");
    let Ok(resp) = client.get(&url).send().await else {
        return Vec::new();
    };
    if !resp.status().is_success() {
        return Vec::new();
    }
    let Ok(xml) = resp.text().await else {
        return Vec::new();
    };

    parse_nyaa_rss(&xml)
}

pub struct TorrentSearch {
    pub query: String,
    pub searching: Arc<AtomicBool>,
    pub status_text: Arc<Mutex<String>>,
    pub results: Arc<Mutex<Vec<TorrentSearchResult>>>,
    pub selected: Option<usize>,
    pub focused: bool,
    pub scroll_offset: f32,
    pub sort_col: SortColumn,
    pub sort_dir: SortDirection,
}

impl Default for TorrentSearch {
    fn default() -> Self {
        Self::new(None)
    }
}

impl TorrentSearch {
    pub fn new(initial: Option<String>) -> Self {
        Self {
            query: initial.unwrap_or_default(),
            searching: Arc::new(AtomicBool::new(false)),
            status_text: Arc::new(Mutex::new(String::new())),
            results: Arc::new(Mutex::new(Vec::new())),
            selected: None,
            focused: false,
            scroll_offset: 0.0,
            sort_col: SortColumn::Seeds,
            sort_dir: SortDirection::Descending,
        }
    }

    pub fn apply_sort(&mut self) {
        let col = self.sort_col;
        let dir = self.sort_dir;
        let mut list = self.results.lock().unwrap();
        list.sort_by(|a, b| a.compare(b, col, dir));
    }

    pub fn toggle_sort(&mut self, col: SortColumn) {
        if self.sort_col == col {
            self.sort_dir = match self.sort_dir {
                SortDirection::Ascending => SortDirection::Descending,
                SortDirection::Descending => SortDirection::Ascending,
            };
        } else {
            self.sort_col = col;
            self.sort_dir = match col {
                SortColumn::Name => SortDirection::Ascending,
                _ => SortDirection::Descending,
            };
        }
        self.apply_sort();
    }

    pub fn search(&mut self, rt: &tokio::runtime::Handle, ctx: egui::Context) {
        let q = self.query.trim().to_owned();
        if q.is_empty() {
            return;
        }
        self.searching.store(true, Ordering::Relaxed);
        *self.status_text.lock().unwrap() =
            crate::i18n::t("Searching public indexers...").to_owned();
        self.selected = None;
        self.results.lock().unwrap().clear();
        let results_cell = self.results.clone();
        let searching_cell = self.searching.clone();
        let status_cell = self.status_text.clone();
        let sort_col = self.sort_col;
        let sort_dir = self.sort_dir;

        rt.spawn(async move {
            let client = build_client();

            let (r_apibay, r_bitsearch, r_nyaa, r_torrents_csv, r_solid) = tokio::join!(
                search_apibay(&client, &q),
                search_bitsearch(&client, &q),
                search_nyaa(&client, &q),
                search_torrents_csv(&client, &q),
                search_solidtorrents(&client, &q),
            );

            let mut map: HashMap<String, TorrentSearchResult> = HashMap::new();

            for it in r_apibay
                .into_iter()
                .chain(r_bitsearch)
                .chain(r_nyaa)
                .chain(r_torrents_csv)
                .chain(r_solid)
            {
                let key = it.info_hash.to_ascii_lowercase();
                if key.len() != 40 || key == "0000000000000000000000000000000000000000" {
                    continue;
                }
                match map.get_mut(&key) {
                    Some(existing) => {
                        if it.seeders > existing.seeders {
                            existing.seeders = it.seeders;
                            existing.leechers = it.leechers;
                        }
                        if existing.size == 0 && it.size > 0 {
                            existing.size = it.size;
                        }
                        if existing.added == 0 && it.added > 0 {
                            existing.added = it.added;
                        }
                    }
                    None => {
                        map.insert(key, it);
                    }
                }
            }

            let mut list: Vec<TorrentSearchResult> = map.into_values().collect();
            list.sort_by(|a, b| a.compare(b, sort_col, sort_dir));

            let count = list.len();
            *results_cell.lock().unwrap() = list;
            searching_cell.store(false, Ordering::Relaxed);
            *status_cell.lock().unwrap() = if count == 0 {
                crate::i18n::t("No torrents found").to_owned()
            } else {
                format!(
                    "{} {}",
                    count,
                    crate::i18n::tn(count as u64, "result found", "results found")
                )
            };
            ctx.request_repaint();
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, rt: &tokio::runtime::Handle) -> Outcome<String> {
        let mut download_magnet = None;

        window(
            ctx,
            "torrent-search",
            "Torrent Search",
            [780.0, 520.0],
            false,
            BG(),
            |ui, o| {
                let p = ui.painter().clone();
                let is_searching = self.searching.load(Ordering::Relaxed);
                let can_search = !self.query.trim().is_empty() && !is_searching;

                // Search bar input
                let edit = winui::edit(
                    ui,
                    r(o, 14.0, 14.0, 630.0, 24.0),
                    &mut self.query,
                    !is_searching,
                    false,
                );
                if !self.focused {
                    edit.request_focus();
                    self.focused = true;
                }

                // Search button
                let enter = edit.has_focus()
                    && winui::enter_pressed(ui)
                    && winui::take_key(ui, egui::Key::Enter);
                let btn = winui::button(
                    ui,
                    r(o, 654.0, 14.0, 112.0, 24.0),
                    if is_searching {
                        crate::i18n::mark("Searching...")
                    } else {
                        crate::i18n::mark("Search")
                    },
                    if can_search {
                        ButtonKind::Default
                    } else {
                        ButtonKind::Disabled
                    },
                    false,
                    None,
                )
                .clicked();

                if (btn || enter) && can_search {
                    self.search(rt, ui.ctx().clone());
                }

                // Update search status text
                let items = self.results.lock().unwrap().clone();
                let status_str = self.status_text.lock().unwrap().clone();
                text(&p, o.x + 14.0, o.y + 57.0, &status_str, MUTED());

                // Sort By Toolbar (Explicit buttons)
                let sort_label = crate::i18n::t("Sort by:");
                text(&p, o.x + 270.0, o.y + 57.0, sort_label, MUTED());

                let arrow = |dir: SortDirection| {
                    if dir == SortDirection::Ascending {
                        " ▲"
                    } else {
                        " ▼"
                    }
                };

                let col_kind = |target: SortColumn, current: SortColumn| {
                    if target == current {
                        ButtonKind::Default
                    } else {
                        ButtonKind::Normal
                    }
                };

                let sort_name_btn = format!(
                    "Name{}",
                    if self.sort_col == SortColumn::Name {
                        arrow(self.sort_dir)
                    } else {
                        ""
                    }
                );
                if winui::button(
                    ui,
                    r(o, 320.0, 44.0, 80.0, 22.0),
                    &sort_name_btn,
                    col_kind(SortColumn::Name, self.sort_col),
                    false,
                    None,
                )
                .clicked()
                {
                    self.toggle_sort(SortColumn::Name);
                }

                let sort_size_btn = format!(
                    "Size{}",
                    if self.sort_col == SortColumn::Size {
                        arrow(self.sort_dir)
                    } else {
                        ""
                    }
                );
                if winui::button(
                    ui,
                    r(o, 404.0, 44.0, 80.0, 22.0),
                    &sort_size_btn,
                    col_kind(SortColumn::Size, self.sort_col),
                    false,
                    None,
                )
                .clicked()
                {
                    self.toggle_sort(SortColumn::Size);
                }

                let sort_seeds_btn = format!(
                    "Seeds{}",
                    if self.sort_col == SortColumn::Seeds {
                        arrow(self.sort_dir)
                    } else {
                        ""
                    }
                );
                if winui::button(
                    ui,
                    r(o, 488.0, 44.0, 84.0, 22.0),
                    &sort_seeds_btn,
                    col_kind(SortColumn::Seeds, self.sort_col),
                    false,
                    None,
                )
                .clicked()
                {
                    self.toggle_sort(SortColumn::Seeds);
                }

                let sort_peers_btn = format!(
                    "Peers{}",
                    if self.sort_col == SortColumn::Peers {
                        arrow(self.sort_dir)
                    } else {
                        ""
                    }
                );
                if winui::button(
                    ui,
                    r(o, 576.0, 44.0, 84.0, 22.0),
                    &sort_peers_btn,
                    col_kind(SortColumn::Peers, self.sort_col),
                    false,
                    None,
                )
                .clicked()
                {
                    self.toggle_sort(SortColumn::Peers);
                }

                let sort_date_btn = format!(
                    "Date{}",
                    if self.sort_col == SortColumn::Date {
                        arrow(self.sort_dir)
                    } else {
                        ""
                    }
                );
                if winui::button(
                    ui,
                    r(o, 664.0, 44.0, 80.0, 22.0),
                    &sort_date_btn,
                    col_kind(SortColumn::Date, self.sort_col),
                    false,
                    None,
                )
                .clicked()
                {
                    self.toggle_sort(SortColumn::Date);
                }

                // Table outer container
                let table_rect = r(o, 14.0, 72.0, 752.0, 396.0);
                winui::solid(&p, table_rect, winui::FIELD_EDGE());
                let header_h = 24.0;
                let head_rect =
                    Rect::from_min_size(table_rect.min, vec2(table_rect.width(), header_h));
                p.rect_filled(head_rect, 0.0, winui::LIST_HEADER());

                // Interactive column headers
                let col_name_rect = Rect::from_min_max(
                    pos2(table_rect.min.x, table_rect.min.y),
                    pos2(table_rect.min.x + 400.0, table_rect.min.y + header_h),
                );
                let col_size_rect = Rect::from_min_max(
                    pos2(table_rect.min.x + 400.0, table_rect.min.y),
                    pos2(table_rect.min.x + 490.0, table_rect.min.y + header_h),
                );
                let col_seeds_rect = Rect::from_min_max(
                    pos2(table_rect.min.x + 490.0, table_rect.min.y),
                    pos2(table_rect.min.x + 570.0, table_rect.min.y + header_h),
                );
                let col_peers_rect = Rect::from_min_max(
                    pos2(table_rect.min.x + 570.0, table_rect.min.y),
                    pos2(table_rect.min.x + 650.0, table_rect.min.y + header_h),
                );
                let col_date_rect = Rect::from_min_max(
                    pos2(table_rect.min.x + 650.0, table_rect.min.y),
                    pos2(table_rect.max.x, table_rect.min.y + header_h),
                );

                if ui
                    .allocate_rect(col_name_rect, egui::Sense::click())
                    .clicked()
                {
                    self.toggle_sort(SortColumn::Name);
                }
                if ui
                    .allocate_rect(col_size_rect, egui::Sense::click())
                    .clicked()
                {
                    self.toggle_sort(SortColumn::Size);
                }
                if ui
                    .allocate_rect(col_seeds_rect, egui::Sense::click())
                    .clicked()
                {
                    self.toggle_sort(SortColumn::Seeds);
                }
                if ui
                    .allocate_rect(col_peers_rect, egui::Sense::click())
                    .clicked()
                {
                    self.toggle_sort(SortColumn::Peers);
                }
                if ui
                    .allocate_rect(col_date_rect, egui::Sense::click())
                    .clicked()
                {
                    self.toggle_sort(SortColumn::Date);
                }

                // Column header labels with indicators
                let name_hdr = format!(
                    "{} {}",
                    crate::i18n::t("Name"),
                    if self.sort_col == SortColumn::Name {
                        arrow(self.sort_dir)
                    } else {
                        ""
                    }
                );
                let size_hdr = format!(
                    "{} {}",
                    crate::i18n::t("Size"),
                    if self.sort_col == SortColumn::Size {
                        arrow(self.sort_dir)
                    } else {
                        ""
                    }
                );
                let seeds_hdr = format!(
                    "{} {}",
                    crate::i18n::t("Seeds"),
                    if self.sort_col == SortColumn::Seeds {
                        arrow(self.sort_dir)
                    } else {
                        ""
                    }
                );
                let peers_hdr = format!(
                    "{} {}",
                    crate::i18n::t("Peers"),
                    if self.sort_col == SortColumn::Peers {
                        arrow(self.sort_dir)
                    } else {
                        ""
                    }
                );
                let date_hdr = format!(
                    "{} {}",
                    crate::i18n::t("Date"),
                    if self.sort_col == SortColumn::Date {
                        arrow(self.sort_dir)
                    } else {
                        ""
                    }
                );

                text(&p, o.x + 22.0, o.y + 88.0, &name_hdr, TEXT());
                text(&p, o.x + 420.0, o.y + 88.0, &size_hdr, TEXT());
                text(
                    &p,
                    o.x + 510.0,
                    o.y + 88.0,
                    &seeds_hdr,
                    Color32::from_rgb(0, 180, 0),
                );
                text(
                    &p,
                    o.x + 590.0,
                    o.y + 88.0,
                    &peers_hdr,
                    Color32::from_rgb(60, 140, 220),
                );
                text(&p, o.x + 670.0, o.y + 88.0, &date_hdr, TEXT());

                // Scrollable rows
                let row_h = 22.0;
                let visible_rows = 16;
                let total_rows = items.len();
                let max_scroll = (total_rows.saturating_sub(visible_rows) as f32 * row_h).max(0.0);

                // Handle mouse scroll
                if ui.rect_contains_pointer(table_rect) {
                    let dy = ui.input(|i| i.smooth_scroll_delta.y);
                    if dy != 0.0 {
                        self.scroll_offset = (self.scroll_offset - dy * 0.5).clamp(0.0, max_scroll);
                    }
                }

                let first_idx = (self.scroll_offset / row_h).floor() as usize;
                let last_idx = (first_idx + visible_rows + 1).min(total_rows);

                let body_rect = Rect::from_min_size(
                    pos2(table_rect.min.x, table_rect.min.y + header_h),
                    vec2(table_rect.width(), table_rect.height() - header_h),
                );
                p.rect_filled(body_rect, 0.0, winui::FIELD());

                let mut double_clicked = false;
                for i in first_idx..last_idx {
                    if let Some(item) = items.get(i) {
                        let y = body_rect.min.y + (i as f32 * row_h - self.scroll_offset);
                        if y + row_h <= body_rect.min.y || y >= body_rect.max.y {
                            continue;
                        }
                        let row_rc = Rect::from_min_size(
                            pos2(body_rect.min.x, y),
                            vec2(body_rect.width(), row_h),
                        );
                        let is_sel = self.selected == Some(i);

                        let resp = ui.allocate_rect(row_rc, egui::Sense::click());
                        if resp.clicked() {
                            self.selected = Some(i);
                        }
                        if resp.double_clicked() {
                            self.selected = Some(i);
                            double_clicked = true;
                        }

                        if is_sel {
                            p.rect_filled(row_rc, 0.0, winui::SELECTION());
                        }

                        let text_color = if is_sel { Color32::WHITE } else { TEXT() };
                        let (fit_name, _) = winui::text_fit(
                            &p,
                            o.x + 22.0,
                            y + 16.0,
                            &item.name,
                            text_color,
                            o.x + 400.0,
                        );
                        let _ = fit_name;
                        text(
                            &p,
                            o.x + 420.0,
                            y + 16.0,
                            &crate::model::size(item.size),
                            text_color,
                        );
                        text(
                            &p,
                            o.x + 510.0,
                            y + 16.0,
                            &item.seeders.to_string(),
                            Color32::from_rgb(0, 180, 0),
                        );
                        text(
                            &p,
                            o.x + 590.0,
                            y + 16.0,
                            &item.leechers.to_string(),
                            Color32::from_rgb(60, 140, 220),
                        );
                        text(
                            &p,
                            o.x + 670.0,
                            y + 16.0,
                            &format_date(item.added),
                            text_color,
                        );
                    }
                }

                // Bottom Action buttons
                let has_sel = self.selected.is_some_and(|idx| idx < items.len());

                let copy_magnet = winui::button(
                    ui,
                    r(o, 410.0, 480.0, 120.0, 26.0),
                    crate::i18n::mark("Copy Magnet"),
                    if has_sel {
                        ButtonKind::Normal
                    } else {
                        ButtonKind::Disabled
                    },
                    false,
                    None,
                )
                .clicked();

                let download = winui::button(
                    ui,
                    r(o, 540.0, 480.0, 110.0, 26.0),
                    crate::i18n::mark("Download"),
                    if has_sel {
                        ButtonKind::Default
                    } else {
                        ButtonKind::Disabled
                    },
                    false,
                    None,
                )
                .clicked();

                let close = winui::button(
                    ui,
                    r(o, 660.0, 480.0, 106.0, 26.0),
                    crate::i18n::mark("Close"),
                    ButtonKind::Normal,
                    false,
                    None,
                )
                .clicked();

                if let Some(item) = self.selected.and_then(|sel| items.get(sel)) {
                    let dn: String =
                        url::form_urlencoded::byte_serialize(item.name.as_bytes()).collect();
                    let base_magnet = format!("magnet:?xt=urn:btih:{}&dn={dn}", item.info_hash);
                    let full_magnet = mdm_torrent::trackers::append_to_magnet(
                        &base_magnet,
                        &crate::torrents::current_public_trackers(),
                    );
                    if copy_magnet {
                        ui.ctx().copy_text(full_magnet.clone());
                    }
                    if download || double_clicked {
                        download_magnet = Some(full_magnet);
                    }
                }

                if winui::escape_pressed(ui) || close {
                    Outcome::Closed
                } else if let Some(mag) = download_magnet.take() {
                    Outcome::Done(mag)
                } else {
                    Outcome::Open
                }
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size_str() {
        assert_eq!(parse_size_str("500 B"), 500);
        assert_eq!(parse_size_str("1.5 KiB"), (1.5 * 1024.0) as u64);
        assert_eq!(
            parse_size_str("686.8 MiB"),
            (686.8 * 1024.0 * 1024.0) as u64
        );
        assert_eq!(
            parse_size_str("1.5 GiB"),
            (1.5 * 1024.0 * 1024.0 * 1024.0) as u64
        );
    }

    #[test]
    fn test_parse_nyaa_rss() {
        let sample = r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:nyaa="https://nyaa.si/xmlns/nyaa">
  <channel>
    <title>Nyaa</title>
    <item>
      <title>[Test] Sample Anime 1080p</title>
      <nyaa:infoHash>f5df914b05b9e0760d621f2f3d55e08b1ecc8082</nyaa:infoHash>
      <nyaa:seeders>123</nyaa:seeders>
      <nyaa:leechers>45</nyaa:leechers>
      <nyaa:size>1.5 GiB</nyaa:size>
      <pubDate>Mon, 16 Nov 2026 09:18:49 -0000</pubDate>
    </item>
  </channel>
</rss>"#;
        let items = parse_nyaa_rss(sample);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "[Test] Sample Anime 1080p");
        assert_eq!(
            items[0].info_hash,
            "f5df914b05b9e0760d621f2f3d55e08b1ecc8082"
        );
        assert_eq!(items[0].seeders, 123);
        assert_eq!(items[0].leechers, 45);
        assert_eq!(items[0].size, (1.5 * 1024.0 * 1024.0 * 1024.0) as u64);
        assert!(items[0].added > 0);
    }

    #[test]
    fn test_torrent_search_compare() {
        let r1 = TorrentSearchResult {
            name: "Anime A".into(),
            info_hash: "1111111111111111111111111111111111111111".into(),
            size: 1000,
            seeders: 50,
            leechers: 10,
            added: 100,
        };
        let r2 = TorrentSearchResult {
            name: "Anime B".into(),
            info_hash: "2222222222222222222222222222222222222222".into(),
            size: 2000,
            seeders: 10,
            leechers: 5,
            added: 200,
        };
        assert_eq!(
            r1.compare(&r2, SortColumn::Seeds, SortDirection::Descending),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            r1.compare(&r2, SortColumn::Size, SortDirection::Descending),
            std::cmp::Ordering::Greater
        );
    }
}
