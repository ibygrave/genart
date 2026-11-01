use anyhow::Result;
use clap::Parser;

use genart::{Colour, Field, FieldArgs};
use rand::distr::Distribution;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[command(flatten)]
    field: FieldArgs,

    #[arg(short, long, default_value_t = 100)]
    reeds: usize,
}

struct Reed {
    x: f64,
    dx: f64,
    colour: Colour,
}

impl Reed {
    fn step(&mut self, width: f64) {
        self.x += self.dx;
        if self.x < 0f64 {
            self.x += width;
        }
        if self.x > width {
            self.x -= width;
        }
    }

    fn grad_to(&self, other: &Self) -> cairo::LinearGradient {
        let grad = cairo::LinearGradient::new(self.x, 0.0, other.x, 0.0);
        self.colour.add_stop(&grad, 0.0);
        Colour::BLACK.add_stop(&grad, 0.5);
        other.colour.add_stop(&grad, 1.0);
        grad
    }

    fn line_to(&self, other: &Self, ctx: &cairo::Context, y: f64) -> Result<()> {
        let grad = self.grad_to(other);
        ctx.set_source(&grad)?;
        ctx.rectangle(self.x, y, other.x - self.x, 1.0);
        ctx.fill()?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let field = Field::new(&args.field)?;
    let ctx = cairo::Context::new(&field.surface)?;
    let mut reeds = Vec::new();
    let mut rng = rand::rng();
    let width = f64::from(field.surface.width());
    let dx_range = rand::distr::Uniform::new(-0.5f64, 0.5f64)?;
    let x_range = rand::distr::Uniform::new(0f64, width)?;

    for _ in 0..args.reeds {
        reeds.push(Reed {
            x: x_range.sample(&mut rng),
            dx: dx_range.sample(&mut rng),
            colour: Colour::random()?,
        });
    }
    for y in 0..field.surface.height() {
        reeds.sort_by(|left, right| left.x.total_cmp(&right.x));
        let mut reed_iter = reeds.iter();
        let mut reed_left = reed_iter.next().unwrap();
        for reed_right in reed_iter {
            reed_left.line_to(reed_right, &ctx, f64::from(y))?;
            reed_left = reed_right;
        }

        for reed in &mut reeds {
            reed.step(width);
        }
    }
    Ok(())
}
