use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl FromStr for RGB {
    type Err = std::num::ParseIntError;

    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(mut hex_code: &str) -> Result<Self, Self::Err> {
        if let Some('#') = hex_code.chars().next() {
            hex_code = &hex_code[1..7];
        }
        // u8::from_str_radix(src: &str, radix: u32) converts a string
        // slice in a given base to u8
        let r: u8 = u8::from_str_radix(&hex_code[0..2], 16)?;
        let g: u8 = u8::from_str_radix(&hex_code[2..4], 16)?;
        let b: u8 = u8::from_str_radix(&hex_code[4..6], 16)?;

        Ok(RGB { r, g, b })
    }
}

impl From<&str> for RGB {
    fn from(value: &str) -> Self {
        match RGB::from_str(value) {
            Ok(a) => a,
            Err(_) => RGB { r: 0, g: 0, b: 0 }
        }
    }
}

impl RGB {
    pub fn to_colcode_frag(&self) -> String {
        format!("{};{};{}", &self.r, &self.g, &self.b)
    }
}
