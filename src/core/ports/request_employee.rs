use anyhow::Result;

use crate::core::model::employee::Employee;

 pub trait RequestEmployee {
    fn get_employee_list(&self) -> Result<Vec<Employee>>;
 }
