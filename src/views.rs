use egui::{Ui, plot::{Plot, Legend, Line, PlotPoints}, Color32};

pub struct PlotSettings {
    // TODO
}

pub struct NumberViewSettings {
    // TODO
}

pub struct ConditionalSettings {
    // TODO
}

pub struct StringSettings {
    // TODO
}

pub struct SelectableChooserSettings {
    // TODO
}

pub struct Field2dSettings {
    // TODO
}

pub struct CameraStreamSettings {
    // TODO
}

pub struct RVizSettings {
    // TODO
}

// TODO serde using https://serde.rs/enum-representations.html#internally-tagged
pub enum View {
    Plot{name: String, settings: PlotSettings},
    Number{name: String, settings: NumberViewSettings},
    Conditional{true_case: Vec<View>, false_case: Vec<View>, settings: ConditionalSettings},
    String{name: String, settings: StringSettings},
    SelectableChooser{name: String, settings: SelectableChooserSettings},
    Field2dSettings{settings: Field2dSettings},
    CameraStream{settings: CameraStreamSettings},
    RViz{settings: RVizSettings},
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
                    // TODO use actual values
                    let plot = Plot::new("demo_plot").legend(Legend::default());
                    plot.width(256.0).height(256.0).show(ui, |plot_ui| {
                        plot_ui.line(
                            Line::new(PlotPoints::from_explicit_callback(|x| x.sin(), .., 512)).color(Color32::from_rgb(200, 100, 100)).name(name)
                        )
                    });
                });
            },
            View::Number { name, settings } => {
                // TODO use actual values
                let mut val = format!("{}", 20);
                ui.horizontal(|ui| {
                    ui.label(name);
                    if ui.text_edit_singleline(&mut val).changed() {

                    }
                });
            },
            View::Conditional { settings, true_case, false_case } => {
                ui.horizontal(|ui| {
                    // TODO use actual values
                    if true {
                        true_case.iter().for_each(|f| f.draw(ui));
                    } else {
                        false_case.iter().for_each(|f| f.draw(ui));
                    }
                });
            },
            _ => todo!()
        }
    }
}
