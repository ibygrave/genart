use std::ops::Range;

use anyhow::Result;
use rand::Rng;

use crate::field::Field;

pub struct Boid {
    x: i32,
    y: i32,
    follow: usize,
    flee: usize,
}

pub struct Boids {
    boids: Vec<Boid>,
    width: Range<i32>,
    height: Range<i32>,
}

impl Boids {
    pub fn new(nboids: usize, width: Range<i32>, height: Range<i32>) -> Self {
        let mut rng = rand::thread_rng();
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
                        x: rng.gen_range(width.clone()),
                        y: rng.gen_range(height.clone()),
                        follow,
                        flee,
                    }
                })
                .collect::<Vec<_>>(),

            width,
            height,
        }
    }

    pub fn imprint(&self, field: &mut Field) -> Result<()> {
        for boid in &self.boids {
            field.inc(boid.x, boid.y)?;
        }
        Ok(())
    }
}
