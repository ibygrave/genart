use std::ops::Range;

use anyhow::Result;
use rand::{distributions::Distribution, Rng};

use crate::field::Field;

pub struct Boid {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    follow: usize,
    flee: usize,
}

impl Boid {
    fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
}

pub struct Boids {
    boids: Vec<Boid>,
    width: i32,
    height: i32,
}

impl Boids {
    pub fn new(nboids: usize, width: i32, height: i32) -> Self {
        #![allow(clippy::cast_lossless)]
        let mut rng = rand::thread_rng();
        let x_range = rand::distributions::Uniform::new(0f64, width as f64);
        let y_range = rand::distributions::Uniform::new(0f64, height as f64);
        Self {
            boids: (0..nboids)
                .map(|me| {
                    let follow = loop {
                        let b = rng.gen_range(0..nboids);
                        if b != me {
                            break b;
                        }
                    };
                    let flee = loop {
                        let b = rng.gen_range(0..nboids);
                        if b != me && b != follow {
                            break b;
                        }
                    };
                    Boid {
                        x: x_range.sample(&mut rng),
                        y: y_range.sample(&mut rng),
                        dx: rng.gen_range(-1f64..=1f64),
                        dy: rng.gen_range(-1f64..=1f64),
                        follow,
                        flee,
                    }
                })
                .collect::<Vec<_>>(),

            width,
            height,
        }
    }

    pub fn update(&mut self) {
        self.boids.iter_mut().for_each(Boid::update);
    }

    pub fn imprint(&self, field: &mut Field) -> Result<()> {
        for boid in &self.boids {
            #[allow(clippy::cast_possible_truncation)]
            field.inc(boid.x as i32, boid.y as i32)?;
        }
        Ok(())
    }
}
