use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use image::{ImageBuffer, Pixel, Rgb};

use genart::KayConfig;

/// Image processing
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Processing configuration
    #[arg(short, long, default_value_t)]
    config: KayConfig,

    /// Input file
    #[arg(value_name = "INPUT")]
    input: PathBuf,

    /// Output file
    #[arg(value_name = "OUTPUT")]
    output: PathBuf,
}

struct ImageScans {
    x: Vec<Rgb<u8>>,
    y: Vec<Rgb<u8>>,
}

trait RowColumn<P: Pixel> {
    fn row<'a>(&'a self, y: u32) -> impl Iterator<Item = &'a P>
    where
        P: 'a;
    fn column<'a>(&'a self, x: u32) -> impl Iterator<Item = &'a P>
    where
        P: 'a;
}

impl<P: Pixel> RowColumn<P> for ImageBuffer<P, Vec<P::Subpixel>> {
    fn row<'a>(&'a self, y: u32) -> impl Iterator<Item = &'a P>
    where
        P: 'a,
    {
        let (xsize, _) = self.dimensions();
        (0..xsize).map(move |x| self.get_pixel(x, y))
    }

    fn column<'a>(&'a self, x: u32) -> impl Iterator<Item = &'a P>
    where
        P: 'a,
    {
        let (_, ysize) = self.dimensions();
        (0..ysize).map(move |y| self.get_pixel(x, y))
    }
}

impl ImageScans {
    fn new(img: &ImageBuffer<Rgb<u8>, Vec<u8>>, config: &KayConfig) -> Self {
        let (xsize, ysize) = img.dimensions();
        Self {
            x: (0..xsize)
                .map(|x| config.scan_x.scan(img.column(x)))
                .collect(),
            y: (0..ysize).map(|y| config.scan_y.scan(img.row(y))).collect(),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut img = image::open(args.input)?;
    let pixels = img.as_mut_rgb8().unwrap();
    let scans = ImageScans::new(pixels, &args.config);
    for (x, y, pixel) in pixels.enumerate_pixels_mut() {
        let r = args
            .config
            .calc_r
            .calc(scans.x[x as usize][0], scans.y[y as usize][0]);
        let g = args
            .config
            .calc_g
            .calc(scans.x[x as usize][1], scans.y[y as usize][1]);
        let b = args
            .config
            .calc_b
            .calc(scans.x[x as usize][2], scans.y[y as usize][2]);
        *pixel = Rgb([r, g, b]);
    }
    img.save(args.output)?;

    Ok(())
}
