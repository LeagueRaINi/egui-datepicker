use chrono::{Datelike, Duration};
use eframe::egui::{self, Color32};
use egui_datepicker::*;

struct ExampleApp {
    date: Date<Utc>,
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            date: Utc::now().date(),
        }
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ctx.set_debug_on_hover(true);
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("exaamples_grid").show(ui, |ui| {
                ui.label("Default");
                ui.add(DatePicker::new("default", &mut self.date));
                ui.end_row();
                ui.label("Sunday first");
                ui.add(DatePicker::new("sundayfirst", &mut self.date).sunday_first(true));
                ui.end_row();
                ui.label("Movable popup");
                ui.add(DatePicker::new("movable", &mut self.date).movable(true));
                ui.end_row();
                ui.label("Different format");
                ui.add(DatePicker::new("differentformat", &mut self.date).date_format(&"%d/%m/%Y"));
                ui.end_row();
                ui.label("Disable weekend highlight");
                ui.add(
                    DatePicker::new("noweekendhighlight", &mut self.date).highlight_weekend(false),
                );
                ui.end_row();
                ui.label("Different weekend color");
                ui.add(
                    DatePicker::new("differentweekendcolor", &mut self.date)
                        .highlight_weekend_color(Color32::from_rgb(0, 196, 0)),
                );
                ui.end_row();
                ui.label("Different weekend days, i.e. holidays, Christmas, etc");
                ui.add(
                    DatePicker::new("differentweekenddays", &mut self.date)
                        .weekend_days(|date| date.day() % 2 == 0),
                );
                ui.end_row();
                ui.label("Minimum date (Today -10 days)");
                ui.add(
                    DatePicker::new("minimumdate", &mut self.date)
                        .min_date(Utc::today() - Duration::days(10)),
                );
                ui.end_row();
                ui.label("Maximum date (Today +10 days)");
                ui.add(
                    DatePicker::new("maximumdate", &mut self.date)
                        .max_date(Utc::today() + Duration::days(10)),
                );
                ui.end_row();
            });
        });
    }
}

fn main() {
    let app = ExampleApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Datepicker example",
        native_options,
        Box::new(|_cc| Box::new(app)),
    );
}
