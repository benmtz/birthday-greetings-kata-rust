use super::birthday::Birthday;
use derive_builder::Builder;

#[derive(Debug, Builder, Clone)]
pub struct Employee {
    pub first_name: String,
    pub last_name: String,
    #[builder(setter(into))]
    pub date_of_birth: Birthday,
    pub email: String
}
