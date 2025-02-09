use std::{
    cmp::{max, min},
    fmt::Display,
    str::FromStr,
};

use anyhow::Result;
use image::{Pixel, Rgb};

#[derive(Clone, Debug)]
pub enum PixelCalc {
    Min,
    Max,
    Av,
    Diff,
    Zero,
}

impl PixelCalc {
    pub fn calc(&self, x: u8, y: u8) -> u8 {
        match self {
            PixelCalc::Min => min(x, y),
            PixelCalc::Max => max(x, y),
            PixelCalc::Av => (x + y) / 2,
            PixelCalc::Diff => {
                if x > y {
                    x - y
                } else {
                    y - x
                }
            }
            PixelCalc::Zero => 0,
        }
    }
}

trait PixelFold {
    fn channel_fold(acc: u64, p: u8) -> u64;
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn pixel_fold(acc: (Rgb<u64>, u64), p: &Rgb<u8>) -> (Rgb<u64>, u64) {
        let mut ans = Rgb([0u64; 3]);
        for s in 0..3 {
            ans[s] = Self::channel_fold(acc.0[s], p[s]);
        }
        (ans, acc.1 + 1)
    }
}

struct MinPixelFold;
struct MaxPixelFold;
struct AvPixelFold;

impl PixelFold for MinPixelFold {
    fn channel_fold(acc: u64, p: u8) -> u64 {
        min(acc, u64::from(p))
    }
}

impl PixelFold for MaxPixelFold {
    fn channel_fold(acc: u64, p: u8) -> u64 {
        max(acc, u64::from(p))
    }
}

impl PixelFold for AvPixelFold {
    fn channel_fold(acc: u64, p: u8) -> u64 {
        acc + u64::from(p)
    }
}

impl PixelCalc {
    pub fn scan<'a>(&self, pixels: impl Iterator<Item = &'a Rgb<u8>>) -> Rgb<u8> {
        let init = (Rgb([0; 3]), 0);
        let ans = match self {
            PixelCalc::Min => pixels.fold(init, MinPixelFold::pixel_fold).0,
            PixelCalc::Max => pixels.fold(init, MaxPixelFold::pixel_fold).0,
            PixelCalc::Av => {
                let (sum, count) = pixels.fold(init, AvPixelFold::pixel_fold);
                sum.map(|chan| chan / count)
            }
            PixelCalc::Diff => todo!(),
            PixelCalc::Zero => Rgb([0; 3]),
        };
        Rgb([
            u8::try_from(ans[0]).unwrap(),
            u8::try_from(ans[1]).unwrap(),
            u8::try_from(ans[2]).unwrap(),
        ])
    }

    fn parse_next(chars: &mut impl Iterator<Item = char>) -> Result<Self, String> {
        chars.next().ok_or("err")?.try_into()
    }
}

impl TryFrom<char> for PixelCalc {
    type Error = String;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'n' => Ok(Self::Min),
            'x' => Ok(Self::Max),
            'a' => Ok(Self::Av),
            'd' => Ok(Self::Diff),
            'z' => Ok(Self::Zero),
            _ => Err("Invalid".into()),
        }
    }
}

impl Display for PixelCalc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            PixelCalc::Min => 'n',
            PixelCalc::Max => 'x',
            PixelCalc::Av => 'a',
            PixelCalc::Diff => 'd',
            PixelCalc::Zero => 'z',
        };
        write!(f, "{c}")
    }
}

#[derive(Clone, Debug)]
pub struct KayConfig {
    pub scan_x: PixelCalc,
    pub scan_y: PixelCalc,
    pub calc_r: PixelCalc,
    pub calc_g: PixelCalc,
    pub calc_b: PixelCalc,
}

impl Default for KayConfig {
    fn default() -> Self {
        Self {
            scan_x: PixelCalc::Av,
            scan_y: PixelCalc::Av,
            calc_r: PixelCalc::Min,
            calc_g: PixelCalc::Min,
            calc_b: PixelCalc::Min,
        }
    }
}

impl FromStr for KayConfig {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut chars = s.chars();
        Ok(Self {
            scan_x: PixelCalc::parse_next(&mut chars)?,
            scan_y: PixelCalc::parse_next(&mut chars)?,
            calc_r: PixelCalc::parse_next(&mut chars)?,
            calc_g: PixelCalc::parse_next(&mut chars)?,
            calc_b: PixelCalc::parse_next(&mut chars)?,
        })
    }
}

impl Display for KayConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.scan_x, self.scan_y, self.calc_r, self.calc_g, self.calc_b
        )
    }
}
