use std::collections::HashMap;

use sdl3_sys::render::SDL_Renderer;

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

type RectTuple = (f32, f32, f32, f32);

pub fn vertical(rect: RectTuple, amount: Amount) -> (RectTuple, RectTuple) {
    let (x, y, w, h) = rect;
    let (height, rem_height) = amount.calculate(h);

    if !amount.negative() {
        (
            (x, y, w, height),
            (x, y + height, w, rem_height),
        )
    } else {
        (
            (x, y + rem_height, w, height),
            (x, y, w, rem_height)
        )
    }
}

pub fn horizontal(rect: RectTuple, amount: Amount) -> (RectTuple, RectTuple) {
    let (x, y, w, h) = rect;
    let (width, rem_width) = amount.calculate(w);

    if !amount.negative() {
        (
            (x, y, width, h),
            (x + width, y, rem_width, h),
        )
    } else {
        (
            (x + rem_width, y, width, h),
            (x, y, rem_width, h)
        )
    }
}

pub fn margin_all(rect: RectTuple, up: f32, down: f32, left: f32, right: f32) -> RectTuple {
    let (x, y, w, h) = rect;
    (x + left, y + up, w - left - right, h - up - down)
}

pub fn margin(rect: RectTuple, margin: Amount) -> RectTuple {
    let (_, _, w, h) = rect;
    let (left, _) = margin.calculate(w);
    let (up, _) = margin.calculate(h);

    margin_all(rect, up, up, left, left)
}

pub fn center_v(rect: RectTuple, height: f32) -> RectTuple {
    let (x, y, w, h) = rect;
    if height > h {
        // just return the original if its too small
        return (x, y, w, h)
    }

    let up = (h - height) / 2.0;

    (x, y + up, w, height)
}

pub fn center_h(rect: RectTuple, width: f32) -> RectTuple {
    let (x, y, w, h) = rect;
    if width > w {
        // just return the original if its too small
        return (x, y, w, h)
    }

    let left = (w - width) / 2.0;

    (x + left, y, width, h)
}

/// Implements helpful functions for making layouts like splitting, margin etc
pub trait RectLayout: Sized {
    /// Splits at amount height-wise, returning both the slices
    fn vertical(self, amount: Amount) -> (Self, Self);

    /// Splits at amount width-wise, returning both the slices
    fn horizontal(self, amount: Amount) -> (Self, Self);

    /// Add margin (in pixels) to all sides
    fn margin_all(self, up: f32, down: f32, left: f32, right: f32) -> Self;

    /// Add margin to whole rect
    fn margin(self, amount: Amount) -> Self;

    /// Center height-wise vertically, if rect is too small then just returns itself
    fn center_v(self, height: f32) -> Self;

    /// Center width-wise vertically, if rect is too small then just returns itself
    fn center_h(self, width: f32) -> Self;

    /// Center a rect
    fn centered(self, width: f32, height: f32) -> Self { self.center_h(width).center_v(height) }
}

/// Macro to implement rect layout for simple structs
#[macro_export]
macro_rules! impl_rect_layout {
    ($type:ty, $x:tt, $y:tt, $w:tt, $h:tt $(, $num_type:ty)?) => {
        impl RectLayout for $type {
            fn vertical(self, amount: Amount) -> (Self, Self) {
                let (rect1, rect2) = $crate::gui::vertical((self.$x as f32, self.$y as f32, self.$w as f32, self.$h as f32), amount);

                (
                    Self {
                        $x: rect1.0 $(as $num_type)?,
                        $y: rect1.1 $(as $num_type)?,
                        $w: rect1.2 $(as $num_type)?,
                        $h: rect1.3 $(as $num_type)?,
                    },
                    Self {
                        $x: rect2.0 $(as $num_type)?,
                        $y: rect2.1 $(as $num_type)?,
                        $w: rect2.2 $(as $num_type)?,
                        $h: rect2.3 $(as $num_type)?,
                    },
                )
            }

            fn horizontal(self, amount: Amount) -> (Self, Self) {
                let (rect1, rect2) = $crate::gui::horizontal((self.$x as f32, self.$y as f32, self.$w as f32, self.$h as f32), amount);

                (
                    Self {
                        $x: rect1.0 $(as $num_type)?,
                        $y: rect1.1 $(as $num_type)?,
                        $w: rect1.2 $(as $num_type)?,
                        $h: rect1.3 $(as $num_type)?,
                    },
                    Self {
                        $x: rect2.0 $(as $num_type)?,
                        $y: rect2.1 $(as $num_type)?,
                        $w: rect2.2 $(as $num_type)?,
                        $h: rect2.3 $(as $num_type)?,
                    },
                )
            }

            fn margin_all(self, up: f32, down: f32, left: f32, right: f32) -> Self {
                let ($x, $y, $w, $h) = $crate::gui::margin_all((self.$x as f32, self.$y as f32, self.$w as f32, self.$h as f32), up, down, left, right);

                Self {
                    $x: $x $(as $num_type)?,
                    $y: $y $(as $num_type)?,
                    $w: $w $(as $num_type)?,
                    $h: $h $(as $num_type)?,
                }
            }

            fn margin(self, amount: Amount) -> Self {
                let ($x, $y, $w, $h) = $crate::gui::margin((self.$x as f32, self.$y as f32, self.$w as f32, self.$h as f32), amount);

                Self {
                    $x: $x $(as $num_type)?,
                    $y: $y $(as $num_type)?,
                    $w: $w $(as $num_type)?,
                    $h: $h $(as $num_type)?,
                }
            }

            fn center_v(self, height: f32) -> Self {
                let ($x, $y, $w, $h) = $crate::gui::center_v((self.$x as f32, self.$y as f32, self.$w as f32, self.$h as f32), height);

                Self {
                    $x: $x $(as $num_type)?,
                    $y: $y $(as $num_type)?,
                    $w: $w $(as $num_type)?,
                    $h: $h $(as $num_type)?,
                }
            }

            fn center_h(self, width: f32) -> Self {
                let ($x, $y, $w, $h) = $crate::gui::center_h((self.$x as f32, self.$y as f32, self.$w as f32, self.$h as f32), width);

                Self {
                    $x: $x $(as $num_type)?,
                    $y: $y $(as $num_type)?,
                    $w: $w $(as $num_type)?,
                    $h: $h $(as $num_type)?,
                }
            }
        }
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
}

impl_rect_layout!(Rect, x, y, width, height);

/// Helper to hold positions and which element they belong to
#[derive(Default)]
pub struct ClickMap {
    width: u16,
    height: u16,
    points: HashMap<u32, String>,
}

impl ClickMap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Resize the map
    pub fn resize(&mut self, width: u16, height: u16) {
        self.clear();
        self.width = width;
        self.height = height;
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.points.clear();
    }

    /// Set id for a specific point
    pub fn set(&mut self, width: u16, height: u16, id: String) {
        self.points.insert((width + height * self.width).into(), id);
    }

    /// Get id for a specific point
    pub fn get(&self, x: u16, y: u16) -> Option<&str> {
        self.points.get(&Into::<u32>::into(x + y * self.width)).map(|x| x.as_str())
    }
}

// pub trait Element {
//     fn render(renderer: *mut SDL_Renderer, rect: SDL_FRect);
// }

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use super::{Rect, Amount, RectLayout};

    #[allow(unused)]
    struct RectI32 {
        pub x: i32,
        pub y: i32,
        pub w: i32,
        pub h: i32,
    }

    // make sure the macro works properly for non-float structs
    impl_rect_layout!(RectI32, x, y, w, h, i32);

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
            Rect::new(0.0, 0.0, 200.0, 100.0).centered(100., 50.),
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
