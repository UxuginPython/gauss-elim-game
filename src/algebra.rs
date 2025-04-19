use super::*;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Equation {
    pub coefficients: [f64; SYSTEM_SIZE],
    pub solution: f64,
}
impl Equation {
    pub const fn new(coefficients: [f64; SYSTEM_SIZE], solution: f64) -> Self {
        Self {
            coefficients: coefficients,
            solution: solution,
        }
    }
    ///Checks if the coefficient can be made 1 without doing it.
    pub const fn can_make_coefficient_1(&self, index: usize) -> bool {
        self.coefficients[index] != 0.0
    }
    pub fn make_coefficient_1(&mut self, index: usize) {
        let dividend = self.coefficients[index];
        *self /= dividend;
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
macro_rules! impl_assign {
    ($trait_name: ident, $func_name: ident, $rhs: ty, $symbol: tt) => {
        impl $trait_name<$rhs> for Equation {
            fn $func_name(&mut self, rhs: $rhs) {
                *self = *self $symbol rhs;
            }
        }
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
impl_assign!(AddAssign, add_assign, Self, +);
impl Sub for Equation {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}
impl_assign!(SubAssign, sub_assign, Self, -);
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
impl_assign!(MulAssign, mul_assign, f64, *);
impl Div<f64> for Equation {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        let mut new_coefficients = self.coefficients;
        for i in 0..SYSTEM_SIZE {
            new_coefficients[i] /= rhs;
        }
        Self::new(new_coefficients, self.solution / rhs)
    }
}
impl_assign!(DivAssign, div_assign, f64, /);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct System {
    pub equations: [Equation; SYSTEM_SIZE],
}
impl System {
    pub const fn new(equations: [Equation; SYSTEM_SIZE]) -> Self {
        Self {
            equations: equations,
        }
    }
    pub const fn switch_rows(&mut self, a: usize, b: usize) {
        let row_a = self.equations[a];
        let row_b = self.equations[b];
        self.equations[b] = row_a;
        self.equations[a] = row_b;
    }
    ///Checks if the coefficient can be made 1 without doing it.
    pub const fn can_make_coefficient_1(&self, equation: usize, coefficient: usize) -> bool {
        self.equations[equation].can_make_coefficient_1(coefficient)
    }
    pub fn make_coefficient_1(&mut self, equation: usize, coefficient: usize) {
        self.equations[equation].make_coefficient_1(coefficient);
    }
}
