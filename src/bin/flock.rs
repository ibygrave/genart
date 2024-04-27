use anyhow::Result;
use clap::Parser;
use indicatif::ProgressBar;

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

    #[arg(short, long, default_value_t = 1_000)]
    move_steps: u64,

    #[arg(short, long, default_value_t = 10_000)]
    draw_steps: u64,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut field = Field::new(&args.field)?;
    let mut boids = Boids::new(args.boids, field.surface.width(), field.surface.height());
    println!("Moving...");
    let pb = ProgressBar::new(args.move_steps);
    for _ in 0..args.move_steps {
        boids.update();
        pb.inc(1);
    }
    pb.finish_and_clear();
    println!("Drawing...");
    let pb = ProgressBar::new(args.draw_steps);
    for _ in 0..args.draw_steps {
        boids.update();
        boids.imprint(&mut field)?;
        pb.inc(1);
    }
    pb.finish_with_message("done");
    Ok(())
}
