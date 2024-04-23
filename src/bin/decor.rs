use anyhow::Result;
use clap::Parser;

#[path = "../field.rs"]
mod field;

#[path = "../region.rs"]
mod region;

#[path = "../colour.rs"]
mod colour;

use crate::colour::Colours;
use crate::field::Field;
use crate::region::Region;

/// Sub-divided tiled gradient
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[command(flatten)]
    field: crate::field::SubArgs,

    #[arg(short, long, default_value_t = 10)]
    depth: u8,

    #[arg(short, long, default_value_t = 2)]
    border: i32,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let field = Field::new(&args.field)?;
    let colours = Colours::random();
    let region = Region::new(&args.field, args.border);
    let ctx = cairo::Context::new(&field.surface)?;
    region.render(&ctx, &colours, args.depth)?;
    Ok(())
}
