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
}

impl Default for Register {
    fn default() -> Self {
        Self(0)
    }
}
