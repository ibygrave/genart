use std::path::PathBuf;

use anyhow::{Context, Result};

pub struct Field {
    filename: PathBuf,
    pub surface: cairo::ImageSurface,
}

impl Field {
    pub fn new(args: &SubArgs) -> Result<Self> {
        let surface = cairo::ImageSurface::create(cairo::Format::Rgb24, args.size.0, args.size.1)?;
        Ok(Self {
            filename: args.output.clone(),
            surface,
        })
    }

    pub fn save(&self) -> Result<()> {
        let mut file = std::fs::File::create(&self.filename)?;
        self.surface.write_to_png(&mut file)?;
        Ok(())
    }

    pub fn inc(&mut self, x: i32, y: i32) -> Result<()> {
        if x < 0 || x >= self.surface.width() || y < 0 || y >= self.surface.height() {
            return Ok(());
        }
        let pixel_ix = self.surface.stride() * y + 4 * x;
        let mut data = self.surface.data()?;
        for channel in [2, 1, 0] {
            let subpixel_ix = usize::try_from(pixel_ix + channel)?;
            if data[subpixel_ix] == 255 {
                continue;
            }
            data[subpixel_ix] += 1;
            break;
        }
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
    #[arg(value_name = "OUTPUT", default_value = "default.png")]
    output: PathBuf,

    #[arg(short, long, value_parser = parse_size, default_value = "1024x1024")]
    pub size: Size,
}
