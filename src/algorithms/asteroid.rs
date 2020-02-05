use std::str::FromStr;
use std::ops::{Index, Add, Mul, AddAssign, MulAssign};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsteroidField {
    width: usize,
    height: usize,
    field: Vec<bool>,
}

#[derive(Copy, Clone, Debug)]
pub struct ParseAsteroidFieldError;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point(pub isize, pub isize);

impl Point {
    fn x(self) -> isize {
        self.0
    }

    fn y(self) -> isize {
        self.1
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = *self + rhs;
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, rhs: isize) -> Point {
        Point(self.x() * rhs, self.y() * rhs)
    }
}

impl MulAssign<isize> for Point {
    fn mul_assign(&mut self, rhs: isize) {
        *self = *self * rhs;
    }
}

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
    use super::*;

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

    #[test]
    fn point_add() {
        assert_eq!(Point(5, -2), Point(2, 3) + Point(3, -5));
    }

    #[test]
    fn point_add_assign() {
        let mut p = Point(1, 2);
        p += Point(5, -10);
        assert_eq!(p, Point(6, -8));
    }

    #[test]
    fn point_mul() {
        assert_eq!(Point(10, 5), Point(2, 1) * 5);
    }

    #[test]
    fn point_mul_assign() {
        let mut p = Point(4, -5);
        p *= -300;
        assert_eq!(Point(-1200, 1500), p);
    }
}
