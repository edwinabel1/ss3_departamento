pub trait Action {
    fn isme(&self, s: &str) -> bool;
    fn run(&self, acts: &str);
}
