use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Swan", native_options, Box::new(|cc| Box::new(SwanApp::new(cc))));
}

#[derive(Default)]
struct SwanApp {
    testVar: i64
}

impl SwanApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        return SwanApp{ testVar: 216 };
    }
}

impl eframe::App for SwanApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
            for n in 1..10 {
                let test_widget = TestWidget::default();
                ui.heading(n.to_string());
                let w_response: egui::Response = ui.add(test_widget);
                println!("{}", w_response.hovered);
            }

       });
   }
}

struct TestWidget {
    counter: u64
}

impl Default for TestWidget {
    fn default() -> TestWidget {
        TestWidget{ counter: 0}
    }
}

impl egui::Widget for TestWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        return ui.heading("beepy")
    }
}