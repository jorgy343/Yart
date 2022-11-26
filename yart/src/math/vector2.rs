use super::vector::Vector;
use crate::common::*;
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use impl_ops::*;
use std::ops::{self, Index, IndexMut};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: Real,
    pub y: Real,
}

impl Vector2 {
    pub fn new(x: Real, y: Real) -> Self {
        Self { x, y }
    }
}

impl Vector for Vector2 {
    fn from_value(value: Real) -> Self {
        Self { x: value, y: value }
    }

    fn abs(value: &Self) -> Self {
        Self::new(Real::abs(value.x), Real::abs(value.y))
    }

    fn abs_mut(&mut self) -> &Self {
        let temp = Self::abs(self);
        *self = temp;

        self
    }

    fn component_mul(left: &Self, right: &Self) -> Self {
        Self::new(left.x * right.x, left.y * right.y)
    }

    fn distance(left: &Self, right: &Self) -> Real {
        Real::sqrt(Self::distance_squared(left, right))
    }

    fn distance_squared(left: &Self, right: &Self) -> Real {
        let x = left.x - right.x;
        let y = left.y - right.y;

        x * x + y * y
    }

    fn dot(left: &Self, right: &Self) -> Real {
        left.x * right.x + left.y * right.y
    }

    fn exp(value: &Self) -> Self {
        Self::new(Real::exp(value.x), Real::exp(value.y))
    }

    fn exp_mut(&mut self) -> &Self {
        let temp = Self::exp(self);
        *self = temp;

        self
    }

    fn length(&self) -> Real {
        Real::sqrt(self.length_squared())
    }

    fn length_squared(&self) -> Real {
        self.x * self.x + self.y * self.y
    }

    fn ln(value: &Self) -> Self {
        Self::new(Real::ln(value.x), Real::ln(value.y))
    }

    fn ln_mut(&mut self) -> &Self {
        let temp = Self::ln(self);
        *self = temp;

        self
    }

    fn max(left: &Self, right: &Self) -> Self {
        Self::new(Real::max(left.x, right.x), Real::max(left.y, right.y))
    }

    fn min(left: &Self, right: &Self) -> Self {
        Self::new(Real::min(left.x, right.x), Real::min(left.y, right.y))
    }

    fn normalize(value: &Self) -> Self {
        let recip_length = value.length().recip();
        Self::new(value.x * recip_length, value.y * recip_length)
    }

    fn normalize_mut(&mut self) -> &Self {
        let temp = Self::normalize(self);
        *self = temp;

        self
    }

    fn reciprical(value: &Self) -> Self {
        Self::new(value.x.recip(), value.y.recip())
    }

    fn reciprical_mut(&mut self) -> &Self {
        let temp = Self::reciprical(self);
        *self = temp;

        self
    }
}

// Index operators.
impl Index<usize> for Vector2 {
    type Output = Real;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => &self.x,
        }
    }
}

impl IndexMut<usize> for Vector2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => &mut self.x,
        }
    }
}

// Vector unary operators.
impl_op_ex!(-|value: &Vector2| -> Vector2 { Vector2::new(-value.x, -value.y) });

// Vector binary operators.
impl_op_ex!(+|left: &Vector2, right: &Vector2| -> Vector2 {
    Vector2::new(left.x + right.x, left.y + right.y)
});

impl_op_ex!(-|left: &Vector2, right: &Vector2| -> Vector2 { Vector2::new(left.x - right.x, left.y - right.y) });

impl_op_ex!(^|left: &Vector2, right: &Vector2| -> Real {
    Vector2::dot(left, right)
});

// Vector binary assignment operators.
impl_op_ex!(+=|left: &mut Vector2, right: &Vector2| {
    left.x += right.x;
    left.y += right.y;
});

impl_op_ex!(-=|left: &mut Vector2, right: &Vector2| {
    left.x -= right.x;
    left.y -= right.y;
});

// Scalar binary operators.
impl_op_ex_commutative!(+|left: &Vector2, right: &Real| -> Vector2 {
    Vector2::new(left.x + right, left.y + right)
});

impl_op_ex!(-|left: &Vector2, right: &Real| -> Vector2 { Vector2::new(left.x - right, left.y - right) });

impl_op_ex!(-|left: &Real, right: &Vector2| -> Vector2 { Vector2::new(left - right.x, left - right.y) });

impl_op_ex_commutative!(*|left: &Vector2, right: &Real| -> Vector2 { Vector2::new(left.x * right, left.y * right) });

impl_op_ex!(/|left: &Vector2, right: &Real| -> Vector2 {
    Vector2::new(left.x / right, left.y / right)
});

impl_op_ex!(/|left: &Real, right: &Vector2| -> Vector2 {
    Vector2::new(left / right.x, left / right.y)
});

// Scalar assignment binary operators.
impl_op_ex!(+=|left: &mut Vector2, right: &Real| {
    left.x += right;
    left.y += right;
});

impl_op_ex!(-=|left: &mut Vector2, right: &Real| {
    left.x -= right;
    left.y -= right;
});

impl_op_ex!(*=|left: &mut Vector2, right: &Real| {
    left.x *= right;
    left.y *= right;
});

impl_op_ex!(/=|left: &mut Vector2, right: &Real| {
    left.x /= right;
    left.y /= right;
});

impl AbsDiffEq for Vector2 {
    type Epsilon = Real;

    fn default_epsilon() -> Real {
        Real::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Real) -> bool {
        Real::abs_diff_eq(&self.x, &other.x, epsilon) && Real::abs_diff_eq(&self.y, &other.y, epsilon)
    }
}

impl RelativeEq for Vector2 {
    fn default_max_relative() -> Real {
        Real::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: Real, max_relative: Real) -> bool {
        Real::relative_eq(&self.x, &other.x, epsilon, max_relative)
            && Real::relative_eq(&self.y, &other.y, epsilon, max_relative)
    }
}

impl UlpsEq for Vector2 {
    fn default_max_ulps() -> u32 {
        Real::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Real, max_ulps: u32) -> bool {
        Real::ulps_eq(&self.x, &other.x, epsilon, max_ulps) && Real::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
    }
}
