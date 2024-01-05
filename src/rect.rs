use crate::{lerp, vec2, Pos2, Vec2};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Rect {
    /// X position.
    pub x: f32,
    /// Y position.
    pub y: f32,
    /// Width.
    pub w: f32,
    /// Height.
    pub h: f32,
}

/// `rect(x, y, w, h)` == `Rect::new(x, y, w, h)`
#[inline]
pub const fn rect(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect::new(x, y, w, h)
}

impl Rect {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        w: 0.0,
        h: 0.0,
    };

    /// Create a new rectangle.
    #[inline]
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    #[inline]
    pub fn from_min_max(min: Pos2, max: Pos2) -> Self {
        Self {
            x: min.x,
            y: min.y,
            w: max.x - min.x,
            h: max.y - min.y,
        }
    }

    #[inline]
    pub const fn from_min_size(min: Pos2, size: Vec2) -> Self {
        Self {
            x: min.x,
            y: min.y,
            w: size.x,
            h: size.y,
        }
    }

    #[inline]
    pub fn contains(&self, pos: Pos2) -> bool {
        pos.x >= self.x && pos.x <= self.x + self.w && pos.y >= self.y && pos.y <= self.y + self.h
    }

    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }

    #[inline]
    pub fn center(&self) -> Pos2 {
        Pos2::new(self.x + self.w / 2.0, self.y + self.h / 2.0)
    }

    #[inline]
    pub const fn min(&self) -> Pos2 {
        Pos2::new(self.x, self.y)
    }

    #[inline]
    pub fn max(&self) -> Pos2 {
        Pos2::new(self.x + self.w, self.y + self.h)
    }

    #[inline]
    pub const fn size(&self) -> Vec2 {
        vec2(self.w, self.h)
    }

    #[inline]
    pub fn expand(&self, amount: f32) -> Self {
        Self {
            x: self.x - amount,
            y: self.y - amount,
            w: self.w + amount * 2.0,
            h: self.h + amount * 2.0,
        }
    }

    #[inline]
    pub fn expand_to_include(&self, pos: Pos2) -> Self {
        Self {
            x: self.x.min(pos.x),
            y: self.y.min(pos.y),
            w: self.w.max(pos.x - self.x),
            h: self.h.max(pos.y - self.y),
        }
    }

    #[inline]
    pub fn translate(&self, offset: Vec2) -> Self {
        Self {
            x: self.x + offset.x,
            y: self.y + offset.y,
            w: self.w,
            h: self.h,
        }
    }

    #[inline]
    pub fn round(&self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            w: self.w.round(),
            h: self.h.round(),
        }
    }

    #[inline]
    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            w: self.w.ceil(),
            h: self.h.ceil(),
        }
    }

    #[inline]
    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            w: self.w.floor(),
            h: self.h.floor(),
        }
    }

    #[inline]
    pub const fn set_x(self, x: f32) -> Self {
        Self { x, ..self }
    }

    #[inline]
    pub const fn set_y(self, y: f32) -> Self {
        Self { y, ..self }
    }

    #[inline]
    pub const fn set_w(self, w: f32) -> Self {
        Self { w, ..self }
    }

    #[inline]
    pub const fn set_h(self, h: f32) -> Self {
        Self { h, ..self }
    }

    #[inline]
    pub fn area(&self) -> f32 {
        self.w * self.h
    }

    /// The distance from the rect to the position.
    ///
    /// The distance is zero when the position is in the interior of the rectangle.
    #[inline]
    pub fn distance_to_pos(&self, pos: Pos2) -> f32 {
        self.distance_sq_to_pos(pos).sqrt()
    }

    /// The distance from the rect to the position, squared.
    ///
    /// The distance is zero when the position is in the interior of the rectangle.
    #[inline]
    pub fn distance_sq_to_pos(&self, pos: Pos2) -> f32 {
        let dx = if pos.x < self.x {
            self.x - pos.x
        } else if pos.x > self.x + self.w {
            pos.x - (self.x + self.w)
        } else {
            0.0
        };

        let dy = if pos.y < self.y {
            self.y - pos.y
        } else if pos.y > self.y + self.h {
            pos.y - (self.y + self.h)
        } else {
            0.0
        };

        dx * dx + dy * dy
    }

    /// Signed distance to the edge of the box.
    ///
    /// Negative inside the box.
    ///
    /// ```
    /// # use xd2d::{pos2, Rect};
    /// let rect = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
    /// assert_eq!(rect.signed_distance_to_pos(pos2(0.50, 0.50)), -0.50);
    /// assert_eq!(rect.signed_distance_to_pos(pos2(0.75, 0.50)), -0.25);
    /// assert_eq!(rect.signed_distance_to_pos(pos2(1.50, 0.50)), 0.50);
    /// ```
    pub fn signed_distance_to_pos(&self, pos: Pos2) -> f32 {
        let edge_distances = (pos - self.center()).abs() - self.size() * 0.5;
        let inside_dist = edge_distances.max_elem().min(0.0);
        let outside_dist = edge_distances.max(Vec2::ZERO).length();
        inside_dist + outside_dist
    }

    /// Linearly interpolate so that `[0, 0]` is the minimum point and
    /// `[1, 1]` is the maximum point.
    #[inline]
    pub fn lerp_inside(&self, t: Vec2) -> Pos2 {
        Pos2 {
            x: lerp(self.min().x..=self.max().x, t.x),
            y: lerp(self.min().y..=self.max().y, t.y),
        }
    }

    /// Linearly self towards other rect.
    #[inline]
    pub fn lerp_towards(&self, other: &Rect, t: f32) -> Self {
        Self {
            x: lerp(self.x..=other.x, t),
            y: lerp(self.y..=other.y, t),
            w: lerp(self.w..=other.w, t),
            h: lerp(self.h..=other.h, t),
        }
    }

    /// `w < 0 || h < 0`
    #[inline]
    pub fn is_negative(&self) -> bool {
        self.w < 0.0 || self.h < 0.0
    }

    /// `width > 0 && height > 0`
    #[inline]
    pub fn is_positive(&self) -> bool {
        self.w > 0.0 || self.h > 0.0
    }

    /// True if all members are also finite.
    #[inline(always)]
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.w.is_finite() && self.h.is_finite()
    }

    /// True if any member is NaN.
    #[inline(always)]
    pub fn any_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.w.is_nan() || self.h.is_nan()
    }
}

impl From<[f32; 4]> for Rect {
    #[inline]
    fn from([x, y, w, h]: [f32; 4]) -> Self {
        Self::new(x, y, w, h)
    }
}

impl From<(f32, f32, f32, f32)> for Rect {
    #[inline]
    fn from((x, y, w, h): (f32, f32, f32, f32)) -> Self {
        Self::new(x, y, w, h)
    }
}

impl From<(Pos2, Vec2)> for Rect {
    #[inline]
    fn from((min, size): (Pos2, Vec2)) -> Self {
        Self::from_min_size(min, size)
    }
}

impl From<Rect> for Vec2 {
    #[inline]
    fn from(rect: Rect) -> Self {
        Self::new(rect.x, rect.y)
    }
}

impl From<Rect> for Pos2 {
    #[inline]
    fn from(rect: Rect) -> Self {
        Self::new(rect.x, rect.y)
    }
}
