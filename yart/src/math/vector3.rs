use super::{color3::Color3, vector::Vector, vector2::Vector2};
use crate::common::*;
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use impl_ops::*;
use std::ops::{self, Index, IndexMut};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: Real,
    pub y: Real,
    pub z: Real,
}

impl Vector3 {
    /// Constructs a new vector with a value for each component.
    ///
    /// # Examples
    ///
    /// ```
    /// # use approx::*;
    /// # use yart::common::*;
    /// # use yart::math::vector::*;
    /// # use yart::math::vector3::*;
    /// #
    /// let result = Vector3::new(2.0, 3.0, 4.0);
    ///
    /// assert_eq!(2.0, result.x);
    /// assert_eq!(3.0, result.y);
    /// assert_eq!(4.0, result.z);
    /// ```
    pub fn new(x: Real, y: Real, z: Real) -> Self {
        Self { x, y, z }
    }

    /// Constructs a new vector using the provided [`Vector2`] for the x and y coordintes and a second parameter for the
    /// z axis.
    ///
    /// # Examples
    ///
    /// ```
    /// # use approx::*;
    /// # use yart::common::*;
    /// # use yart::math::vector::*;
    /// # use yart::math::vector2::*;
    /// # use yart::math::vector3::*;
    /// #
    /// let vector2 = Vector2::new(2.0, 3.0);
    /// let result = Vector3::from_vector2(&vector2, 4.0);
    ///
    /// assert_eq!(2.0, result.x);
    /// assert_eq!(3.0, result.y);
    /// assert_eq!(4.0, result.z);
    /// ```
    pub fn from_vector2(vector2: &Vector2, z: Real) -> Self {
        Self::new(vector2.x, vector2.y, z)
    }

    /// Constructs a new vector using the provided [`Color3`]. The components are assigned as follows: `r -> x`, `g ->
    /// y`, `b -> z`.
    ///
    /// ```
    /// # use approx::*;
    /// # use yart::common::*;
    /// # use yart::math::color3::*;
    /// # use yart::math::vector::*;
    /// # use yart::math::vector3::*;
    /// #
    /// let color3 = Color3::new(0.2, 0.3, 0.4);
    /// let result = Vector3::from_color3(&color3);
    ///
    /// assert_eq!(0.2, result.x);
    /// assert_eq!(0.3, result.y);
    /// assert_eq!(0.4, result.z);
    /// ```
    pub fn from_color3(color3: &Color3) -> Self {
        Self::new(color3.r, color3.g, color3.b)
    }

    pub fn build_perpendicular_vector(value: &Self) -> Self {
        // From: Efficient Construction of Perpendicular Vectors Without Branching.
        let a = Self::abs(value);

        let xm = if a.x - a.y < 0.0 && a.x - a.z < 0.0 { 1u32 } else { 0u32 };
        let ym = if a.y - a.z < 0.0 { 1u32 ^ xm } else { 0u32 };
        let zm = 1u32 ^ (xm | ym);

        value % Self::new(xm as Real, ym as Real, zm as Real)
    }

    /// Performs the cross product between two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use approx::*;
    /// # use yart::common::*;
    /// # use yart::math::vector::*;
    /// # use yart::math::vector3::*;
    /// #
    /// let left = Vector3::new(2.0, 3.0, 4.0);
    /// let right = Vector3::new(5.0, 6.0, 7.0);
    ///
    /// let result = Vector3::cross(&left, &right);
    ///
    /// assert_relative_eq!(Vector3::new(-3.0, 6.0, -3.0), &result, max_relative = EPSILON);
    /// ```
    pub fn cross(left: &Self, right: &Self) -> Self {
        Self::new(
            left.y * right.z - left.z * right.y,
            left.z * right.x - left.x * right.z,
            left.x * right.y - left.y * right.x,
        )
    }

    pub fn project_onto(&self, vector_to_project_onto: &Self) -> Self {
        vector_to_project_onto * ((self ^ vector_to_project_onto) / (vector_to_project_onto ^ vector_to_project_onto))
    }

    /// Returns a new vector that is an outgoing reflection of itself around the normal.
    ///
    /// ```text
    /// ^    ^    ^
    ///  \   |   /
    ///   \  |  /
    ///    \ | /
    ///    s n r
    /// s = self
    /// n = normal
    /// r = reflected (result)
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # use approx::*;
    /// # use yart::common::*;
    /// # use yart::math::vector::*;
    /// # use yart::math::vector3::*;
    /// #
    /// let target = Vector3::new(-0.707, 0.707, 0.0);
    /// let normal = Vector3::new(0.0, 1.0, 0.0);
    ///
    /// let result = target.reflect(&normal);
    ///
    /// assert_relative_eq!(Vector3::new(0.707, 0.707, 0.0), result, max_relative = EPSILON);
    /// ```
    pub fn reflect(&self, normal: &Self) -> Self {
        self - 2.0 * (self ^ normal) * normal
    }

    pub fn refract(incoming_direction: &Self, normal: &Self, from_index: Real, to_index: Real) -> Self {
        let n = from_index / to_index;
        let cos = -(incoming_direction ^ normal);

        let under_sqrt_root = 1.0 - n * n * (1.0 - cos * cos);
        if under_sqrt_root < 0.0 {
            Self::default()
        } else {
            (n * incoming_direction) - (n * cos - Real::sqrt(under_sqrt_root)) * normal
        }
    }

    pub fn schlick_approximation(incoming_direction: &Self, normal: &Self, from_index: Real, to_index: Real) -> Real {
        let r = (from_index / to_index) / (from_index + to_index);
        let r_2 = r * r;

        let cos = -(incoming_direction ^ normal);
        let x = 1.0 - cos;

        r_2 + (1.0 - r_2) * x * x * x * x * x
    }
}

impl Vector for Vector3 {
    /// Creates a vector from a scalar value. All components of the vector are set to the scalar value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use approx::*;
    /// # use yart::common::*;
    /// # use yart::math::vector::*;
    /// # use yart::math::vector3::*;
    /// #
    /// let result = Vector3::from_value(2.0);
    ///
    /// assert_eq!(Vector3::new(2.0, 2.0, 2.0), result);
    /// ```
    fn from_value(value: Real) -> Self {
        Self::new(value, value, value)
    }

    /// Creates a new vector where each component is the absolute value of the corresponding component of the input
    /// vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use approx::*;
    /// # use yart::common::*;
    /// # use yart::math::vector::*;
    /// # use yart::math::vector3::*;
    /// #
    /// let vector = Vector3::new(-2.0, 3.0, -4.0);
    /// let result = Vector3::abs(&vector);
    ///
    /// assert_eq!(Vector3::new(2.0, 3.0, 4.0), result);
    /// ```
    fn abs(value: &Self) -> Self {
        Self::new(Real::abs(value.x), Real::abs(value.y), Real::abs(value.z))
    }

    fn abs_mut(&mut self) -> &Self {
        let temp = Self::abs(self);
        *self = temp;

        self
    }

    /// Performs a component-wise multiplication of two vectors. That is the resulting vector is `(left.x * right.x,
    /// left.y * right.y, left.z * right.z)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use approx::*;
    /// # use yart::common::*;
    /// # use yart::math::vector::*;
    /// # use yart::math::vector3::*;
    /// #
    /// let left = Vector3::new(2.0, 3.0, 4.0);
    /// let right = Vector3::new(5.0, 6.0, 7.0);
    ///
    /// let result = Vector3::component_mul(&left, &right);
    ///
    /// assert_eq!(Vector3::new(10.0, 18.0, 28.0), result);
    /// ```
    fn component_mul(left: &Self, right: &Self) -> Self {
        Self::new(left.x * right.x, left.y * right.y, left.z * right.z)
    }

    /// Calculates the distance between two points.
    ///
    /// # Examples
    ///
    /// ```
    /// # use approx::*;
    /// # use yart::common::*;
    /// # use yart::math::vector::*;
    /// # use yart::math::vector3::*;
    /// #
    /// let left = Vector3::new(2.0, 3.0, 4.0);
    /// let right = Vector3::new(5.0, 6.0, 7.0);
    ///
    /// let result = Vector3::distance(&left, &right);
    ///
    /// assert_relative_eq!(5.196152, result, max_relative = EPSILON);
    /// ```
    fn distance(left: &Self, right: &Self) -> Real {
        Real::sqrt(Self::distance_squared(left, right))
    }

    /// Calculates the distance squared between two points. That is the distance between two points multiplied by
    /// itself.
    ///
    /// # Examples
    ///
    /// ```
    /// # use approx::*;
    /// # use yart::common::*;
    /// # use yart::math::vector::*;
    /// # use yart::math::vector3::*;
    /// #
    /// let left = Vector3::new(2.0, 3.0, 4.0);
    /// let right = Vector3::new(5.0, 6.0, 7.0);
    ///
    /// let result = Vector3::distance_squared(&left, &right);
    ///
    /// assert_relative_eq!(27.0, result, max_relative = EPSILON);
    /// ```
    fn distance_squared(left: &Self, right: &Self) -> Real {
        let x = left.x - right.x;
        let y = left.y - right.y;
        let z = left.z - right.z;

        x * x + y * y + z * z
    }

    /// Calculates the dot product between two vectors. The dot product is
    /// `left.x * right.x + left.y * right.y + left.z * right.z`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use approx::*;
    /// # use yart::common::*;
    /// # use yart::math::vector::*;
    /// # use yart::math::vector3::*;
    /// #
    /// let left = Vector3::new(2.0, 3.0, 4.0);
    /// let right = Vector3::new(5.0, 6.0, 7.0);
    ///
    /// let result = Vector3::dot(&left, &right);
    ///
    /// assert_eq!(56.0, result);
    /// ```
    fn dot(left: &Self, right: &Self) -> Real {
        left.x * right.x + left.y * right.y + left.z * right.z
    }

    fn exp(value: &Self) -> Self {
        Self::new(Real::exp(value.x), Real::exp(value.y), Real::exp(value.z))
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
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn ln(value: &Self) -> Self {
        Self::new(Real::ln(value.x), Real::ln(value.y), Real::ln(value.z))
    }

    fn ln_mut(&mut self) -> &Self {
        let temp = Self::ln(self);
        *self = temp;

        self
    }

    fn max(left: &Self, right: &Self) -> Self {
        Self::new(
            Real::max(left.x, right.x),
            Real::max(left.y, right.y),
            Real::max(left.z, right.z),
        )
    }

    fn min(left: &Self, right: &Self) -> Self {
        Self::new(
            Real::min(left.x, right.x),
            Real::min(left.y, right.y),
            Real::min(left.z, right.z),
        )
    }

    fn normalize(value: &Self) -> Self {
        let recip_length = value.length().recip();
        Self::new(value.x * recip_length, value.y * recip_length, value.z * recip_length)
    }

    fn normalize_mut(&mut self) -> &Self {
        let temp = Self::normalize(self);
        *self = temp;

        self
    }

    fn reciprical(value: &Self) -> Self {
        Self::new(value.x.recip(), value.y.recip(), value.z.recip())
    }

    fn reciprical_mut(&mut self) -> &Self {
        let temp = Self::reciprical(self);
        *self = temp;

        self
    }
}

// Index operators.
impl Index<usize> for Vector3 {
    type Output = Real;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => &mut self.x,
        }
    }
}

// Vector unary operators.
impl_op_ex!(-|value: &Vector3| -> Vector3 { Vector3::new(-value.x, -value.y, -value.z) });

// Vector binary operators.
impl_op_ex!(+|left: &Vector3, right: &Vector3| -> Vector3 {
    Vector3::new(left.x + right.x, left.y + right.y, left.z + right.z)
});

impl_op_ex!(-|left: &Vector3, right: &Vector3| -> Vector3 {
    Vector3::new(left.x - right.x, left.y - right.y, left.z - right.z)
});

impl_op_ex!(^|left: &Vector3, right: &Vector3| -> Real {
    Vector3::dot(left, right)
});

impl_op_ex!(%|left: &Vector3, right: &Vector3| -> Vector3 {
    Vector3::cross(left, right)
});

// Vector binary assignment operators.
impl_op_ex!(+=|left: &mut Vector3, right: &Vector3| {
    left.x += right.x;
    left.y += right.y;
    left.z += right.z;
});

impl_op_ex!(-=|left: &mut Vector3, right: &Vector3| {
    left.x -= right.x;
    left.y -= right.y;
    left.z -= right.z;
});

// Scalar binary operators.
impl_op_ex_commutative!(+|left: &Vector3, right: &Real| -> Vector3 {
    Vector3::new(left.x + right, left.y + right, left.z + right)
});

impl_op_ex!(-|left: &Vector3, right: &Real| -> Vector3 {
    Vector3::new(left.x - right, left.y - right, left.z - right)
});

impl_op_ex!(-|left: &Real, right: &Vector3| -> Vector3 {
    Vector3::new(left - right.x, left - right.y, left - right.z)
});

impl_op_ex_commutative!(*|left: &Vector3, right: &Real| -> Vector3 {
    Vector3::new(left.x * right, left.y * right, left.z * right)
});

impl_op_ex!(/|left: &Vector3, right: &Real| -> Vector3 {
    Vector3::new(left.x / right, left.y / right, left.z / right)
});

impl_op_ex!(/|left: &Real, right: &Vector3| -> Vector3 {
    Vector3::new(left / right.x, left / right.y, left / right.z)
});

// Scalar assignment binary operators.
impl_op_ex!(+=|left: &mut Vector3, right: &Real| {
    left.x += right;
    left.y += right;
    left.z += right;
});

impl_op_ex!(-=|left: &mut Vector3, right: &Real| {
    left.x -= right;
    left.y -= right;
    left.z -= right;
});

impl_op_ex!(*=|left: &mut Vector3, right: &Real| {
    left.x *= right;
    left.y *= right;
    left.z *= right;
});

impl_op_ex!(/=|left: &mut Vector3, right: &Real| {
    left.x /= right;
    left.y /= right;
    left.z /= right;
});

impl AbsDiffEq for Vector3 {
    type Epsilon = Real;

    fn default_epsilon() -> Real {
        Real::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Real) -> bool {
        Real::abs_diff_eq(&self.x, &other.x, epsilon)
            && Real::abs_diff_eq(&self.y, &other.y, epsilon)
            && Real::abs_diff_eq(&self.z, &other.z, epsilon)
    }
}

impl RelativeEq for Vector3 {
    fn default_max_relative() -> Real {
        Real::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: Real, max_relative: Real) -> bool {
        Real::relative_eq(&self.x, &other.x, epsilon, max_relative)
            && Real::relative_eq(&self.y, &other.y, epsilon, max_relative)
            && Real::relative_eq(&self.z, &other.z, epsilon, max_relative)
    }
}

impl UlpsEq for Vector3 {
    fn default_max_ulps() -> u32 {
        Real::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Real, max_ulps: u32) -> bool {
        Real::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
            && Real::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
            && Real::ulps_eq(&self.z, &other.z, epsilon, max_ulps)
    }
}
