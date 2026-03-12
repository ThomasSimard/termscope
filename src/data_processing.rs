use crate::DataPoint;
use crate::min_max::MinMax;

#[derive(Default)]
pub struct Processing {
    dataset: Vec<DataPoint>, 

    pub domain: MinMax,   // x axis
    pub range: MinMax,     // y axis
}

impl Processing {
    pub fn process(&mut self, data: DataPoint) { 
        self.domain.update(data.0);
        self.range.update(data.1);

        self.dataset.push(data);
    }

    pub fn get_data(&self) -> &Vec<DataPoint> {
       &self.dataset
    }
}
