/// A 2D point / vector in the section plane.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub(crate) fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }

    pub(crate) fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    pub(crate) fn scale(self, s: f64) -> Self {
        Self::new(self.x * s, self.y * s)
    }

    pub(crate) fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub(crate) fn normalized(self) -> Self {
        let l = self.length();
        Self::new(self.x / l, self.y / l)
    }

    /// Rotate around the origin by `theta` radians (CCW positive).
    pub(crate) fn rotate(self, theta: f64) -> Self {
        let (s, c) = theta.sin_cos();
        Self::new(self.x * c - self.y * s, self.x * s + self.y * c)
    }
}
