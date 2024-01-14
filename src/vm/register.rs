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
        65
    }
    pub fn sl_idx() -> usize {
        66
    }
    pub fn sb_idx() -> usize {
        67
    }
    pub fn sp_idx() -> usize {
        68
    }
    pub fn fp_idx() -> usize {
        69
    }
    pub fn hp_idx() -> usize {
        70
    }
}

impl Default for Register {
    fn default() -> Self {
        Self(0)
    }
}
