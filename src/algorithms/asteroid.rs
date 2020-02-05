use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsteroidField {
    width: usize,
    height: usize,
    field: Vec<bool>,
}

#[derive(Copy, Clone, Debug)]
pub struct ParseAsteroidFieldError;

impl FromStr for AsteroidField {
    type Err = ParseAsteroidFieldError;

    fn from_str(string: &str) -> Result<Self, ParseAsteroidFieldError> {
        let mut field = Vec::new();
        let mut height = 0;
        let mut width = None;
        for line in string.lines() {
            height += 1;
            if let Some(w) = width {
                if w != line.len() {
                    return Err(ParseAsteroidFieldError);
                }
            } else {
                width = Some(line.len());
            }
            for c in line.chars() {
                field.push(match c {
                    ' ' => false,
                    '#' => true,
                    _ => return Err(ParseAsteroidFieldError),
                });
            }
        }
        Ok(AsteroidField {
            width: width.unwrap_or(0),
            height,
            field,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::asteroid::AsteroidField;

    #[test]
    fn asteroid_parse() {
        assert_eq!(
            AsteroidField {
                width: 3,
                height: 2,
                field: vec![true, false, false, true, false, true]
            },
            "#  \n# #".parse::<AsteroidField>().unwrap()
        );
    }

    #[test]
    fn asteroid_wrong_widths() {
        assert!("## \n#\n   #".parse::<AsteroidField>().is_err());
    }

    #[test]
    fn asteroid_wrong_chars() {
        assert!("abc\ndef\nhij".parse::<AsteroidField>().is_err());
    }

    #[test]
    fn asteroid_empty() {
        assert_eq!(
            AsteroidField {
                width: 0,
                height: 0,
                field: vec![]
            },
            "".parse::<AsteroidField>().unwrap()
        );
    }
}
