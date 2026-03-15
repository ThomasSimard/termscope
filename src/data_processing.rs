use crate::DataPoint;
use crate::min_max::MinMax;

#[derive(Default)]
pub struct Processing {
    datasets: Vec<Vec<(f64, f64)>>,

    pub domain: MinMax,
    pub range: MinMax,
}

impl Processing {
    pub fn init(&mut self, number_of_charts: usize) {
        if self.datasets.is_empty() {
            self.datasets = vec![Vec::default(); number_of_charts];
        }
    }

    pub fn process(&mut self, row_of_data: &[f64]) {
        if let Some((&x, ys)) = row_of_data.split_first() {
            self.domain.update(x);

            for &y in ys {
                self.range.update(y);
            }

            for (dataset, &y) in self.datasets.iter_mut().zip(ys.iter()) {
                dataset.push((x, y));
            }
        }
    }

    pub fn get_data(&self) -> &Vec<Vec<DataPoint>> {
        &self.datasets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup1chart() -> Processing {
        let mut processing = Processing::default();

        processing.init(1);

        processing.process(&[0.0, 1.0]);
        processing.process(&[1.0, 2.0]);

        processing
    }

    #[test]
    fn valid_domain_and_range_1_chart() {
        let processing = setup1chart();

        assert_eq!(processing.domain.get_minimum(), 0.0);
        assert_eq!(processing.domain.get_maximum(), 1.0);

        assert_eq!(processing.range.get_minimum(), 1.0);
        assert_eq!(processing.range.get_maximum(), 2.0);
    }

    #[test]
    fn valid_data_1_chart() {
        let processing = setup1chart();

        assert_eq!(processing.get_data(), &[[(0.0, 1.0), (1.0, 2.0)]]);
    }

    fn setup2chart() -> Processing {
        let mut processing = Processing::default();

        processing.init(2);

        processing.process(&[0.0, 1.0, 2.0]);
        processing.process(&[1.0, 2.0, 3.0]);

        processing
    }

    #[test]
    fn valid_domain_and_range_2_chart() {
        let processing = setup2chart();

        assert_eq!(processing.domain.get_minimum(), 0.0);
        assert_eq!(processing.domain.get_maximum(), 1.0);

        assert_eq!(processing.range.get_minimum(), 1.0);
        assert_eq!(processing.range.get_maximum(), 3.0);
    }

    #[test]
    fn valid_data_2_chart() {
        let processing = setup2chart();

        assert_eq!(
            processing.get_data(),
            &[[(0.0, 1.0), (1.0, 2.0)], [(0.0, 2.0), (1.0, 3.0)]]
        );
    }
}
