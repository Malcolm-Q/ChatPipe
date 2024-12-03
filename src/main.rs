use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{self, BufRead};

fn main() -> eframe::Result<()> {
    let messages = Arc::new(Mutex::new(Vec::new()));

    {
        let messages = Arc::clone(&messages);
        thread::spawn(move || {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                if let Ok(line) = line {
                    let mut messages = messages.lock().unwrap();
                    messages.push(line);
                }
            }
        });
    }

    eframe::run_native(
        "STRAFTAT Chat Proxy",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(MyApp { messages })),
    )
}

struct MyApp {
    messages: Arc<Mutex<Vec<String>>>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (egui::TextStyle::Body, egui::FontId::new(20.0, egui::FontFamily::Proportional)),
        ].into();
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            let messages = self.messages.lock().unwrap();
            let combined_text = messages.join("\n");

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.add(egui::Label::new(combined_text).wrap(true));
            });
        });

        ctx.request_repaint();
    }
}
