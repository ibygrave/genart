#![allow(clippy::cast_precision_loss)]

use std::collections::VecDeque;

use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

#[derive(Clone, Copy, Debug)]
pub struct Colour {
    r: f64,
    g: f64,
    b: f64,
}

impl Colour {
    const BLACK: Self = Self {
        r: 0f64,
        g: 0f64,
        b: 0f64,
    };
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let dist = Uniform::new(0f64, 1f64);
        Self {
            r: dist.sample(&mut rng),
            g: dist.sample(&mut rng),
            b: dist.sample(&mut rng),
        }
    }

    fn scale(&self, scale: f64) -> Self {
        Self {
            r: self.r * scale,
            g: self.g * scale,
            b: self.b * scale,
        }
    }

    fn add(self, other: Colour) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }

    pub fn mix(colours: &VecDeque<Colour>) -> Self {
        let mut rng = rand::thread_rng();
        let dist = Uniform::new(0f64, 1f64);
        let quants = colours
            .iter()
            .map(|_| dist.sample(&mut rng))
            .collect::<Vec<_>>();
        let scale = 1f64 / quants.iter().sum::<f64>();
        colours
            .iter()
            .zip(quants)
            .map(|(c, q)| c.scale(q * scale))
            .fold(Colour::BLACK, Colour::add)
    }
}

pub struct Colours(VecDeque<Colour>);

impl Colours {
    pub fn random() -> Self {
        Self(VecDeque::from([
            Colour::random(),
            Colour::random(),
            Colour::random(),
            Colour::random(),
            Colour::random(),
        ]))
    }

    pub fn split(&self) -> (Self, Self) {
        let mut left = self.0.clone();
        let mut right = self.0.clone();

        if self.0.len() >= 5 {
            left.pop_back();
            right.pop_front();
        }

        // enhance the two new colour lists with random new colours
        let mut rng = rand::thread_rng();
        left.insert(rng.gen_range(0..left.len()), Colour::mix(&left));
        right.insert(rng.gen_range(0..right.len()), Colour::mix(&right));
        (Self(left), Self(right))
    }

    pub fn add_gradient_stops(&self, gradient: &cairo::LinearGradient) {
        let max_ix = (self.0.len() - 1) as f64;
        for (ix, colour) in self.0.iter().enumerate() {
            let offset = (ix as f64) / max_ix;
            gradient.add_color_stop_rgb(offset, colour.r, colour.g, colour.b);
        }
    }
}
