use super::*;

/// A point in 2D.
#[derive(Default, Copy, Clone, PartialEq)]
pub struct Point {
    /// The x coordinate.
    pub x: Length,
    /// The y coordinate.
    pub y: Length,
}

impl Point {
    /// The origin point.
    pub const ZERO: Self = Self { x: Length::ZERO, y: Length::ZERO };

    /// Create a new point from x and y coordinate.
    pub fn new(x: Length, y: Length) -> Self {
        Self { x, y }
    }

    /// Create an instance with two equal components.
    pub fn uniform(value: Length) -> Self {
        Self { x: value, y: value }
    }
}

impl Get<SpecAxis> for Point {
    type Component = Length;

    fn get(self, axis: SpecAxis) -> Length {
        match axis {
            SpecAxis::Horizontal => self.x,
            SpecAxis::Vertical => self.y,
        }
    }

    fn get_mut(&mut self, axis: SpecAxis) -> &mut Length {
        match axis {
            SpecAxis::Horizontal => &mut self.x,
            SpecAxis::Vertical => &mut self.y,
        }
    }
}

impl Switch for Point {
    type Other = Gen<Length>;

    fn switch(self, main: SpecAxis) -> Self::Other {
        match main {
            SpecAxis::Horizontal => Gen::new(self.x, self.y),
            SpecAxis::Vertical => Gen::new(self.y, self.x),
        }
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Point({:?}, {:?})", self.x, self.y)
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

sub_impl!(Point - Point -> Point);

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self { x: self.x * other, y: self.y * other }
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        other * self
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self { x: self.x / other, y: self.y / other }
    }
}

assign_impl!(Point += Point);
assign_impl!(Point -= Point);
assign_impl!(Point *= f64);
assign_impl!(Point /= f64);
