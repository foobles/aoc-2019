use std::io::{self, Write};

#[derive(Clone)]
pub struct Image {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(data: Vec<u8>, width: usize, height: usize) -> Option<Self> {
        if data.len() % (width * height) == 0 {
            Some(Image {
                data,
                width,
                height,
            })
        } else {
            None
        }
    }

    pub fn from_str_with_dims(
        string: &str,
        width: usize,
        height: usize,
    ) -> Result<Self, ParseImageError> {
        let mut data = Vec::with_capacity(string.len());
        for c in string.chars() {
            data.push(c.to_digit(10).ok_or(ParseImageError)? as u8);
        }
        Self::new(data, width, height).ok_or(ParseImageError)
    }

    pub fn get_layers(&self) -> impl Iterator<Item = Layer> {
        self.data
            .chunks_exact(self.width * self.height)
            .map(|data| Layer { data })
    }

    pub fn layer_size(&self) -> usize {
        self.width * self.height
    }

    pub fn flatten(&self) -> Self {
        Image {
            data: (0..self.layer_size())
                .map(|i| {
                    self.get_layers()
                        .map(|layer| layer.data[i])
                        .skip_while(|&n| n == 2)
                        .next()
                        .unwrap_or(2)
                })
                .collect(),
            ..*self
        }
    }

    pub fn render_image(&self, writer: &mut impl Write) -> io::Result<()> {
        let flat = self.flatten();
        writeln!(writer, "P1 {} {} 1", self.width, self.height)?;
        for n in flat.data {
            if n > 1 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid pixel color",
                ));
            }
            writeln!(writer, "{}", 1-n);
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ParseImageError;

#[derive(Copy, Clone, Debug)]
pub struct Layer<'a> {
    data: &'a [u8],
}

impl Layer<'_> {
    pub fn get_data(self) -> [usize; 10] {
        let mut ret = [0; 10];
        for &n in self.data {
            ret[n as usize] += 1;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_image() {
        assert_eq!(
            Image::from_str_with_dims("12344390", 2, 1)
                .unwrap()
                .data
                .as_slice(),
            &[1, 2, 3, 4, 4, 3, 9, 0]
        );
    }

    #[test]
    fn parse_image_wrong_dims() {
        assert!(Image::from_str_with_dims("1234", 3, 2).is_err());
    }

    #[test]
    fn parse_image_wrong_string() {
        assert!(Image::from_str_with_dims("123a0", 5, 1).is_err());
    }

    fn get_layers() {
        assert_eq!(
            Image::from_str_with_dims("12344390", 2, 1)
                .unwrap()
                .get_layers()
                .map(|x| x.data)
                .collect::<Vec<_>>()
                .as_slice(),
            &[&[1, 2], &[3, 4], &[4, 3], &[9, 0]]
        );
    }

    fn get_data() {
        assert_eq!(
            Layer {
                data: &[4, 2, 2, 5, 0, 9, 1, 9, 9, 9]
            }
            .get_data(),
            [1, 1, 2, 0, 1, 1, 0, 0, 0, 4]
        );
    }

    #[test]
    fn flatten() {
        assert_eq!(
            Image::from_str_with_dims("0222112222120000", 2, 2)
                .unwrap()
                .flatten()
                .data
                .as_slice(),
            &[0, 1, 1, 0]
        );
    }
}
