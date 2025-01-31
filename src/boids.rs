use std::{borrow::BorrowMut, ops::Range};

use anyhow::Result;
use rand::{distr::Distribution, Rng};

use crate::field::Field;

#[derive(Clone, Copy)]
pub struct Boid {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    follow: usize,
    flee: usize,
}

impl Boid {
    fn move_to(&mut self, other: &Boid, scale: f64) {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let h = (dx * dx + dy * dy).sqrt();
        self.dx += scale * (dx / h);
        self.dy += scale * (dy / h);
    }

    fn update(&mut self, x_range: Range<f64>, y_range: Range<f64>) {
        self.x += self.dx;
        self.y += self.dy;
        if self.x > x_range.end {
            self.x = x_range.start;
        } else if self.x < x_range.start {
            self.x = x_range.end;
        }
        if self.y > y_range.end {
            self.y = y_range.start;
        } else if self.y < y_range.start {
            self.y = y_range.end;
        }
    }
}

pub struct Flock {
    boids: Vec<Boid>,
    width: i32,
    height: i32,
}

impl Flock {
    pub fn new(nboids: usize, width: i32, height: i32) -> Result<Self> {
        #![allow(clippy::cast_lossless)]
        let mut rng = rand::rng();
        let x_range = rand::distr::Uniform::new(0f64, width as f64)?;
        let y_range = rand::distr::Uniform::new(0f64, height as f64)?;
        Ok(Self {
            boids: (0..nboids)
                .map(|me| {
                    let follow = loop {
                        let b = rng.random_range(0..nboids);
                        if b != me {
                            break b;
                        }
                    };
                    let flee = loop {
                        let b = rng.random_range(0..nboids);
                        if b != me && b != follow {
                            break b;
                        }
                    };
                    Boid {
                        x: x_range.sample(&mut rng),
                        y: y_range.sample(&mut rng),
                        dx: rng.random_range(-1f64..=1f64),
                        dy: rng.random_range(-1f64..=1f64),
                        follow,
                        flee,
                    }
                })
                .collect::<Vec<_>>(),

            width,
            height,
        })
    }

    pub fn update(&mut self) {
        (0..self.boids.len()).for_each(|ix| {
            let follow = self.boids[self.boids[ix].follow];
            let flee = self.boids[self.boids[ix].flee];
            let boid = self.boids[ix].borrow_mut();
            boid.dx *= 0.95;
            boid.dy *= 0.95;
            boid.move_to(&follow, 0.05);
            boid.move_to(&flee, -0.05);
        });
        self.boids.iter_mut().for_each(|boid| {
            boid.update(0.0..(f64::from(self.width)), 0.0..(f64::from(self.height)));
        });
    }

    pub fn imprint(&self, field: &mut Field) -> Result<()> {
        for boid in &self.boids {
            #[allow(clippy::cast_possible_truncation)]
            field.inc(boid.x as i32, boid.y as i32)?;
        }
        Ok(())
    }
}
