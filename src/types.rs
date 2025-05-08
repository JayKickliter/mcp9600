/// A wrapper around the raw temperature bytes in MSB order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Temperature(pub [u8; 2]);

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
    pub fn to_f32(self) -> f32 {
        i16::from_be_bytes(self.0) as f32 / 16.0
    }

    #[inline]
    pub fn to_i16(self) -> i16 {
        i16::from_be_bytes(self.0) / 16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_convert_temperature() {
        let data = Temperature([0b0000_1100, 0b0101_0010]);
        assert_eq!(data.to_i16(), 197);
        assert_eq!(data.to_f32(), 197.125);
    }

    #[test]
    fn test_negative_convert_temperature() {
        let data = Temperature([0b1111_0011, 0b1010_1101]);
        assert_eq!(data.to_i16(), -197);
        assert_eq!(data.to_f32(), -197.1875);
    }

    #[test]
    fn test_from_raw() {
        let raw = Temperature([0b1000_1011, 0b1010_1110]);
        assert_eq!(i16::from(raw), -1861);
        assert_eq!(f32::from(raw), -1861.125);
    }
}
