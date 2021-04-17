use crate::core::model::employee::Employee;
use crate::core::ports::send_greeting::SendGreeting;
use std::cell::Cell;

pub struct NoopGreetingAdapter {
    pub calls: Cell<i64>
}

impl NoopGreetingAdapter {

    pub fn build() -> NoopGreetingAdapter {
        NoopGreetingAdapter {
            calls: Cell::new(0)
        }
    }
}

impl SendGreeting for NoopGreetingAdapter {
    fn to(&self, employee: &Employee) -> anyhow::Result<()> {
        let previous_ref = self.calls.get();
        self.calls.set(previous_ref + 1);
        println!("[{:?}] Happy birthday {} {}", employee.date_of_birth.date, employee.first_name, employee.last_name);
        Ok(())
    }
}
