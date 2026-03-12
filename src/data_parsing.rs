use crate::DataPoint;

pub struct Parser {
    delimiter: char
}

impl Default for Parser {
    fn default() -> Self {
        Self { delimiter: ',' }
    }
}

impl Parser {
    pub fn parse_line(&self, line: String) -> Result<Option<DataPoint>, std::num::ParseFloatError> {
        let mut split_line= line.split(self.delimiter);

        let first_str = split_line.next();
        let second_str = split_line.next();

        if let Some(first) = first_str 
            && let Some(second) = second_str {

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
        let parser = Parser::default();

        assert_eq!(
            parser.parse_line(String::from("2.0,3.2")),
            Ok(Some((2.0, 3.2)))
        );
    }

    #[test]
    fn valid_line_if_max_float() {
        let parser = Parser::default();

        assert_eq!(
            parser.parse_line(String::from(format!("{},{}", f64::MAX, f64::MAX))),
            Ok(Some((f64::MAX, f64::MAX)))
        );
    }

    #[test]
    fn valid_line_if_int() {
        let parser = Parser::default();

        assert_eq!(
            parser.parse_line(String::from("2,3")),
            Ok(Some((2.0, 3.0)))
        );
    }

    #[test]
    fn valid_if_more_float() {
        let parser = Parser::default();

        assert_eq!(
            parser.parse_line(String::from("1.0,2.0,3.0,4.0")),
            Ok(Some((1.0, 2.0)))
        );
    }

    #[test]
    fn parse_error_if_empty() {
        let parser = Parser::default();

        let result = parser.parse_line(String::from("2.0,"));

        assert!(result.is_err());
    }

    #[test]
    fn parse_error_if_invalid() {
        let parser = Parser::default();

        let result = parser.parse_line(String::from("invalid,2.0"));

        assert!(result.is_err());
    }

    #[test]
    fn invalid_if_empty() {
        let parser = Parser::default();

        assert_eq!(
            parser.parse_line(String::from("")),
            Ok(None)
        );
    }
}
