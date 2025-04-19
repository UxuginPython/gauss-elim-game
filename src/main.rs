use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, DrawingArea, Orientation, glib};
use std::mem::MaybeUninit;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
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
    ///Checks if the coefficient can be made 1 without doing it.
    const fn can_make_coefficient_1(&self, index: usize) -> bool {
        self.coefficients[index] != 0.0
    }
    fn make_coefficient_1(&mut self, index: usize) {
        let dividend = self.coefficients[index];
        for i in 0..SYSTEM_SIZE {
            self.coefficients[i] /= dividend;
        }
        debug_assert_eq!(self.coefficients[index], 1.0);
    }
}
impl Neg for Equation {
    type Output = Self;
    fn neg(self) -> Self {
        let mut new_coefficients = [0.0; SYSTEM_SIZE];
        for i in 0..SYSTEM_SIZE {
            new_coefficients[i] = -new_coefficients[i];
        }
        Self::new(new_coefficients, -self.solution)
    }
}
impl Add for Equation {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut new_coefficients = self.coefficients;
        for i in 0..SYSTEM_SIZE {
            new_coefficients[i] += rhs.coefficients[i];
        }
        Self::new(new_coefficients, self.solution + rhs.solution)
    }
}
impl AddAssign for Equation {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Sub for Equation {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}
impl SubAssign for Equation {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl Mul<f64> for Equation {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        let mut new_coefficients = self.coefficients;
        for i in 0..SYSTEM_SIZE {
            new_coefficients[i] *= rhs;
        }
        Self::new(new_coefficients, self.solution * rhs)
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
    ///Checks if the coefficient can be made 1 without doing it.
    const fn can_make_coefficient_1(&self, equation: usize, coefficient: usize) -> bool {
        self.equations[equation].can_make_coefficient_1(coefficient)
    }
    fn make_coefficient_1(&mut self, equation: usize, coefficient: usize) {
        self.equations[equation].make_coefficient_1(coefficient);
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
