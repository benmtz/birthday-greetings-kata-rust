use std::fs;
use std::path::PathBuf;

use crate::core::model::employee::Employee;
use crate::core::ports::request_employee::RequestEmployee;

use anyhow::Result;
use fs::File;
use serde::Deserialize;
use serde::Serialize;

pub struct EmployeeFileReader {
    file: File
}

impl EmployeeFileReader {
    pub fn from_path(path: &PathBuf) -> EmployeeFileReader {
        let file = File::open(path).expect(&format!("Could not read {:?}", path.display()));
        EmployeeFileReader {
            file: file
        }
    }
}

impl RequestEmployee for EmployeeFileReader {
    fn get_employee_list(&self) -> Result<Vec<Employee>> {
        let mut rdr = csv::Reader::from_reader(&self.file);
        let result: Vec<Employee> = rdr.deserialize().map(|r: Result<CsvEmployee, csv::Error>| r.unwrap().into()).collect();
        Ok(result)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CsvEmployee {
    first_name: String,
    last_name: String,
    date_of_birth: String,
    email: String
}

impl Into<Employee> for CsvEmployee {

    fn into(self) -> Employee {
        Employee {
            first_name: self.first_name,
            last_name: self.last_name,
            date_of_birth: self.date_of_birth.as_str().into(),
            email: self.email,
        }
    }

}
