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
    pub category: String,
}

#[derive(Clone, Debug)]
pub struct TorrentSearchResult {
    pub name: String,
    pub info_hash: String,
    pub size: u64,
    pub seeders: u32,
    pub leechers: u32,
}

pub struct TorrentSearch {
    pub query: String,
    pub searching: bool,
    pub status_text: String,
    pub results: Arc<Mutex<Vec<TorrentSearchResult>>>,
    pub selected: Option<usize>,
    pub focused: bool,
    pub scroll_offset: f32,
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
        }
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
                        });
                    }
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
        let outcome = window(
            ctx,
            "torrent-search",
            "Torrent Search",
            [680.0, 480.0],
            false,
            BG(),
            |ui, o| {
                let p = ui.painter().clone();
                let can_search = !self.query.trim().is_empty() && !self.searching;

                // Search bar input
                let edit = winui::edit(
                    ui,
                    r(o, 14.0, 14.0, 530.0, 24.0),
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
                    r(o, 554.0, 14.0, 110.0, 24.0),
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

                text(&p, o.x + 14.0, o.y + 54.0, &self.status_text, MUTED());

                // Table outer container
                let table_rect = r(o, 14.0, 64.0, 650.0, 360.0);
                winui::solid(&p, table_rect, winui::FIELD_EDGE());
                let header_h = 24.0;
                let head_rect = Rect::from_min_size(table_rect.min, vec2(table_rect.width(), header_h));
                p.rect_filled(head_rect, 0.0, winui::LIST_HEADER());

                // Column headers
                text(&p, o.x + 22.0, o.y + 80.0, crate::i18n::t("Name"), TEXT());
                text(&p, o.x + 440.0, o.y + 80.0, crate::i18n::t("Size"), TEXT());
                text(&p, o.x + 530.0, o.y + 80.0, crate::i18n::t("Seeds"), Color32::from_rgb(0, 180, 0));
                text(&p, o.x + 600.0, o.y + 80.0, crate::i18n::t("Peers"), Color32::from_rgb(60, 140, 220));

                // Scrollable rows
                let row_h = 20.0;
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
                        let (fit_name, _) = winui::text_fit(&p, o.x + 22.0, y + 15.0, &item.name, text_color, o.x + 420.0);
                        let _ = fit_name;
                        text(&p, o.x + 440.0, y + 15.0, &crate::model::size(item.size), text_color);
                        text(&p, o.x + 530.0, y + 15.0, &item.seeders.to_string(), Color32::from_rgb(0, 180, 0));
                        text(&p, o.x + 600.0, y + 15.0, &item.leechers.to_string(), Color32::from_rgb(60, 140, 220));
                    }
                }

                // Bottom Action buttons
                let has_sel = self.selected.is_some_and(|idx| idx < items.len());
                let download = winui::button(
                    ui,
                    r(o, 440.0, 436.0, 110.0, 24.0),
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

                let copy_magnet = winui::button(
                    ui,
                    r(o, 310.0, 436.0, 120.0, 24.0),
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

                let close = winui::button(
                    ui,
                    r(o, 560.0, 436.0, 104.0, 24.0),
                    crate::i18n::mark("Close"),
                    ButtonKind::Normal,
                    false,
                    None,
                )
                .clicked();

                if winui::escape_pressed(ui) || close {
                    Outcome::Closed
                } else if let Some(mag) = download_magnet.take() {
                    Outcome::Done(mag)
                } else {
                    Outcome::Open
                }
            },
        );

        outcome
    }
}
