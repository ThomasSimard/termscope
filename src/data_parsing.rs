use crate::DataPoint;

pub struct DataParser {
    delimiter: char
}

impl Default for DataParser {
    fn default() -> Self {
        Self { delimiter: ',' }
    }
}

impl DataParser {
    pub fn parse_line(&self, line: String, x_column: usize, y_column: usize) -> Result<Option<DataPoint>, std::num::ParseFloatError> {
        let x_str = line.split(self.delimiter).nth(x_column - 1);
        let y_str = line.split(self.delimiter).nth(y_column - 1);

        if let Some(first) = x_str 
            && let Some(second) = y_str {

                return Ok(Some(
                        (first.parse::<f64>()?,
                        second.parse::<f64>()?)
                ));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_line_if_float() {
        let parser = DataParser::default();

        assert_eq!(
            parser.parse_line(String::from("2.0,3.2")),
            Ok(Some((2.0, 3.2)))
        );
    }

    #[test]
    fn valid_line_if_max_float() {
        let parser = DataParser::default();

        assert_eq!(
            parser.parse_line(String::from(format!("{},{}", f64::MAX, f64::MAX))),
            Ok(Some((f64::MAX, f64::MAX)))
        );
    }

    #[test]
    fn valid_line_if_int() {
        let parser = DataParser::default();

        assert_eq!(
            parser.parse_line(String::from("2,3")),
            Ok(Some((2.0, 3.0)))
        );
    }

    #[test]
    fn valid_if_more_float() {
        let parser = DataParser::default();

        assert_eq!(
            parser.parse_line(String::from("1.0,2.0,3.0,4.0")),
            Ok(Some((1.0, 2.0)))
        );
    }

    #[test]
    fn parse_error_if_empty() {
        let parser = DataParser::default();

        let result = parser.parse_line(String::from("2.0,"));

        assert!(result.is_err());
    }

    #[test]
    fn parse_error_if_invalid() {
        let parser = DataParser::default();

        let result = parser.parse_line(String::from("invalid,2.0"));

        assert!(result.is_err());
    }

    #[test]
    fn invalid_if_empty() {
        let parser = DataParser::default();

        assert_eq!(
            parser.parse_line(String::from("")),
            Ok(None)
        );
    }
}
