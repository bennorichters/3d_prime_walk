pub struct ColorGradient {
    current_step: usize,
    total_steps: usize,
    start: (f64, f64, f64),
    end: (f64, f64, f64),
}

impl ColorGradient {
    pub fn new(start: (u8, u8, u8), end: (u8, u8, u8), steps: usize) -> Self {
        Self {
            current_step: 0,
            total_steps: steps,
            start: (start.0 as f64, start.1 as f64, start.2 as f64),
            end: (end.0 as f64, end.1 as f64, end.2 as f64),
        }
    }
}

impl Iterator for ColorGradient {
    type Item = (u8, u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_step >= self.total_steps {
            return None;
        }

        let t = if self.total_steps == 1 {
            0.0
        } else {
            self.current_step as f64 / (self.total_steps - 1) as f64
        };

        let r = (self.start.0 + (self.end.0 - self.start.0) * t).round() as u8;
        let g = (self.start.1 + (self.end.1 - self.start.1) * t).round() as u8;
        let b = (self.start.2 + (self.end.2 - self.start.2) * t).round() as u8;

        self.current_step += 1;
        Some((r, g, b))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.total_steps - self.current_step;
        (remaining, Some(remaining))
    }
}
