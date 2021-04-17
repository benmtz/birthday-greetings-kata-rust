pub trait ErrorReporter {
    fn report(&self, message: String) -> ();   
}