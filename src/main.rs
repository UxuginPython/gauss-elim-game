// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2025 UxuginPython
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Button, DrawingArea, GestureClick, GestureDrag, Label,
    Notebook, Orientation, glib,
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
#[allow(dead_code)]
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
    //Format -0.0 as 0
    if float == 0.0 {
        return "0".into();
    }
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
        .application_id("com.uxugin.gauss_elim_game")
        .build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    /*#[rustfmt::skip]
    let system = Rc::new(RefCell::new(System::new([
        Equation::new([1.0, 0.0, 0.0, 0.0], 1.0),
        Equation::new([0.0, 1.0, 0.0, 0.0], 2.0),
        Equation::new([0.0, 0.0, 2.0, 3.0], 6.0),
        Equation::new([0.0, 0.0, 0.0, 1.0], 4.0),
    ])));*/
    let system = Rc::new(RefCell::new(System::random()));
    let selected_row: Rc<Cell<Option<usize>>> = Rc::new(Cell::new(None));
    let hint: Rc<Cell<Option<(usize, usize, usize)>>> = Rc::new(Cell::new(None));
    let main_box = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    let drawing_area = DrawingArea::builder()
        .width_request(BOX_SIZE as i32 * (SYSTEM_SIZE + 2) as i32)
        .height_request(BOX_SIZE as i32 * SYSTEM_SIZE as i32)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    let button_box = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();
    let new_button = Button::builder().label("New").build();
    let my_system = Rc::clone(&system);
    let my_drawing_area = drawing_area.clone();
    let my_selected_row = Rc::clone(&selected_row);
    let my_hint = Rc::clone(&hint);
    new_button.connect_clicked(move |_| {
        my_hint.set(None);
        *my_system.borrow_mut() = System::random();
        my_drawing_area.queue_draw();
    });
    let hint_button = Button::builder().label("Hint").build();
    let my_hint = Rc::clone(&hint);
    let my_system = Rc::clone(&system);
    let my_drawing_area = drawing_area.clone();
    hint_button.connect_clicked(move |_| {
        my_hint.set(my_system.borrow().hint());
        my_drawing_area.queue_draw();
    });
    let help_button = Button::builder().label("Help").build();
    help_button.connect_clicked(move |_| {
        let notebook = Notebook::new();
        let about = Label::builder()
            .use_markup(true)
            .label(
                "<big>Gaussian Elimination Game</big>\nGaussian elimination puzzle game using GTK4\n\n<small>BSD 3-Clause \"New\" or \"Revised\" License\nCopyright 2025 UxuginPython\nhttps://github.com/UxuginPython/gauss-elim-game</small>",
            )
            .margin_top(20)
            .margin_bottom(20)
            .margin_start(20)
            .margin_end(20)
            .build();
        let about_tab_label = Label::builder().label("About").build();
        let about_gauss = Label::builder()
            .wrap(true)
            .label("Gaussian elimination is a method of solving linear systems of equations named after mathematician Carl Friedrich Gauss. It arranges the coefficients and solutions of the equations into a matrix and then allows three operations: swapping two rows, scaling a row, and adding a multiple of a row to another. These operations are performed until the coefficients form the identity matrix (called reduced row echelon form) if a unique solution exists.")
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();
        let about_gauss_tab_label = Label::builder().label("About Gaussian Elimination").build();
        let how_to_play = Label::builder()
            .wrap(true)
            .label("To swap two rows, drag from the circle to the left of one to the circle of the other.\nTo scale a row to make a coefficient 1, click the coefficient.\nTo add a multiple of a row to another row to make a coefficient 0, drag from the row's circle to the coefficient.\nClick \"Hint\" for a suggestion for what to do.\nClick \"New\" to generate a new random system.")
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();
        let how_to_play_tab_label = Label::builder().label("How to Play").build();
        notebook.append_page(&about, Some(&about_tab_label));
        notebook.append_page(&about_gauss, Some(&about_gauss_tab_label));
        notebook.append_page(&how_to_play, Some(&how_to_play_tab_label));
        let help_window = gtk4::Window::builder()
            .title("Help")
            .child(&notebook)
            .build();
        help_window.set_default_width(200);
        help_window.present();
    });
    button_box.append(&new_button);
    button_box.append(&hint_button);
    button_box.append(&help_button);
    main_box.append(&button_box);
    main_box.append(&drawing_area);
    let my_system = Rc::clone(&system);
    let my_hint = Rc::clone(&hint);
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
        context.move_to(0.75 * BOX_SIZE, 0.5 * BOX_SIZE);
        for i in 0..SYSTEM_SIZE {
            context.arc(
                0.5 * BOX_SIZE,
                i as f64 * BOX_SIZE + 0.5 * BOX_SIZE,
                0.25 * BOX_SIZE,
                0.0,
                7.0,
            );
            context.stroke().unwrap();
        }
        match my_selected_row.get() {
            Some(i) => {
                context.set_source_rgb(0.0, 0.5, 1.0);
                context.arc(
                    0.5 * BOX_SIZE,
                    i as f64 * BOX_SIZE + 0.5 * BOX_SIZE,
                    0.25 * BOX_SIZE,
                    0.0,
                    7.0,
                );
                context.fill().unwrap();
            }
            None => {}
        }
        if let Some((equation, coefficient, with)) = my_hint.get() {
            let (start_x, start_y) = CanvasItem::Circle(with).get_center();
            let (end_x, end_y) = CanvasItem::Coefficient(equation, coefficient).get_center();
            context.set_source_rgb(0.0, 0.0, 1.0);
            context.line_to(start_x, start_y);
            context.line_to(end_x, end_y);
            context.stroke().unwrap();
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
    let my_selected_row = Rc::clone(&selected_row);
    drag.connect_drag_begin(move |_, x, y| {
        my_start_coords.set((x, y));
        if let CanvasItem::Circle(i) = CanvasItem::from_coordinates(x, y) {
            my_selected_row.set(Some(i));
        }
    });
    let my_start_coords = Rc::clone(&start_coords);
    let my_selected_row = Rc::clone(&selected_row);
    let my_hint = Rc::clone(&hint);
    drag.connect_drag_end(move |_, relative_x, relative_y| {
        my_selected_row.set(None);
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
                my_hint.set(None);
            } else if let CanvasItem::Coefficient(end_equation, end_coefficient) = end_item {
                if start_equation == end_equation {
                    return;
                }
                if system
                    .borrow()
                    .can_make_coefficient_0_with_row(end_coefficient, start_equation)
                {
                    system.borrow_mut().make_coefficient_0_with_row(
                        end_equation,
                        end_coefficient,
                        start_equation,
                    );
                    my_hint.set(None);
                }
            }
        }
        my_drawing_area.queue_draw();
    });
    drawing_area.add_controller(drag);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Gaussian Elimination Game")
        .child(&main_box)
        .build();
    window.present();
}
