pub struct DataParser {
    delimiter: char
}

impl Default for DataParser {
    fn default() -> Self {
        Self { delimiter: ',' }
    }
}

impl DataParser {
    pub fn parse_line(&self, line: String, x_column: usize, y_columns: &Vec<usize>) -> Result<Vec<f64>, std::num::ParseFloatError> {
        let mut datapoints:  Vec<f64> = Vec::default();

        let x_str = line.split(self.delimiter).nth(x_column - 1);

        if let Some(x) = x_str {
            datapoints.push(x.parse::<f64>()?);
        }

        for y_column in y_columns {
            let y_str = line.split(self.delimiter).nth(y_column - 1);

            if let Some(y) = y_str {
                datapoints.push(y.parse::<f64>()?);
            }
        }

        Ok(datapoints)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_line_if_float() {
        let parser = DataParser::default();

        assert_eq!(
            parser.parse_line(String::from("2.0,3.2"), 1, &vec![2]),
            Ok(vec![2.0, 3.2])
        );
    }

    #[test]
    fn valid_line_if_max_float() {
        let parser = DataParser::default();

        assert_eq!(
            parser.parse_line(String::from(format!("{},{}", f64::MAX, f64::MAX)), 1, &vec![2]),
            Ok(vec![f64::MAX, f64::MAX])
        );
    }

    #[test]
    fn valid_line_if_int() {
        let parser = DataParser::default();

        assert_eq!(
            parser.parse_line(String::from("2,3"), 1, &vec![2]),
            Ok(vec![2.0, 3.0])
        );
    }

    #[test]
    fn valid_if_more_float() {
        let parser = DataParser::default();

        assert_eq!(
            parser.parse_line(String::from("1.0,2.0,3.0,4.0"), 1, &vec![2]),
            Ok(vec![1.0, 2.0])
        );
    }

    #[test]
    fn parse_error_if_empty() {
        let parser = DataParser::default();

        let result = parser.parse_line(String::from("2.0,"), 1, &vec![2]);

        assert!(result.is_err());
    }

    #[test]
    fn parse_error_if_invalid() {
        let parser = DataParser::default();

        let result = parser.parse_line(String::from("invalid,2.0"), 1, &vec![2]);

        assert!(result.is_err());
    }

    #[test]
    fn invalid_if_empty() {
        let parser = DataParser::default();

        assert_eq!(
            parser.parse_line(String::from(""), 1, &vec![2]),
            Ok(Vec::default())
        );
    }
}
