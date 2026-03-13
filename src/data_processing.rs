use crate::DataPoint;
use crate::min_max::MinMax;

pub struct Processing {
    datasets: Vec<Vec<(f64, f64)>>, 

    pub domain: MinMax,
    pub range: MinMax,
}

impl Processing {
    pub fn new(number_of_charts: usize) -> Self {
        let datasets = vec![Vec::default(); number_of_charts];

        Self {
            datasets,
            domain: MinMax::default(), 
            range: MinMax::default(), 
        }
    }

    pub fn process(&mut self, row_of_data: &[f64]) { 
        let mut data_iterator = row_of_data.iter();

        if let Some(data) = data_iterator.next() {
            self.domain.update(*data);
        }

        for data in data_iterator {
            self.range.update(*data);
        }

        let mut data_iterator = row_of_data.iter();

        if let Some(x) = data_iterator.next() {
            for dataset in &mut self.datasets {
                if let Some(y) = data_iterator.next() {
                    dataset.push((*x, *y));
                }
            }
        }
    }

    pub fn get_data(&self) -> &Vec<Vec<DataPoint>> {
       &self.datasets
    }
}
