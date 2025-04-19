use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, DrawingArea, GestureClick, GestureDrag, Orientation, glib,
};
use std::cell::{Cell, RefCell};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::rc::Rc;
mod algebra;
use algebra::*;
const SYSTEM_SIZE: usize = 4;
const BOX_SIZE: f64 = 50.0;
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
fn format_float(float: f64) -> String {
    let normal = format!("{}", float);
    let rounded = format!("{:.3}", float);
    if normal.len() <= rounded.len() {
        normal
    } else {
        rounded
    }
}
fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.uxugin.matrixfun")
        .build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    #[rustfmt::skip]
    let system = Rc::new(RefCell::new(System::new([
        Equation::new([1.0, 0.0, 0.0, 0.0], 1.0),
        Equation::new([0.0, 1.0, 0.0, 0.0], 2.0),
        Equation::new([0.0, 0.0, 2.0, 3.0], 6.0),
        Equation::new([0.0, 0.0, 0.0, 1.0], 4.0),
    ])));
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
    let my_system = Rc::clone(&system);
    drawing_area.set_draw_func(move |_drawing_area, context, _width, _height| {
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
                    &format_float(my_system.borrow().equations[i].coefficients[j]),
                );
            }
        }
        for i in 0..SYSTEM_SIZE {
            let (x, y) = CanvasItem::Solution(i).get_center();
            draw_text_centered(
                context,
                x,
                y,
                &format_float(my_system.borrow().equations[i].solution),
            );
        }
    });
    let left_click = GestureClick::new();
    left_click.set_button(1);
    let my_drawing_area = drawing_area.clone();
    let my_system = Rc::clone(&system);
    left_click.connect_pressed(move |_, _, x, y| {
        let canvas_item = CanvasItem::from_coordinates(x, y);
        if let CanvasItem::Coefficient(equation, coefficient) = canvas_item {
            if my_system
                .borrow()
                .can_make_coefficient_1(equation, coefficient)
            {
                my_system
                    .borrow_mut()
                    .make_coefficient_1(equation, coefficient);
            }
        }
        my_drawing_area.queue_draw();
    });
    drawing_area.add_controller(left_click);
    let drag = GestureDrag::new();
    let start_coords = Rc::new(Cell::new((0.0, 0.0)));
    let my_start_coords = Rc::clone(&start_coords);
    let my_drawing_area = drawing_area.clone();
    drag.connect_drag_begin(move |_, x, y| {
        my_start_coords.set((x, y));
    });
    let my_start_coords = Rc::clone(&start_coords);
    drag.connect_drag_end(move |_, relative_x, relative_y| {
        let (start_x, start_y) = my_start_coords.get();
        let end_x = start_x + relative_x;
        let end_y = start_y + relative_y;
        let start_item = CanvasItem::from_coordinates(start_x, start_y);
        let end_item = CanvasItem::from_coordinates(end_x, end_y);
        if let CanvasItem::Circle(start_equation) = start_item {
            if let CanvasItem::Circle(end_equation) = end_item {
                system
                    .borrow_mut()
                    .switch_rows(start_equation, end_equation);
                my_drawing_area.queue_draw();
            } else if let CanvasItem::Coefficient(end_equation, end_coefficient) = end_item {
                if system
                    .borrow()
                    .can_make_coefficient_0_with_row(end_coefficient, start_equation)
                {
                    system.borrow_mut().make_coefficient_0_with_row(
                        end_equation,
                        end_coefficient,
                        start_equation,
                    );
                    my_drawing_area.queue_draw();
                }
            }
        }
    });
    drawing_area.add_controller(drag);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&main_box)
        .build();
    window.present();
}
