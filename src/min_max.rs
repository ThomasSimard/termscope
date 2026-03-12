pub struct MinMax {
    minimum: f64,
    maximum: f64
}

impl Default for MinMax {
    fn default() -> Self {
        Self { minimum: f64::MAX, maximum: f64::MIN }
    }
}

impl MinMax {
    pub fn update(&mut self, value: f64) {
        if self.minimum > value {
            self.minimum = value
        }

        if self.maximum < value {
            self.maximum = value
        }
    }

    pub fn get_minimum(&self) -> f64 {
        self.minimum
    }

    pub fn get_maximum(&self) -> f64 {
        self.maximum
    }
}
