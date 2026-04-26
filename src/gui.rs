use sdl3_sys::render::SDL_Renderer;

pub use sdl3_sys::rect::{SDL_FRect, SDL_Rect};

#[derive(Debug, PartialEq)]
pub enum Amount {
    /// Percentage as a float, 50.0 is equal to 50%
    Percentage(f32),

    /// Size in pixels
    Pixel(f32),
}

impl Amount {
    fn calculate(&self, value: f32) -> (f32, f32) {
        match self {
            Self::Percentage(x) => {
                let y = value * (x / 100.0);
                (y, value - y)
            },
            Self::Pixel(x) => (*x, value - x)
        }
    }
}

impl From<f32> for Amount {
    fn from(value: f32) -> Self {
        Self::Percentage(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn new_i(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            width: width as f32,
            height: height as f32,
        }
    }

    /// Center vertically
    pub fn center_v(self, height: f32) -> Option<Self> {
        if height > self.height {
            return None;
        }

        let up = (self.height - height) / 2.0;

        Some(Self {
            x: self.x,
            y: self.y + up,
            width: self.width,
            height,
        })
    }

    /// Center horizontally
    pub fn center_h(self, width: f32) -> Option<Self> {
        if width > self.width {
            return None;
        }

        let left = (self.width - width) / 2.0;

        Some(Self {
            x: self.x + left,
            y: self.y,
            width,
            height: self.height,
        })
    }

    /// Center a rect
    pub fn centered(self, width: f32, height: f32) -> Option<Self> { self.center_h(width)?.center_v(height) }

    /// Reserves certain amount vertically
    pub fn reserve_v(self, amount: Amount) -> (Self, Self) {
        let (height, rem_height) = amount.calculate(self.height);

        (
            Self {
                x: self.x,
                y: self.y,
                width: self.width,
                height,
            },
            Self {
                x: self.x,
                y: self.y + height,
                width: self.width,
                height: rem_height,
            }
        )
    }

    /// Reserves certain amount horizontally
    pub fn reserve_h(self, amount: Amount) -> (Self, Self) {
        let (width, rem_width) = amount.calculate(self.width);

        (
            Self {
                x: self.x,
                y: self.y,
                width,
                height: self.height,
            },
            Self {
                x: self.x + width,
                y: self.y,
                width: rem_width,
                height: self.height,
            }
        )
    }

    /// Divide vertically in two
    pub fn halve_v(self) -> (Self, Self) { self.reserve_v(50.0.into()) }

    /// Divide horizontally in two
    pub fn halve_h(self) -> (Self, Self) { self.reserve_h(50.0.into()) }

    /// Divide vertically by a fraction 1/x
    pub fn fraction_v(self, div: f32) -> (Self, Self) { self.reserve_v((100.0 / div).into()) }

    /// Divide horizontally by a fraction 1/x
    pub fn fraction_h(self, div: f32) -> (Self, Self) { self.reserve_h((100.0 / div).into()) }

    /// Add margin to the rect but each margin can be different
    pub fn margin_all(self, up: f32, down: f32, left: f32, right: f32) -> Self {
        Self {
            x: self.x + left,
            y: self.y + up,
            width: self.width - left - right,
            height: self.height - up - down,
        }
    }

    /// Add margin to whole rect
    pub fn margin(self, margin: Amount) -> Self {
        let (left, _) = margin.calculate(self.width);
        let (up, _) = margin.calculate(self.height);

        self.margin_all(up, up, left, left)
    }

    /// Add margin to top and bottom
    pub fn margin_v(self, margin: f32) -> Self { self.margin_all(margin, margin, 0.0, 0.0) }

    /// Add margin to left and right
    pub fn margin_h(self, margin: f32) -> Self { self.margin_all(0.0, 0.0, margin, margin) }
}

impl From<SDL_FRect> for Rect {
    fn from(value: SDL_FRect) -> Self {
        Self {
            x: value.x,
            y: value.y,
            width: value.w,
            height: value.h,
        }
    }
}

impl From<SDL_Rect> for Rect {
    fn from(value: SDL_Rect) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
            width: value.w as f32,
            height: value.h as f32,
        }
    }
}

pub trait Element {
    fn render(renderer: *mut SDL_Renderer, rect: Rect);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect() {
        assert_eq!(
            Rect::new(0.0, 0.0, 200.0, 100.0).halve_v(),
            (
                Rect::new(0.0, 0.0, 200.0, 50.0),
                Rect::new(0.0, 50.0, 200.0, 50.0),
            )
        );

        assert_eq!(
            Rect::new(0.0, 0.0, 200.0, 100.0).halve_h(),
            (
                Rect::new(0.0, 0.0, 100.0, 100.0),
                Rect::new(100.0, 00.0, 100.0, 100.0),
            )
        );

        // divide into equal thirds
        let (lhs1, lhs2) = Rect::new(0.0, 0.0, 200.0, 100.0).fraction_v(3.);
        let (rhs1, rhs2) = (
            Rect::new(0.0, 0.0, 200.0, 33.33333),
            Rect::new(100.0, 00.0, 200.0, 100.0),
        );

        // TODO round this
        assert_eq!(lhs1.x, rhs1.x);
        assert_eq!(lhs1.y, rhs1.y);
        assert_eq!(lhs1.width, rhs1.width);
        assert_eq!(lhs1.height, rhs1.height);

        assert_eq!(lhs2.x, rhs2.x);
        assert_eq!(lhs2.y, rhs2.y);
        assert_eq!(lhs2.width, rhs2.width);
        assert_eq!(lhs2.height, rhs2.height);
    }
}
