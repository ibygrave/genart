use std::path::PathBuf;

use anyhow::{Context, Result};

pub struct Field {
    filename: PathBuf,
    surface: cairo::ImageSurface,
    pub ctx: cairo::Context,
}

impl Field {
    pub fn new(args: &SubArgs) -> Result<Self> {
        let surface = cairo::ImageSurface::create(cairo::Format::Rgb24, args.size.0, args.size.1)?;
        let ctx = cairo::Context::new(&surface)?;
        Ok(Self {
            filename: args.output.clone(),
            surface,
            ctx,
        })
    }

    pub fn save(&self) -> Result<()> {
        let mut file = std::fs::File::create(&self.filename)?;
        self.surface.write_to_png(&mut file)?;
        Ok(())
    }
}

impl Drop for Field {
    fn drop(&mut self) {
        let _ = self.save();
    }
}

#[derive(Clone, Debug)]
pub struct Size(pub i32, pub i32);

fn parse_size(arg: &str) -> Result<Size> {
    let (w, h) = arg.split_once('x').context("No 'x' in size")?;
    Ok(Size(w.parse()?, h.parse()?))
}

#[derive(clap::Args, Debug)]
pub struct SubArgs {
    /// Output file
    #[arg(value_name = "OUTPUT", default_value = "default_decor.png")]
    output: PathBuf,

    #[arg(short, long, value_parser = parse_size, default_value = "1024x1024")]
    pub size: Size,
}
