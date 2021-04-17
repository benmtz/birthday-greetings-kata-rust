use anyhow::Result;

use crate::core::model::employee::Employee;

 pub trait SendGreeting {
    fn to(&self, employee: &Employee) -> Result<()>;
 }
