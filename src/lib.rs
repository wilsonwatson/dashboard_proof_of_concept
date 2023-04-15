#![warn(clippy::all, rust_2018_idioms)]

pub struct NTConnection {}

pub fn is_mobile(ctx: &egui::Context) -> bool {
    let screen_size = ctx.screen_rect().size();
    screen_size.x < 550.0
}

mod views;

struct Tab {
    name: String,
    code_open: bool,
    views: Vec<View>
}

impl Tab {
    fn new<S : Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            code_open: false,
            views: Vec::new()
        }
    }

    fn with_view(mut self, v: View) -> Self {
        self.views.push(v);
        self
    }

    fn ui(&self, ui: &mut Ui) {
        if self.views.is_empty() {
            ui.heading("There's nothing here.");
            ui.label("Click \"Layout\" in the topbar to change that.");
        } else {
            for view in &self.views {
                view.draw(ui);
            }
        }
    }
}

pub struct App {
    tabs: Vec<Tab>,
    selected: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            tabs: vec![Tab::new("Tab 1").with_view(View::Number { name: "Test Number".to_string(), settings: NumberViewSettings {} }).with_view(View::Plot {name: "Test Plot".to_string(), settings: PlotSettings {}})],
            selected: 0usize,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            #[cfg(feature = "persistence")]
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

use egui::{Layout, Color32, Pos2, Ui};
use views::{View, NumberViewSettings, PlotSettings};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

macro_rules! alert {
    ($message:literal) => {
        #[cfg(target_arch = "wasm32")]
        web_sys::window()
            .and_then(|win| {
                win.alert_with_message($message).ok()?;
                Some(())
            })
            .expect("Unable to alert!")
    };
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        #[cfg(feature = "persistence")]
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { tabs, selected } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Load From Code").clicked() {
                        alert!("TODO");
                    }
                    if ui.button("Save To Code").clicked() {
                        alert!("TODO");
                    }
                    if ui.button("Clear Layouts").clicked() {
                        alert!("TODO");
                    }

                    #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                ui.menu_button("Tabs", |ui| {
                    if ui.button("New Tab").clicked() {
                        let mut num = 0;
                        'a:
                        loop {
                            num += 1;
                            for tab in tabs.iter() {
                                if tab.name == format!("Tab {}", num) {
                                    continue 'a;
                                }
                            }
                            break;
                        }
                        tabs.push(Tab::new(format!("Tab {}", num)));
                    }
                    if ui.button("Delete Tab").clicked() {
                        alert!("TODO");
                    }
                });
                ui.menu_button("Layout", |ui| {
                    if ui.button("Load From Code").clicked() {
                        alert!("TODO");
                    }
                    if ui.button("Save To Code").clicked() {
                        alert!("TODO");
                    }
                    if ui.button("Clear").clicked() {
                        alert!("TODO");
                    }
                    if ui.button("Edit Layout Code").clicked() {
                        tabs[*selected].code_open = true;
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").exact_width(80.0).show(ctx, |ui| {
            ui.with_layout(
                Layout::from_main_dir_and_cross_align(
                    egui::Direction::TopDown,
                    egui::Align::Center,
                )
                .with_cross_justify(true),
                |ui| {
                    for (i, tab) in tabs.iter().enumerate() {
                        ui.selectable_value(selected, i, &tab.name);
                    }
                },
            );
            ui.with_layout(Layout::from_main_dir_and_cross_align(egui::Direction::BottomUp, egui::Align::Center), |ui| {
                ui.hyperlink_to("(source)", "https://www.github.com/wilsonwatson/dashboard_proof_of_concept")
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            tabs[*selected].ui(ui);
        });
    }
}

/*
 * WebAssembly Stuff
 */

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn run() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "main", // hardcode it
        web_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
    .await
    .expect("failed to start eframe");
}
