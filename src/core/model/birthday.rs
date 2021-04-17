use std::str::FromStr;

use chrono::Datelike;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Birthday {
    pub date: NaiveDate,
}

impl Birthday {

    /// # Examples
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use hexagonal_rs::core::model::birthday::Birthday;
    /// use std::str::FromStr;
    ///
    /// let birthday = NaiveDate::from_str("2020-02-25").unwrap();
    /// let date_of_birth = Birthday::from("1990-02-25");
    /// let not_birthday = NaiveDate::from_str("2020-06-06").unwrap();
    ///
    /// assert!(!date_of_birth.equals(&not_birthday));
    /// assert!(date_of_birth.equals(&birthday));
    /// ```
    pub fn equals(&self, naive_date: &NaiveDate) -> bool {
        return self.date.month().eq(&naive_date.month()) && self.date.day().eq(&naive_date.day())
    }

    /// # Examples
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use hexagonal_rs::core::model::birthday::Birthday;
    /// use std::str::FromStr;
    ///
    /// let leap = Birthday::from("2020-02-29");
    /// let not_leap = Birthday::from("2020-02-28");
    ///
    /// assert!(leap.is_leap());
    /// assert!(!not_leap.is_leap());
    /// ```
    pub fn is_leap(&self) -> bool {
        self.date.month() == 2 && self.date.day() == 29
    }

}

impl From<&str> for Birthday {

    fn from(str: &str) -> Birthday {
        Birthday {
            date: NaiveDate::from_str(str).unwrap()
        }
    }

}
