use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, DrawingArea, GestureClick, Orientation, glib};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
mod algebra;
use algebra::*;
const SYSTEM_SIZE: usize = 4;
const BOX_SIZE: f64 = 50.0;
#[rustfmt::skip]
static mut SYSTEM: System = System::new([
    Equation::new([1.0, 0.0, 0.0, 0.0], 1.0),
    Equation::new([0.0, 1.0, 0.0, 0.0], 2.0),
    Equation::new([0.0, 0.0, 2.0, 0.0], 6.0),
    Equation::new([0.0, 0.0, 0.0, 1.0], 4.0),
]);
#[derive(Clone, Copy, Debug, PartialEq)]
enum CanvasItem {
    Circle(usize),
    Coefficient(usize, usize),
    Solution(usize),
}
impl CanvasItem {
    fn from_coordinates(x: f64, y: f64) -> Self {
        let equation = (y / BOX_SIZE) as usize; //rounds down
        if x < BOX_SIZE {
            Self::Circle(equation)
        } else if x < (SYSTEM_SIZE + 1) as f64 * BOX_SIZE {
            Self::Coefficient(equation, (x / BOX_SIZE) as usize - 1)
        } else {
            Self::Solution(equation)
        }
    }
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
fn draw_text_centered(context: &gtk4::cairo::Context, x: f64, y: f64, text: &str) {
    let extents = context.text_extents(text).unwrap();
    context.move_to(x - extents.width() / 2.0, y + extents.height() / 2.0);
    context.show_text(text).unwrap();
}
fn main() -> glib::ExitCode {
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
        context.set_font_size(18.0);
        for i in 0..SYSTEM_SIZE {
            for j in 0..SYSTEM_SIZE {
                let (x, y) = CanvasItem::Coefficient(i, j).get_center();
                draw_text_centered(
                    context,
                    x,
                    y,
                    &format!("{}", unsafe { SYSTEM.equations[i].coefficients[j] }),
                );
            }
        }
        for i in 0..SYSTEM_SIZE {
            let (x, y) = CanvasItem::Solution(i).get_center();
            draw_text_centered(
                context,
                x,
                y,
                &format!("{}", unsafe { SYSTEM.equations[i].solution }),
            );
        }
    });
    let left_click = GestureClick::new();
    left_click.set_button(1);
    let my_drawing_area = drawing_area.clone();
    left_click.connect_pressed(move |_, _, x, y| {
        let canvas_item = CanvasItem::from_coordinates(x, y);
        println!("{} {} {:?}", x, y, canvas_item);
        if let CanvasItem::Coefficient(equation, coefficient) = canvas_item {
            unsafe { SYSTEM }.make_coefficient_1(equation, coefficient);
        }
        my_drawing_area.queue_draw();
    });
    drawing_area.add_controller(left_click);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&main_box)
        .build();
    window.present();
}
