use sdl3_sys::render::SDL_Renderer;

pub use sdl3_sys::rect::{SDL_FRect, SDL_Rect};

// TODO implement minus operator
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Amount {
    /// Percentage as a float, 50.0 is equal to 50%
    Percentage(f32),

    /// Fraction as in 1/x
    Fraction(f32),

    /// Size in pixels
    Pixel(f32),
}

impl Amount {
    fn calculate(&self, value: f32) -> (f32, f32) {
        let (neg, x) = match self {
            Self::Percentage(x) => {
                let y = value * (x.abs() / 100.0);

                (x.is_sign_negative(), y)

                // // flip the output if the input is negative
                // if x.is_sign_negative() {
                //     (value - y, y)
                // } else {
                //     (y, value - y)
                // }
            },
            Self::Fraction(x) => {
                let y = value * (100.0 / x.abs() / 100.0);

                (x.is_sign_negative(), y)
                // if x.is_sign_negative() {
                //     (value - y, y)
                // } else {
                //     (value - y, y)
                // }
            },
            Self::Pixel(x) => {
                // let neg = x.is_sign_negative();
                // let x = x.abs();

                (x.is_sign_negative(), x.abs())
                // // flip the output if the input is negative
                // if neg {
                //     (value - x, x)
                // } else {
                //     (x, value - x)
                // }
            }
        };

        if neg {
            (value - x, x)
        } else {
            (x, value - x)
        }
    }
    //
    // pub fn abs(self) -> Self {
    //     match self {
    //         Self::Percentage(x) => Self::Percentage(x.abs()),
    //         Self::Pixel(x) => Self::Pixel(x.abs()),
    //     }
    // }
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

    // TODO rename to vert, horiz ?
    /// Reserves certain amount vertically
    pub fn vertical(self, amount: Amount) -> (Self, Self) {
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
    pub fn horizontal(self, amount: Amount) -> (Self, Self) {
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
    pub fn halve_v(self) -> (Self, Self) { self.vertical(50.0.into()) }

    /// Divide horizontally in two
    pub fn halve_h(self) -> (Self, Self) { self.horizontal(50.0.into()) }

    // TODO remove once the Amount::Fraction is done
    /// Divide vertically by a fraction 1/x
    pub fn fraction_v(self, div: f32) -> (Self, Self) { self.vertical((100.0 / div).into()) }

    /// Divide horizontally by a fraction 1/x
    pub fn fraction_h(self, div: f32) -> (Self, Self) { self.horizontal((100.0 / div).into()) }

    /// Add margin to the rect but each margin can be different
    pub fn margin_all(self, up: f32, down: f32, left: f32, right: f32) -> Self {
        let up = up.abs();
        let down = down.abs();
        let left = left.abs();
        let right = right.abs();

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

    /// Wrapper around rect that can be compared without rounding errors
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
    fn test_rect() {
        rect_tuple_eq!(
            Rect::new(0.0, 0.0, 200.0, 100.0).halve_v(),
            (
                Rect::new(0.0, 0.0, 200.0, 50.0),
                Rect::new(0.0, 50.0, 200.0, 50.0),
            )
        );

        rect_tuple_eq!(
            Rect::new(0.0, 0.0, 200.0, 100.0).halve_h(),
            (
                Rect::new(0.0, 0.0, 100.0, 100.0),
                Rect::new(100.0, 00.0, 100.0, 100.0),
            )
        );

        rect_tuple_eq!(
            Rect::new(0.0, 0.0, 200.0, 100.0).fraction_v(3.0),
            (
                Rect::new(0.0, 0.0, 200.0, 33.3333),
                Rect::new(0.0, 33.3333, 200.0, 66.6666),
            )
        );

        rect_tuple_eq!(
            Rect::new(0.0, 0.0, 200.0, 100.0).fraction_v(-3.0),
            (
                Rect::new(0.0, 0.0, 200.0, 66.6666),
                Rect::new(0.0, 66.6666, 200.0, 33.33333),
            )
        );

        // test reserving with pixels (both directions)
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
                Rect::new(0.0, 0.0, 200.0, 70.0),
                Rect::new(0.0, 70.0, 200.0, 30.0),
            )
        );

        // now horizontal
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
                Rect::new(0.0, 0.0, 150.0, 100.0),
                Rect::new(150.0, 0.0, 50.0, 100.0),
            )
        );

        // TODO
        let rect = Rect::new(0., 0., 800., 600.);
        let (toolbar, rect) = rect.vertical(Amount::Pixel(30.));
        // dbg!(&rect);
        let (sidebar, rect) = rect.horizontal(Amount::Pixel(60.));
        dbg!(&rect);
        // let content = rect.margin(Amount::Pixel(10.));
//  p{ x: 0.0, y: 30.0, width: 740.0, height: 570.0 })
        rect_eq!(toolbar, Rect::new(0., 30., 740., 570.));
        rect_eq!(sidebar, Rect::new(0., 0., 0., 0.));
        // rect_eq!(content, Rect::new(0., 0., 0., 0.));
        //
        // // TODO i cannot add sidebar to the right side?
        // assert_eq!(toolbar, Rect::new(0., 0., 800., 30.));
        // assert_eq!(sidebar, Rect::new(770.0, 0., 60., 570.));
        // assert_eq!(content, Rect::new(0., 0., 60., 570.));
    }
}
