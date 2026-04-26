use sdl3_sys::render::SDL_Renderer;

pub use sdl3_sys::rect::{SDL_FRect, SDL_Rect};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Amount {
    /// Percentage as a float, 50.0 is equal to 50%
    Percentage(f32),

    /// Fraction
    Fraction(f32),

    /// Size in pixels
    Pixel(f32),
}

impl Amount {
    fn negative(&self) -> bool {
        match self {
            Self::Percentage(x) => x.is_sign_negative(),
            Self::Fraction(x) => x.is_sign_negative(),
            Self::Pixel(x) => x.is_sign_negative(),
        }
    }

    fn calculate(&self, value: f32) -> (f32, f32) {
        let x = match self {
            Self::Percentage(x) => value * (x.abs() / 100.0),
            Self::Fraction(x) => value * x.abs(),
            Self::Pixel(x) => x.abs(),
        };

        (x, value - x)
    }
}

impl std::ops::Neg for Amount {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Percentage(x) => Self::Percentage(-x),
            Self::Fraction(x) => Self::Fraction(-x),
            Self::Pixel(x) => Self::Pixel(-x),
        }
    }
}

impl From<f32> for Amount {
    fn from(value: f32) -> Self {
        Self::Percentage(value)
    }
}

#[derive(Debug, PartialEq, Clone)]
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

    /// Splits rect vertically, returning two rects
    pub fn vertical(self, amount: Amount) -> (Self, Self) {
        let (height, rem_height) = amount.calculate(self.height);

        if !amount.negative() {
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
        } else {
            (
                Self {
                    x: self.x,
                    y: self.y + rem_height,
                    width: self.width,
                    height: height,
                },
                Self {
                    x: self.x,
                    y: self.y,
                    width: self.width,
                    height: rem_height,
                }
            )
        }
    }

    /// Reserves certain amount horizontally
    pub fn horizontal(self, amount: Amount) -> (Self, Self) {
        let (width, rem_width) = amount.calculate(self.width);

        if !amount.negative() {
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
        } else {
            (
                Self {
                    x: self.x + rem_width,
                    y: self.y,
                    width: width,
                    height: self.height,
                },
                Self {
                    x: self.x,
                    y: self.y,
                    width: rem_width,
                    height: self.height,
                }
            )
        }
    }

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

    /// Center vertically
    pub fn center_v(self, height: f32) -> Option<Self> {
        let height = height.abs();

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
        let width = width.abs();

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
    use std::ops::Deref;
    use super::{Rect, Amount};

    #[derive(Debug)]
    struct RectWrapper(Rect);

    impl PartialEq for RectWrapper {
        fn eq(&self, other: &Self) -> bool {
            // NOTE the error for floats is arbitrary atm
            fn is_eq (a: f32, b: f32) -> bool { (a - b).abs() < 0.0001 }

            is_eq(self.x, other.x) &&
            is_eq(self.y, other.y) &&
            is_eq(self.width, other.width) &&
            is_eq(self.height, other.height)
        }
    }

    impl Eq for RectWrapper {}

    impl Deref for RectWrapper {
        type Target = Rect;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[allow(unused)]
    macro_rules! rect_eq {
        ($a:expr, $b:expr $(, $msg:expr)?) => {
            assert_eq!(RectWrapper($a), RectWrapper($b) $(, $msg)?)
        };
    }

    #[allow(unused)]
    macro_rules! rect_tuple_eq {
        ($a:expr, $b:expr $(, $msg:expr)?) => {
            {
                let lhs = $a;
                let rhs = $b;
                assert_eq!(
                    (
                        RectWrapper(lhs.0),
                        RectWrapper(lhs.1),
                    ),
                    (
                        RectWrapper(rhs.0),
                        RectWrapper(rhs.1),
                    )
                    $(, $msg)?
                )
            }
        }
    }

    #[test]
    fn test_amount() {
        assert_eq!(Amount::Percentage(30.).calculate(300.), (90., 210.));
        assert_eq!(Amount::Fraction(1./3.).calculate(300.), (100., 200.));
        assert_eq!(Amount::Pixel(100.).calculate(300.), (100., 200.));

        // negative amounts (they should be the same)
        assert_eq!(Amount::Percentage(-30.).calculate(300.), (90., 210.));
        assert_eq!(Amount::Fraction(-1./3.).calculate(300.), (100., 200.));
        assert_eq!(Amount::Pixel(-100.).calculate(300.), (100., 200.));
    }

    #[test]
    fn test_rect() {
        // vertical
        rect_tuple_eq!(
            Rect::new(0.0, 0.0, 200.0, 100.0).vertical(Amount::Pixel(30.0)),
            (
                Rect::new(0.0, 0.0, 200.0, 30.0),
                Rect::new(0.0, 30.0, 200.0, 70.0),
            )
        );

        rect_tuple_eq!(
            Rect::new(0.0, 0.0, 200.0, 100.0).vertical(Amount::Pixel(-30.0)),
            (
                Rect::new(0.0, 70.0, 200.0, 30.0),
                Rect::new(0.0, 0.0, 200.0, 70.0),
            )
        );

        // horizontal
        rect_tuple_eq!(
            Rect::new(0.0, 0.0, 200.0, 100.0).horizontal(Amount::Pixel(50.0)),
            (
                Rect::new(0.0, 0.0, 50.0, 100.0),
                Rect::new(50.0, 0.0, 150.0, 100.0),
            )
        );

        rect_tuple_eq!(
            Rect::new(0.0, 0.0, 200.0, 100.0).horizontal(Amount::Pixel(-50.0)),
            (
                Rect::new(150.0, 0.0, 50.0, 100.0),
                Rect::new(0.0, 0.0, 150.0, 100.0),
            )
        );

        // vertical fraction
        rect_tuple_eq!(
            Rect::new(0.0, 0.0, 200.0, 100.0).vertical(Amount::Fraction(1./3.)),
            (
                Rect::new(0.0, 0.0, 200.0, 33.3333),
                Rect::new(0.0, 33.3333, 200.0, 66.6666),
            )
        );

        rect_tuple_eq!(
            Rect::new(0.0, 0.0, 200.0, 100.0).vertical(Amount::Fraction(-1./3.)),
            (
                Rect::new(0.0, 66.6666, 200.0, 33.3333),
                Rect::new(0.0, 0.0, 200.0, 66.6666),
            )
        );

        // margin
        rect_eq!(
            Rect::new(0.0, 0.0, 200.0, 100.0).margin(Amount::Pixel(5.)),
            Rect::new(5., 5., 190., 90.)
        );

        // margin all
        rect_eq!(
            Rect::new(0.0, 0.0, 200.0, 100.0).margin_all(2., 4., 6., 8.),
            Rect::new(6., 2., 186., 94.)
        );

        // center
        rect_eq!(
            Rect::new(0.0, 0.0, 200.0, 100.0).centered(100., 50.).unwrap(),
            Rect::new(50., 25., 100., 50.)
        );
    }

    #[test]
    fn test_rect_complex() {
        // a basic layout
        let rect = Rect::new(0., 0., 800., 600.);
        let (toolbar, rect) = rect.vertical(Amount::Pixel(30.));
        let (sidebar, rect) = rect.horizontal(Amount::Pixel(-60.));
        let content = rect.margin(Amount::Pixel(10.));

        rect_eq!(toolbar, Rect::new(0., 0., 800., 30.));
        rect_eq!(sidebar, Rect::new(740., 30., 60., 570.));
        rect_eq!(content, Rect::new(10., 40., 720., 550.));
    }
}
