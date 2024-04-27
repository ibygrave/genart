use rand::Rng;

use crate::Colours;
use anyhow::Result;

#[derive(Copy, Clone)]
struct Range {
    min: i32,
    max: i32,
    size: i32,
    border: i32,
}

impl Range {
    fn new(min: i32, max: i32, border: i32) -> Self {
        let size = max - min;
        Self {
            min,
            max,
            size,
            border,
        }
    }

    fn inner(&self) -> Self {
        Self::new(self.min + self.border, self.max - self.border, self.border)
    }

    fn is_splittable(&self) -> bool {
        self.size > 2 * (self.border + 1)
    }

    fn split(&self, at: i32) -> (Self, Self) {
        (
            Self::new(self.min, self.min + at, self.border),
            Self::new(self.min + at, self.max, self.border),
        )
    }
}

pub struct Region {
    x: Range,
    y: Range,
    border: i32,
}

impl Region {
    pub fn new(args: &crate::field::SubArgs, border: i32) -> Self {
        Self {
            x: Range::new(0, args.size.0, border),
            y: Range::new(0, args.size.1, border),
            border,
        }
    }

    fn is_splittable(&self) -> bool {
        self.x.is_splittable() && self.y.is_splittable()
    }

    fn split(&self) -> (Self, Self) {
        assert!(self.is_splittable());

        // Split the rectangle
        let x_inner = self.x.inner();
        let y_inner = self.y.inner();
        let perim = x_inner.size + y_inner.size;
        let mut rng = rand::thread_rng();
        let split_point = rng.gen_range(0..perim);
        if split_point < x_inner.size {
            // Horizontal split
            let (low_x, high_x) = x_inner.split(split_point);
            (
                Self {
                    x: low_x,
                    y: y_inner,
                    border: self.border,
                },
                Self {
                    x: high_x,
                    y: y_inner,
                    border: self.border,
                },
            )
        } else {
            // Vertical split
            let (low_y, high_y) = y_inner.split(split_point - x_inner.size);
            (
                Self {
                    x: x_inner,
                    y: low_y,
                    border: self.border,
                },
                Self {
                    x: x_inner,
                    y: high_y,
                    border: self.border,
                },
            )
        }
    }

    fn get_gradient(&self) -> cairo::LinearGradient {
        cairo::LinearGradient::new(
            f64::from(self.x.min),
            f64::from(self.y.min),
            f64::from(self.x.max),
            f64::from(self.y.max),
        )
    }

    pub fn render(&self, ctx: &cairo::Context, colours: &Colours, depth: u8) -> Result<()> {
        let grad = self.get_gradient();
        colours.add_gradient_stops(&grad);
        ctx.set_source(&grad)?;
        ctx.rectangle(
            f64::from(self.x.min),
            f64::from(self.y.min),
            f64::from(self.x.size),
            f64::from(self.y.size),
        );
        ctx.fill()?;
        if (depth > 0) && self.is_splittable() {
            let (rl, rr) = self.split();
            let (cl, cr) = colours.split();
            rl.render(ctx, &cl, depth - 1)?;
            rr.render(ctx, &cr, depth - 1)?;
        }
        Ok(())
    }
}
