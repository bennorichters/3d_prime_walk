use crate::{color_gradient::ColorGradient, primes::Primes, space::{Pixel3D, Tuple3D}};

static DIRS: &[[isize; 3]] = &[
    [1, 0, 0],
    [0, 1, 0],
    [0, 0, 1],
    [-1, 0, 0],
    [0, -1, 0],
    [0, 0, -1],
];

struct DirIterator {
    index: usize,
}

impl Iterator for DirIterator {
    type Item = [isize; 3];

    fn next(&mut self) -> Option<Self::Item> {
        let result = DIRS[self.index];
        self.index += 1;
        if self.index == DIRS.len() {
            self.index = 0;
        }

        Some(result)
    }
}

pub fn walk(steps: usize, mut gradient: ColorGradient) -> Vec<Pixel3D> {
    let mut result = vec![];

    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    let mut dir_it = DirIterator { index: 0 };

    let mut dir = dir_it.next().unwrap();
    let mut primes = Primes::with_limit(steps);
    let mut p = primes.next().unwrap();
    for n in 0..steps {
        if n == (p as usize) {
            dir = dir_it.next().unwrap();
            // Use a value larger than steps if we run out of primes
            p = primes.next().unwrap_or(steps as u64 + 1);
        }

        let color = gradient.next().unwrap();
        result.push(Pixel3D {
            coordinate: Tuple3D {
                x: x as f64,
                y: y as f64,
                z: z as f64,
            },
            color,
        });

        x += dir[0];
        y += dir[1];
        z += dir[2];
    }

    result
}
