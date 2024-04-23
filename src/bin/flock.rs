use anyhow::Result;
use clap::Parser;

#[path = "../boids.rs"]
mod boids;

#[path = "../field.rs"]
mod field;

use crate::boids::Boids;
use crate::field::Field;

/// Heat-map of flocking boids
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[command(flatten)]
    field: crate::field::SubArgs,

    #[arg(short, long, default_value_t = 100_000)]
    boids: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut field = Field::new(&args.field)?;
    let boids = Boids::new(
        args.boids,
        0..field.surface.width(),
        0..field.surface.height(),
    );
    for _ in 0..500 {
        boids.imprint(&mut field)?;
    }
    Ok(())
}
