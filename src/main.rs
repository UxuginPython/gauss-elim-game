use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, DrawingArea, Orientation, glib};
const SYSTEM_SIZE: usize = 4;
#[derive(Clone, Copy, Debug, PartialEq)]
struct Equation {
    coefficients: [f64; SYSTEM_SIZE],
    solution: f64,
}
impl Equation {
    const fn new(coefficients: [f64; SYSTEM_SIZE], solution: f64) -> Self {
        Self {
            coefficients: coefficients,
            solution: solution,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct System {
    equations: [Equation; SYSTEM_SIZE],
}
impl System {
    const fn new(equations: [Equation; SYSTEM_SIZE]) -> Self {
        Self {
            equations: equations,
        }
    }
    const fn switch_rows(&mut self, a: usize, b: usize) {
        let row_a = self.equations[a];
        let row_b = self.equations[b];
        self.equations[b] = row_a;
        self.equations[a] = row_b;
    }
}
#[rustfmt::skip]
static mut SYSTEM: System = System::new([
    Equation::new([1.0, 0.0, 0.0, 0.0], 1.0),
    Equation::new([0.0, 1.0, 0.0, 0.0], 2.0),
    Equation::new([0.0, 0.0, 1.0, 0.0], 3.0),
    Equation::new([0.0, 0.0, 0.0, 1.0], 4.0),
]);
fn main() -> glib::ExitCode {
    println!("{:#?}", unsafe { SYSTEM });
    let app = Application::builder()
        .application_id("com.uxugin.matrixfun")
        .build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let main_box = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();
    let drawing_area = DrawingArea::builder()
        .width_request(500)
        .height_request(500)
        .build();
    main_box.append(&drawing_area);
    drawing_area.set_draw_func(|_drawing_area, context, _width, _height| {
        context.line_to(100.0, 100.0);
        context.line_to(300.0, 200.0);
        context.stroke().unwrap();
    });
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&main_box)
        .build();
    window.present();
}
