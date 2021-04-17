use chrono::Datelike;
use chrono::NaiveDate;

use super::model::employee::Employee;
use super::ports::error_reporter::ErrorReporter;
use super::ports::request_employee::RequestEmployee;
use super::ports::send_greeting::SendGreeting;

pub struct BirthdayChecker<'a> {
    pub request_employee: &'a dyn RequestEmployee,
    pub send_greeting: &'a dyn SendGreeting,
    pub error_reporter: &'a dyn ErrorReporter,
}

impl<'a> BirthdayChecker<'a> {

    pub fn greet_employee(&self, date_of_day: &NaiveDate) -> anyhow::Result<()>{

        let employee_list = self.request_employee.get_employee_list()?;

        employee_list
            .iter()
            .filter(|&e| self.should_greet(e, date_of_day))
            .for_each(|e| self.send_greeting
                .to(e)
                .unwrap_or_else(|error| self.error_reporter.report(format!("Failed to send greeting to {:?}", error))));

        Ok(())

    }

    fn should_greet(&self, employee: &Employee, date_of_day: &NaiveDate) -> bool {

        if employee.date_of_birth.is_leap() && (date_of_day.month() == 2 && date_of_day.day() == 28) {
            return true;
        }

        employee.date_of_birth.equals(date_of_day)

    }

}

#[cfg(test)]
mod test {

    use crate::core::model::employee::EmployeeBuilder;
    use crate::core::birthday_checker::BirthdayChecker;
    use crate::infrastructure::greeter::noop_greeting_adapter::NoopGreetingAdapter;
    use crate::infrastructure::file::inmemory_reader::InMemoryReader;
    use crate::infrastructure::logger::Logger;

    use std::str::FromStr;

    use chrono::NaiveDate;

    #[test]
    fn leap_years() {
        let employee = EmployeeBuilder::default()
            .first_name("Ben".to_string())
            .last_name("Mertz".to_string())
            .email("no@mail.com".to_string())
            .date_of_birth("2020-02-29").build().unwrap();        

        let reader = InMemoryReader::prepare(vec![employee]);
        let greeter = NoopGreetingAdapter::build();
        let error_reporter = Logger{};

        let checker = BirthdayChecker {
            request_employee: &reader,
            send_greeting: &greeter,
            error_reporter: &error_reporter,
        };

        checker.greet_employee(&NaiveDate::from_str("2021-02-28").unwrap()).unwrap();

        assert_eq!(greeter.calls.get(), 1);
    }

    #[test]
    fn standatard() {
        let e1 = EmployeeBuilder::default()
            .first_name("Ben".to_string())
            .last_name("Mertz".to_string())
            .email("no@mail.com".to_string())
            .date_of_birth("2020-06-29").build().unwrap();        

        let e2 = EmployeeBuilder::default()
            .first_name("Ben".to_string())
            .last_name("Mertz".to_string())
            .email("no@mail.com".to_string())
            .date_of_birth("2020-05-29").build().unwrap();        
            
        let reader = InMemoryReader::prepare(vec![e1, e2]);
        let greeter = NoopGreetingAdapter::build();
        let error_reporter = Logger{};

        let checker = BirthdayChecker {
            request_employee: &reader,
            send_greeting: &greeter,
            error_reporter: &error_reporter,
        };

        checker.greet_employee(&NaiveDate::from_str("2021-06-29").unwrap()).unwrap();

        assert_eq!(greeter.calls.get(), 1);
    }

}
