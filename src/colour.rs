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
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let dist = Uniform::new(0f64, 1f64);
        Self {
            r: dist.sample(&mut rng),
            g: dist.sample(&mut rng),
            b: dist.sample(&mut rng),
        }
    }

    pub fn mix(colours: &VecDeque<Colour>) -> Self {
        // TODO: a random mixture instead of the average
        let quants = (0..colours.len()).map(|_| 1f64).collect::<Vec<_>>();
        let scale = 1f64 / quants.iter().sum::<f64>();
        colours
            .iter()
            .zip(quants)
            .map(|(c, q)| Colour {
                r: c.r * q * scale,
                g: c.g * q * scale,
                b: c.b * q * scale,
            })
            .fold(
                Colour {
                    r: 0f64,
                    g: 0f64,
                    b: 0f64,
                },
                |c1, c2| Colour {
                    r: c1.r + c2.r,
                    g: c1.g + c2.g,
                    b: c1.b + c2.b,
                },
            )
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
