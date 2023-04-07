use std::{str::FromStr, ops::{Add, Sub}};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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

impl Add for RGB {
    type Output = RGB;
    fn add(self, rhs: Self) -> Self::Output {
        let r = self.r.saturating_add(rhs.r);
        let g = self.g.saturating_add(rhs.g);
        let b = self.b.saturating_add(rhs.b);
        RGB {r, g, b}
    }
}

impl Sub for RGB {
    type Output = RGB;
    fn sub(self, rhs: Self) -> Self::Output {
        let r = self.r.saturating_sub(rhs.r);
        let g = self.g.saturating_sub(rhs.g);
        let b = self.b.saturating_sub(rhs.b);
        RGB {r, g, b}
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
pub const CLEARCOL: &str = "\\[\x1b[0m\\]";

impl RGB {
    pub fn to_colcode_frag(&self) -> String {
        format!("{};{};{}", &self.r, &self.g, &self.b)
    }

    pub fn as_foreground(&self) -> String {
        format!("\\[\x1b[38;2;{}m\\]", self.to_colcode_frag())
    }
    pub fn as_background(&self) -> String {
        format!("\\[\x1b[48;2;{}m\\]", self.to_colcode_frag())
    }
}
