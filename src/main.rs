use egui::{Ui, Response};

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
                ui.add(test_widget);
            }

       });
   }
}

struct TestWidget {
    counter: u64
}

impl Default for TestWidget {
    fn default() -> TestWidget {
        TestWidget{ counter: 0 }
    }
}

impl egui::Widget for TestWidget {
    fn ui(self, ui: &mut Ui) -> Response {

        let size = egui::Vec2{ x: 100.0, y: 100.0 };

        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());

        if response.clicked() {
            println!("Beep! You clicked me.");
        } else if response.hovered() {
            println!("You're hovering over meee!");
        } else if response.dragged() {
            println!("I can't move! :O");
        }

        return ui.heading("beepy")
    }
}