use crate::core::ports::error_reporter::ErrorReporter;

pub struct Logger {}

impl ErrorReporter for Logger {
    fn report(&self, message: String) -> () {
        println!("{}", message);
    }
}