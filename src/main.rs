use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, DrawingArea, Orientation, glib};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
mod algebra;
use algebra::*;
const SYSTEM_SIZE: usize = 4;
const BOX_SIZE: f64 = 50.0;
#[rustfmt::skip]
static mut SYSTEM: System = System::new([
    Equation::new([1.0, 0.0, 0.0, 0.0], 1.0),
    Equation::new([0.0, 1.0, 0.0, 0.0], 2.0),
    Equation::new([0.0, 0.0, 1.0, 0.0], 3.0),
    Equation::new([0.0, 0.0, 0.0, 1.0], 4.0),
]);
enum CanvasItem {
    Circle(usize),
    Coefficient(usize, usize),
    Solution(usize),
}
impl CanvasItem {
    fn get_center(&self) -> (f64, f64) {
        match *self {
            Self::Circle(equation) => (BOX_SIZE / 2.0, BOX_SIZE * equation as f64 + BOX_SIZE / 2.0),
            Self::Coefficient(equation, coefficient) => (
                BOX_SIZE * coefficient as f64 + BOX_SIZE * 1.5,
                BOX_SIZE * equation as f64 + BOX_SIZE / 2.0,
            ),
            Self::Solution(equation) => (
                SYSTEM_SIZE as f64 * BOX_SIZE + BOX_SIZE * 1.5,
                BOX_SIZE * equation as f64 + BOX_SIZE / 2.0,
            ),
        }
    }
}
fn draw_x(context: &gtk4::cairo::Context, x: f64, y: f64) {
    context.line_to(x - 5.0, y - 5.0);
    context.line_to(x + 5.0, y + 5.0);
    context.stroke().unwrap();
    context.line_to(x + 5.0, y - 5.0);
    context.line_to(x - 5.0, y + 5.0);
    context.stroke().unwrap();
}
fn plot_centers(context: &gtk4::cairo::Context) {
    context.set_source_rgb(1.0, 0.0, 0.0);
    for i in 0..SYSTEM_SIZE {
        let (x, y) = CanvasItem::Circle(i).get_center();
        draw_x(context, x, y);
    }
    context.set_source_rgb(0.0, 0.5, 0.0);
    for i in 0..SYSTEM_SIZE {
        for j in 0..SYSTEM_SIZE {
            let (x, y) = CanvasItem::Coefficient(i, j).get_center();
            draw_x(context, x, y);
        }
    }
    context.set_source_rgb(0.0, 0.0, 1.0);
    for i in 0..SYSTEM_SIZE {
        let (x, y) = CanvasItem::Solution(i).get_center();
        draw_x(context, x, y);
    }
}
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
        .width_request(BOX_SIZE as i32 * (SYSTEM_SIZE + 2) as i32)
        .height_request(BOX_SIZE as i32 * SYSTEM_SIZE as i32)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    main_box.append(&drawing_area);
    drawing_area.set_draw_func(|_drawing_area, context, _width, _height| {
        context.line_to(BOX_SIZE * 1.5, 0.0);
        context.line_to(BOX_SIZE, 0.0);
        context.line_to(BOX_SIZE, BOX_SIZE * SYSTEM_SIZE as f64);
        context.line_to(BOX_SIZE * 1.5, BOX_SIZE * SYSTEM_SIZE as f64);
        context.stroke().unwrap();
        context.line_to((SYSTEM_SIZE + 2) as f64 * BOX_SIZE - 0.5 * BOX_SIZE, 0.0);
        context.line_to((SYSTEM_SIZE + 2) as f64 * BOX_SIZE, 0.0);
        context.line_to(
            (SYSTEM_SIZE + 2) as f64 * BOX_SIZE,
            BOX_SIZE * SYSTEM_SIZE as f64,
        );
        context.line_to(
            (SYSTEM_SIZE + 2) as f64 * BOX_SIZE - 0.5 * BOX_SIZE,
            BOX_SIZE * SYSTEM_SIZE as f64,
        );
        context.stroke().unwrap();
        context.line_to((SYSTEM_SIZE + 1) as f64 * BOX_SIZE, 0.0);
        context.line_to(
            (SYSTEM_SIZE + 1) as f64 * BOX_SIZE,
            BOX_SIZE * SYSTEM_SIZE as f64,
        );
        context.stroke().unwrap();
        plot_centers(context);
    });
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&main_box)
        .build();
    window.present();
}
