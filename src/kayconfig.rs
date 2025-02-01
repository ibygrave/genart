use std::{
    cmp::{max, min},
    fmt::Display,
    str::FromStr,
};

use anyhow::Result;

#[derive(Clone, Debug)]
pub enum PixelCalc {
    Min,
    Max,
    Av,
}

impl PixelCalc {
    pub fn calc(&self, x: u8, y: u8) -> u8 {
        match self {
            PixelCalc::Min => min(x, y),
            PixelCalc::Max => max(x, y),
            PixelCalc::Av => (x + y) / 2,
        }
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
        };
        write!(f, "{c}")
    }
}

#[derive(Clone, Debug)]
pub struct KayConfig {
    pub calc_r: PixelCalc,
    pub calc_g: PixelCalc,
    pub calc_b: PixelCalc,
}

impl Default for KayConfig {
    fn default() -> Self {
        Self {
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
            calc_r: PixelCalc::parse_next(&mut chars)?,
            calc_g: PixelCalc::parse_next(&mut chars)?,
            calc_b: PixelCalc::parse_next(&mut chars)?,
        })
    }
}

impl Display for KayConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.calc_r, self.calc_g, self.calc_b)
    }
}
