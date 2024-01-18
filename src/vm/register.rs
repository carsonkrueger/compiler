#[derive(Clone, Copy)]
pub struct Register(i32);

impl Register {
    pub fn get_f32(&self) -> f32 {
        self.0 as f32
    }
    pub fn get_i32(&self) -> i32 {
        self.0
    }
    pub fn get_u8(&self) -> u8 {
        self.0 as u8
    }
    pub fn set_f32(&mut self, num: f32) {
        self.0 = num as i32;
    }
    pub fn set_i32(&mut self, num: i32) {
        self.0 = num;
    }
    pub fn set_u8(&mut self, num: u8) {
        self.0 = num as i32;
    }
    pub fn pc_idx() -> usize {
        64
    }
    pub fn sl_idx() -> usize {
        65
    }
    pub fn sb_idx() -> usize {
        66
    }
    pub fn sp_idx() -> usize {
        67
    }
    pub fn fp_idx() -> usize {
        68
    }
    pub fn hp_idx() -> usize {
        69
    }
}

impl Default for Register {
    fn default() -> Self {
        Self(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_i32() {
        let mut r = Register(0);

        assert_eq!(r.get_i32(), 0);
        r.0 = 5;
        assert_eq!(r.get_i32(), 5);
        r.0 = 255;
        assert_eq!(r.get_i32(), 255);
        r.0 = 256;
        assert_eq!(r.get_i32(), 256);
        r.0 = 1000000000;
        assert_eq!(r.get_i32(), 1000000000);
        r.0 = -1000000000;
        assert_eq!(r.get_i32(), -1000000000);
    }

    #[test]
    fn test_get_u8() {
        let mut r = Register(0);

        assert_eq!(r.get_u8(), 0);
        r.0 = 5u8 as i32;
        assert_eq!(r.get_u8(), 5);
        r.0 = 255u8 as i32;
        assert_eq!(r.get_u8(), 255);
        r.0 = 161u8 as i32;
        assert_eq!(r.get_u8(), 161);
    }

    #[test]
    fn test_set_u8() {
        let mut r = Register(0);

        r.set_u8(0);
        assert_eq!(r.get_u8(), 0);

        r.set_u8(151);
        assert_eq!(r.get_u8(), 151);

        r.set_u8(255);
        assert_eq!(r.get_u8(), 255);

        r.set_i32(255);
        assert_eq!(r.get_u8(), 255);

        r.set_i32(256);
        assert_eq!(r.get_u8(), 0);

        r.set_i32(257);
        assert_eq!(r.get_u8(), 1);

        r.set_i32(258);
        assert_ne!(r.get_u8(), 1);
    }
}
