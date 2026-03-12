use core::f64;

pub struct MinMax {
    minimum: f64,
    maximum: f64
}

impl Default for MinMax {
    fn default() -> Self {
        Self { minimum: f64::INFINITY, maximum: f64::NEG_INFINITY}
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

#[cfg(test)]
mod tests {
    use core::f64;

    use super::*;

    #[test]
    fn edge_case_update_infinity() {
        let mut min_max = MinMax::default();

        min_max.update(f64::INFINITY);

        assert_eq!(min_max.get_minimum(), f64::INFINITY);
        assert_eq!(min_max.get_maximum(), f64::INFINITY);
    }

    #[test]
    fn edge_case_update_negative_infinity() {
        let mut min_max = MinMax::default();

        min_max.update(f64::NEG_INFINITY);

        assert_eq!(min_max.get_minimum(), f64::NEG_INFINITY);
        assert_eq!(min_max.get_maximum(), f64::NEG_INFINITY);
    }

    #[test]
    fn edge_case_update_nan() {
        let mut min_max = MinMax::default();

        min_max.update(f64::NAN);

        assert_eq!(min_max.get_minimum(), f64::INFINITY);
        assert_eq!(min_max.get_maximum(), f64::NEG_INFINITY);
    }
}
