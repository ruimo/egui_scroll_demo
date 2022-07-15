use eframe::{egui::{self, Event, ScrollArea}, emath::Vec2};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    scroll: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            scroll: 0.
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if ui.button("Scroll").clicked() {
                self.scroll = 10.;
            }
        });
        egui::CentralPanel::default()
        .show(ctx, |ui| {
            ScrollArea::both()
            .show_viewport(ui, |ui, viewport| {
                ui.set_width(2000.);
                ui.set_height(2000.);
                println!("Scroll: {}", self.scroll);
                ui.scroll_with_delta(Vec2::new(self.scroll, 0.));
                self.scroll = 0.;
            });
        });
    }
}
