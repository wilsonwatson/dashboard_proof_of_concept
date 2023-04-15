use std::cell::RefCell;

use egui::{Ui, plot::{Plot, Legend, PlotPoint, Line, PlotPoints}, Color32};


pub struct PlotSettings {
    
}

pub struct NumberViewSettings {
    
}

pub enum View {
    Plot{name: String, settings: PlotSettings},
    Number{name: String, settings: NumberViewSettings},
    BoolView{name: String, true_case: Box<View>, false_case: Box<View>},
    /*
     * TODO: 
     * - Selectable Chooser
     * - Field2d
     * - Camera Stream
     */
}

impl View {
    pub fn draw(&self, ui: &mut Ui) {
        match self {
            View::Plot { name, settings } => {
                ui.vertical(|ui| {
                    ui.label(name);
                    let mut plot = Plot::new("demo_plot").legend(Legend::default());
                    plot.width(256.0).height(256.0).show(ui, |plot_ui| {
                        plot_ui.line(
                            Line::new(PlotPoints::from_explicit_callback(|x| x.sin(), .., 512)).color(Color32::from_rgb(200, 100, 100)).name(name)
                        )
                    });
                });
            },
            View::Number { name, settings } => {
                let mut val = format!("{}", 20);
                ui.horizontal(|ui| {
                    ui.label(name);
                    if ui.text_edit_singleline(&mut val).changed() {

                    }
                });
            },
            View::BoolView { name, true_case, false_case } => {
                ui.horizontal(|ui| {
                    ui.label(name);
                    if true {
                        true_case.draw(ui);
                    } else {
                        false_case.draw(ui);
                    }
                });
            },
        }
    }
}
