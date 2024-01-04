pub struct Register(i32);

impl Register {
    fn get_i32(&self) -> i32 {
        self.0
    }
    fn get_u8(&self) -> u8 {
        self.0 as u8
    }
    fn set_i32(&mut self, num: i32) {
        self.0 = num;
    }
    fn set_u8(&mut self, num: u8) {
        self.0 = num as i32;
    }
}