//! Integrated Torrent Search: Search torrents from public indexers directly within SDM.

use std::sync::{Arc, Mutex};
use egui::{pos2, vec2, Color32, Rect};
use serde::Deserialize;

use crate::dialogs::{Outcome, window};
use crate::winui::{self, ButtonKind, r, text, TEXT, MUTED, BG};

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

pub struct TorrentSearch {
    pub query: String,
    pub searching: bool,
    pub status_text: String,
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
            searching: false,
            status_text: String::new(),
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
        self.searching = true;
        self.status_text = crate::i18n::t("Searching public indexers...").to_owned();
        self.selected = None;
        let results_cell = self.results.clone();
        let sort_col = self.sort_col;
        let sort_dir = self.sort_dir;
        rt.spawn(async move {
            let encoded: String = url::form_urlencoded::byte_serialize(q.as_bytes()).collect();
            let url = format!("https://apibay.org/q.php?q={encoded}");
            let res: Result<Vec<ApibayItem>, String> = match reqwest::get(&url).await {
                Ok(resp) if resp.status().is_success() => {
                    resp.json::<Vec<ApibayItem>>().await.map_err(|e| e.to_string())
                }
                Ok(resp) => Err(format!("HTTP error: {}", resp.status())),
                Err(e) => Err(format!("Request error: {e}")),
            };
            match res {
                Ok(items) => {
                    let mut list = Vec::new();
                    for item in items {
                        if item.name == "No results returned"
                            || item.info_hash == "0000000000000000000000000000000000000000"
                        {
                            continue;
                        }
                        list.push(TorrentSearchResult {
                            name: item.name,
                            info_hash: item.info_hash,
                            size: item.size.parse().unwrap_or(0),
                            seeders: item.seeders.parse().unwrap_or(0),
                            leechers: item.leechers.parse().unwrap_or(0),
                            added: item.added.parse().unwrap_or(0),
                        });
                    }
                    list.sort_by(|a, b| a.compare(b, sort_col, sort_dir));
                    *results_cell.lock().unwrap() = list;
                }
                Err(e) => {
                    tracing::warn!("torrent search error: {e:#}");
                }
            }
            ctx.request_repaint();
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, rt: &tokio::runtime::Handle) -> Outcome<String> {
        if !self.query.trim().is_empty()
            && !self.searching
            && self.results.lock().unwrap().is_empty()
            && self.status_text.is_empty()
        {
            self.search(rt, ctx.clone());
        }
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
                let can_search = !self.query.trim().is_empty() && !self.searching;

                // Search bar input
                let edit = winui::edit(
                    ui,
                    r(o, 14.0, 14.0, 630.0, 24.0),
                    &mut self.query,
                    !self.searching,
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
                    if self.searching {
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

                // Update search status
                let items = self.results.lock().unwrap().clone();
                if self.searching && !items.is_empty() {
                    self.searching = false;
                    self.status_text = format!(
                        "{} {}",
                        items.len(),
                        crate::i18n::tn(items.len() as u64, "result found", "results found")
                    );
                } else if self.searching && items.is_empty() {
                    // Still searching or 0
                } else if !self.searching && items.is_empty() && !self.query.trim().is_empty() {
                    self.status_text = crate::i18n::t("No torrents found").to_owned();
                }

                text(&p, o.x + 14.0, o.y + 57.0, &self.status_text, MUTED());

                // Sort By Toolbar (Explicit buttons)
                let sort_label = crate::i18n::t("Sort by:");
                text(&p, o.x + 270.0, o.y + 57.0, sort_label, MUTED());

                let arrow = |dir: SortDirection| {
                    if dir == SortDirection::Ascending { " ▲" } else { " ▼" }
                };

                let col_kind = |target: SortColumn, current: SortColumn| {
                    if target == current { ButtonKind::Default } else { ButtonKind::Normal }
                };

                let sort_name_btn = format!("Name{}", if self.sort_col == SortColumn::Name { arrow(self.sort_dir) } else { "" });
                if winui::button(ui, r(o, 320.0, 44.0, 80.0, 22.0), &sort_name_btn, col_kind(SortColumn::Name, self.sort_col), false, None).clicked() {
                    self.toggle_sort(SortColumn::Name);
                }

                let sort_size_btn = format!("Size{}", if self.sort_col == SortColumn::Size { arrow(self.sort_dir) } else { "" });
                if winui::button(ui, r(o, 404.0, 44.0, 80.0, 22.0), &sort_size_btn, col_kind(SortColumn::Size, self.sort_col), false, None).clicked() {
                    self.toggle_sort(SortColumn::Size);
                }

                let sort_seeds_btn = format!("Seeds{}", if self.sort_col == SortColumn::Seeds { arrow(self.sort_dir) } else { "" });
                if winui::button(ui, r(o, 488.0, 44.0, 84.0, 22.0), &sort_seeds_btn, col_kind(SortColumn::Seeds, self.sort_col), false, None).clicked() {
                    self.toggle_sort(SortColumn::Seeds);
                }

                let sort_peers_btn = format!("Peers{}", if self.sort_col == SortColumn::Peers { arrow(self.sort_dir) } else { "" });
                if winui::button(ui, r(o, 576.0, 44.0, 84.0, 22.0), &sort_peers_btn, col_kind(SortColumn::Peers, self.sort_col), false, None).clicked() {
                    self.toggle_sort(SortColumn::Peers);
                }

                let sort_date_btn = format!("Date{}", if self.sort_col == SortColumn::Date { arrow(self.sort_dir) } else { "" });
                if winui::button(ui, r(o, 664.0, 44.0, 80.0, 22.0), &sort_date_btn, col_kind(SortColumn::Date, self.sort_col), false, None).clicked() {
                    self.toggle_sort(SortColumn::Date);
                }

                // Table outer container
                let table_rect = r(o, 14.0, 72.0, 752.0, 396.0);
                winui::solid(&p, table_rect, winui::FIELD_EDGE());
                let header_h = 24.0;
                let head_rect = Rect::from_min_size(table_rect.min, vec2(table_rect.width(), header_h));
                p.rect_filled(head_rect, 0.0, winui::LIST_HEADER());

                // Interactive column headers
                let col_name_rect = Rect::from_min_max(pos2(table_rect.min.x, table_rect.min.y), pos2(table_rect.min.x + 400.0, table_rect.min.y + header_h));
                let col_size_rect = Rect::from_min_max(pos2(table_rect.min.x + 400.0, table_rect.min.y), pos2(table_rect.min.x + 490.0, table_rect.min.y + header_h));
                let col_seeds_rect = Rect::from_min_max(pos2(table_rect.min.x + 490.0, table_rect.min.y), pos2(table_rect.min.x + 570.0, table_rect.min.y + header_h));
                let col_peers_rect = Rect::from_min_max(pos2(table_rect.min.x + 570.0, table_rect.min.y), pos2(table_rect.min.x + 650.0, table_rect.min.y + header_h));
                let col_date_rect = Rect::from_min_max(pos2(table_rect.min.x + 650.0, table_rect.min.y), pos2(table_rect.max.x, table_rect.min.y + header_h));

                if ui.allocate_rect(col_name_rect, egui::Sense::click()).clicked() {
                    self.toggle_sort(SortColumn::Name);
                }
                if ui.allocate_rect(col_size_rect, egui::Sense::click()).clicked() {
                    self.toggle_sort(SortColumn::Size);
                }
                if ui.allocate_rect(col_seeds_rect, egui::Sense::click()).clicked() {
                    self.toggle_sort(SortColumn::Seeds);
                }
                if ui.allocate_rect(col_peers_rect, egui::Sense::click()).clicked() {
                    self.toggle_sort(SortColumn::Peers);
                }
                if ui.allocate_rect(col_date_rect, egui::Sense::click()).clicked() {
                    self.toggle_sort(SortColumn::Date);
                }

                // Column header labels with indicators
                let name_hdr = format!("{} {}", crate::i18n::t("Name"), if self.sort_col == SortColumn::Name { arrow(self.sort_dir) } else { "" });
                let size_hdr = format!("{} {}", crate::i18n::t("Size"), if self.sort_col == SortColumn::Size { arrow(self.sort_dir) } else { "" });
                let seeds_hdr = format!("{} {}", crate::i18n::t("Seeds"), if self.sort_col == SortColumn::Seeds { arrow(self.sort_dir) } else { "" });
                let peers_hdr = format!("{} {}", crate::i18n::t("Peers"), if self.sort_col == SortColumn::Peers { arrow(self.sort_dir) } else { "" });
                let date_hdr = format!("{} {}", crate::i18n::t("Date"), if self.sort_col == SortColumn::Date { arrow(self.sort_dir) } else { "" });

                text(&p, o.x + 22.0, o.y + 88.0, &name_hdr, TEXT());
                text(&p, o.x + 420.0, o.y + 88.0, &size_hdr, TEXT());
                text(&p, o.x + 510.0, o.y + 88.0, &seeds_hdr, Color32::from_rgb(0, 180, 0));
                text(&p, o.x + 590.0, o.y + 88.0, &peers_hdr, Color32::from_rgb(60, 140, 220));
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
                        let row_rc = Rect::from_min_size(pos2(body_rect.min.x, y), vec2(body_rect.width(), row_h));
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
                        let (fit_name, _) = winui::text_fit(&p, o.x + 22.0, y + 16.0, &item.name, text_color, o.x + 400.0);
                        let _ = fit_name;
                        text(&p, o.x + 420.0, y + 16.0, &crate::model::size(item.size), text_color);
                        text(&p, o.x + 510.0, y + 16.0, &item.seeders.to_string(), Color32::from_rgb(0, 180, 0));
                        text(&p, o.x + 590.0, y + 16.0, &item.leechers.to_string(), Color32::from_rgb(60, 140, 220));
                        text(&p, o.x + 670.0, y + 16.0, &format_date(item.added), text_color);
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

                if has_sel && let Some(sel) = self.selected && let Some(item) = items.get(sel) {
                    let dn: String = url::form_urlencoded::byte_serialize(item.name.as_bytes()).collect();
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
