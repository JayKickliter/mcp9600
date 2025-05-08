#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Temperature {
    pub lsb: u8,
    pub msb: u8,
}

impl From<Temperature> for i16 {
    fn from(other: Temperature) -> i16 {
        other.to_i16()
    }
}

impl From<Temperature> for f32 {
    fn from(other: Temperature) -> f32 {
        other.to_f32()
    }
}

impl Temperature {
    #[inline]
    fn to_f32(self) -> f32 {
        match (self.msb as i8).is_negative() {
            true => (self.msb as f32 * 16.0 + self.lsb as f32 / 16.0) - 4096.0,
            false => self.msb as f32 * 16.0 + self.lsb as f32 / 16.0,
        }
    }

    #[inline]
    fn to_i16(self) -> i16 {
        match (self.msb as i8).is_negative() {
            true => (self.msb as i16 * 16 + self.lsb as i16 / 16) - 4096,
            false => self.msb as i16 * 16 + self.lsb as i16 / 16,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_convert_temperature() {
        let data = Temperature {
            msb: 0b0000_1100u8,
            lsb: 0b0101_0010u8,
        };
        assert_eq!(data.to_i16(), 197);
        assert_eq!(data.to_f32(), 197.125);
    }

    #[test]
    fn test_negative_convert_temperature() {
        let data = Temperature {
            msb: 0b1111_0011u8,
            lsb: 0b1010_1101u8,
        };
        assert_eq!(data.to_i16(), -198);
        assert_eq!(data.to_f32(), -197.1875);
    }

    #[test]
    fn test_from_raw() {
        let raw = Temperature {
            msb: 0b1000_1011u8,
            lsb: 0b1010_1110u8,
        };
        assert_eq!(i16::from(raw), -1862);
        assert_eq!(f32::from(raw), -1861.125);
    }
}
