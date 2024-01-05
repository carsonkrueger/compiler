pub trait Reportable {
    fn report(&self) -> String;
    fn report_panic(&self) -> ! {
        panic!("{}", self.report());
    }
}
