#[derive(Clone, Copy)]
pub struct Register(i32);

impl Register {
    pub fn get_i32(&self) -> i32 {
        self.0
    }
    pub fn get_u8(&self) -> u8 {
        self.0 as u8
    }
    pub fn set_i32(&mut self, num: i32) {
        self.0 = num;
    }
    pub fn set_u8(&mut self, num: u8) {
        self.0 = num as i32;
    }
}
