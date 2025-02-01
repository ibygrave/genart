use std::{cmp::min, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use image::{ImageBuffer, Rgb};

/// Image processing
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Input file
    #[arg(value_name = "INPUT")]
    input: PathBuf,

    /// Output file
    #[arg(value_name = "OUTPUT")]
    output: PathBuf,
}

fn pixel_average<'a>(pixels: impl Iterator<Item = &'a Rgb<u8>>) -> Rgb<u8> {
    let mut r = 0u64;
    let mut g = 0u64;
    let mut b = 0u64;
    let mut count = 0u64;
    for p in pixels {
        r += u64::from(p.0[0]);
        g += u64::from(p.0[1]);
        b += u64::from(p.0[2]);
        count += 1;
    }
    r /= count;
    g /= count;
    b /= count;
    Rgb([
        u8::try_from(r).unwrap(),
        u8::try_from(g).unwrap(),
        u8::try_from(b).unwrap(),
    ])
}

struct ImageScans {
    x: Vec<Rgb<u8>>,
    y: Vec<Rgb<u8>>,
}

trait RowColumn {
    fn row(&self, y: u32) -> impl Iterator<Item = &Rgb<u8>>;
    fn column(&self, x: u32) -> impl Iterator<Item = &Rgb<u8>>;
}

impl RowColumn for ImageBuffer<Rgb<u8>, Vec<u8>> {
    fn row(&self, y: u32) -> impl Iterator<Item = &Rgb<u8>> {
        let (xsize, _) = self.dimensions();
        (0..xsize).map(move |x| self.get_pixel(x, y))
    }

    fn column(&self, x: u32) -> impl Iterator<Item = &Rgb<u8>> {
        let (_, ysize) = self.dimensions();
        (0..ysize).map(move |y| self.get_pixel(x, y))
    }
}

impl ImageScans {
    fn new(img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Self {
        let (xsize, ysize) = img.dimensions();
        Self {
            x: (0..xsize).map(|x| pixel_average(img.column(x))).collect(),
            y: (0..ysize).map(|y| pixel_average(img.row(y))).collect(),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut img = image::open(args.input)?;
    let pixels = img.as_mut_rgb8().unwrap();
    let scans = ImageScans::new(pixels);
    for (x, y, pixel) in pixels.enumerate_pixels_mut() {
        let r = min(scans.x[x as usize].0[0], scans.y[y as usize].0[0]);
        let g = min(scans.x[x as usize].0[1], scans.y[y as usize].0[1]);
        let b = min(scans.x[x as usize].0[2], scans.y[y as usize].0[2]);
        *pixel = Rgb([r, g, b]);
    }
    img.save(args.output)?;

    Ok(())
}
