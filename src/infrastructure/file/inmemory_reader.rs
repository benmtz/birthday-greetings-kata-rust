use crate::core::model::employee::Employee;
use crate::core::ports::request_employee::RequestEmployee;

use anyhow::Result;

pub struct InMemoryReader {
    employees: Vec<Employee>,
}

impl InMemoryReader {
    pub fn prepare(employees: Vec<Employee>) -> InMemoryReader {
        InMemoryReader {
            employees: employees,
        }
    }
}

impl RequestEmployee for InMemoryReader {
    fn get_employee_list(&self) -> Result<Vec<Employee>> {
        let employees = self.employees.clone();
        Ok(employees)
    }
}
