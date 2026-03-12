
use crate::min_max::MinMax;

#[derive(Default)]
pub struct Processing {
    dataset: Vec<(f64, f64)>, 

    pub domain: MinMax,   // x axis
    pub range: MinMax,     // y axis
}

impl Processing {
    pub fn process(&mut self, data: (f64, f64)) { 
        self.domain.update(data.0);
        self.range.update(data.1);

        self.dataset.push(data);
    }

    pub fn get_data(&self) -> &Vec<(f64, f64)> {
       &self.dataset
    }
}
