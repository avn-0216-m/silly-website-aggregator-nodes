use egui::{Ui, Response, Context, Id};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Swan", native_options, Box::new(|cc| Box::new(SwanApp::new(cc))));
}

struct SwanApp {
    counter: u64,
}

impl Default for SwanApp {
    fn default() -> SwanApp {
        SwanApp{ counter: 0 }
    }
}


impl SwanApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        return SwanApp::default();
    }
}

enum TestResp {
    None,
    Clicked
}

impl eframe::App for SwanApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

       egui::Window::new("Hello world").show(ctx, |ui| {
            let resp = ui.add(TestWidget{ counter: self.counter });
            if resp.clicked(){
                println!("Cliiicked!");
                self.counter += 1;
            }
       });
   }
}

struct TestWidget {
    counter: u64,
}

impl Default for TestWidget {
    fn default() -> TestWidget {
        return TestWidget{ counter: 0};
    }
}

impl TestWidget {
    fn print_counter(&self) {
        println!("{}", self.counter);
    }
}

impl egui::Widget for TestWidget {

    fn ui(self, ui: &mut Ui) -> Response {

        let size = egui::Vec2{ x: 100.0, y: 100.0 };

        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());

        if response.clicked() {
            self.print_counter();
        } else if response.hovered() {
            //println!("You're hovering over meee!");
        } else if response.dragged() {
            println!("I can't move! :O");
        }

        ui.label(self.counter.to_string());

        return response;
    }
}