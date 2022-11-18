use crate::math::{vector::Vector, vector3::Vector3};

/// Represents an AABB (Axis Aligned Bounding Box) that encloses points or geometry.
#[derive(Debug)]
pub struct BoundingBox {
    pub minimum: Vector3,
    pub maximum: Vector3,
}

impl BoundingBox {
    /// Creates a new [`BoundingBox`] with a specific minimum and maximum.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// #
    /// let minimum = Vector3::from_value(-2.0);
    /// let maximum = Vector3::from_value(3.0);
    ///
    /// let bounding_box = BoundingBox::new(&minimum, &maximum);
    ///
    /// assert_eq!(minimum, bounding_box.minimum);
    /// assert_eq!(maximum, bounding_box.maximum);
    /// ```
    pub fn new(minimum: &Vector3, maximum: &Vector3) -> BoundingBox {
        Self {
            minimum: *minimum,
            maximum: *maximum,
        }
    }

    /// Creates a new [`BoundingBox`] with the minimum set to negative infinity and the maximum set to positive
    /// infinity. This would encompass all points and geometry in all space.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// #
    /// let bounding_box = BoundingBox::new_infinity();
    ///
    /// assert_eq!(-Vector3::new_infinity(), bounding_box.minimum);
    /// assert_eq!(Vector3::new_infinity(), bounding_box.maximum);
    /// ```
    pub fn new_infinity() -> BoundingBox {
        Self::new(&-Vector3::new_infinity(), &Vector3::new_infinity())
    }

    /// Creates a new [`BoundingBox`] with the minimum set to positive infinity and the maximum set to negative
    /// infinity. This would encompass no points or geometry.
    ///
    /// This is useful for starting with a bounding box that doesn't contain anything so you can union points and
    /// geometry into it allowing it to grow to fit its contents.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// #
    /// let bounding_box = BoundingBox::new_inverse_infinity();
    ///
    /// assert_eq!(Vector3::new_infinity(), bounding_box.minimum);
    /// assert_eq!(-Vector3::new_infinity(), bounding_box.maximum);
    /// ```
    pub fn new_inverse_infinity() -> BoundingBox {
        Self::new(&Vector3::new_infinity(), &-Vector3::new_infinity())
    }

    /// Creates a new [`BoundingBox`] that exactly contains all of the points provided.
    ///
    /// Due to floating point rounding errors, it is often a good idea to add margin using the
    /// [`BoundingBox::add_margin()`] method to ensure tests against the bounding box will succeed even in the case of
    /// small floating point errors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// #
    /// let points = vec![
    ///     Vector3::from_value(0.0),
    ///     Vector3::from_value(-3.0),
    ///     Vector3::from_value(7.0),
    ///     Vector3::from_value(2.0),
    /// ];
    ///
    /// let bounding_box = BoundingBox::from_points(points.iter());
    ///
    /// assert_eq!(Vector3::from_value(-3.0), bounding_box.minimum);
    /// assert_eq!(Vector3::from_value(7.0), bounding_box.maximum);
    /// ```
    pub fn from_points<'a>(points: impl Iterator<Item = &'a Vector3>) -> BoundingBox {
        let mut minimum = Vector3::new_infinity();
        let mut maximum = -Vector3::new_infinity();

        for point in points {
            minimum = Vector3::min(&minimum, point);
            maximum = Vector3::max(&maximum, point);
        }

        BoundingBox::new(&minimum, &maximum)
    }

    /// Increases the [`BoundingBox`] by the specified margin amount. Returns a mutable reference to itself for easy
    /// chaining of method calls.
    ///
    /// The operation performed is that the minimum is decreased by the margin amount and the maximum is increased by
    /// the margin amount.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// #
    /// let minimum = Vector3::from_value(-2.0);
    /// let maximum = Vector3::from_value(3.0);
    ///
    /// let mut bounding_box = BoundingBox::new(&minimum, &maximum);
    ///
    /// let margin = Vector3::from_value(5.0);
    /// bounding_box.add_margin(&margin);
    ///
    /// assert_eq!(Vector3::from_value(-7.0), bounding_box.minimum);
    /// assert_eq!(Vector3::from_value(8.0), bounding_box.maximum);
    /// ```
    pub fn add_margin(&mut self, margin_amount: &Vector3) -> &mut BoundingBox {
        self.minimum -= margin_amount;
        self.maximum += margin_amount;

        self
    }

    /// Increase the [`BoundingBox`] dimensions so that it contains the provided point. If the point is already
    /// contained within the [`BoundingBox`] is not changed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// #
    /// let minimum = Vector3::from_value(-2.0);
    /// let maximum = Vector3::from_value(3.0);
    ///
    /// let mut bounding_box = BoundingBox::new(&minimum, &maximum);
    ///
    /// let point = Vector3::new(0.0, 9.0, -7.0);
    /// bounding_box.add_point(&point);
    ///
    /// assert_eq! (Vector3::new(-2.0, -2.0, -7.0), bounding_box.minimum);
    /// assert_eq!(Vector3::new(3.0, 9.0, 3.0), bounding_box.maximum);
    /// ```
    pub fn add_point(&mut self, point: &Vector3) -> &mut BoundingBox {
        self.minimum = Vector3::min(point, &self.minimum);
        self.maximum = Vector3::max(point, &self.maximum);

        self
    }

    /// Increases the [`BoundingBox`] dimensions so that it contains the provided bounding box.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// #
    /// let minimum = Vector3::from_value(-2.0);
    /// let maximum = Vector3::from_value(3.0);
    ///
    /// let mut bounding_box = BoundingBox::new(&minimum, &maximum);
    ///
    /// let other_bounding_box = BoundingBox::new(&Vector3::from_value(-7.0), &Vector3::from_value(9.0));
    /// bounding_box.add_bounding_box(&other_bounding_box);
    ///
    /// assert_eq! (Vector3::from_value(-7.0), bounding_box.minimum);
    /// assert_eq!(Vector3::from_value(9.0), bounding_box.maximum);
    /// ```
    pub fn add_bounding_box(&mut self, other_bounding_box: &BoundingBox) -> &mut BoundingBox {
        self.minimum = Vector3::min(&other_bounding_box.minimum, &self.minimum);
        self.maximum = Vector3::max(&other_bounding_box.maximum, &self.maximum);

        self
    }
}
