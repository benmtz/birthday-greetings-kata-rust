use std::path::PathBuf;
use std::str::FromStr;
use chrono::Utc;
use chrono::NaiveDate;
use structopt::StructOpt;
use anyhow::Result;

use crate::core::birthday_checker::BirthdayChecker;
use crate::infrastructure::file::employee_file_reader::EmployeeFileReader;
use crate::infrastructure::greeter::email_greeting_adapter::EmailGreetingAdapterBuilder;
use crate::infrastructure::greeter::noop_greeting_adapter::NoopGreetingAdapter;
use crate::infrastructure::logger::Logger;

pub struct Console<'a> {
    pub birthday_checker: &'a BirthdayChecker<'a>
}

impl<'a> Console<'a> {

    pub fn run() -> Result<()>{
        
        let opt = Opt::from_args();
        let date = NaiveDate::from_str(&opt.date.unwrap_or("".to_string())).unwrap_or(Utc::today().naive_local());
        
        let reader = EmployeeFileReader::from_path(&opt.input);
        let reporter = Logger{};
        
        let noop_greeter = NoopGreetingAdapter::build();
        let email_greeter = EmailGreetingAdapterBuilder::default()
                .smtp_server(opt.smtp_server.unwrap())
                .smtp_port(opt.smtp_port.unwrap())
                .smtp_username(opt.smtp_username.unwrap())
                .smtp_password(opt.smtp_password.unwrap())
                .from(opt.from)
                .build().unwrap();
        
        let birthday_checker = BirthdayChecker {
            request_employee: &reader,
            send_greeting: if opt.dry_run {&noop_greeter} else { &email_greeter },
            error_reporter: &reporter
        };

        
        birthday_checker.greet_employee(&date)?;
        Ok(())
    }

}


#[derive(Debug, StructOpt)]
#[structopt(name = "greet-cli", about = "Greet employee on birthday")]
struct Opt {
    #[structopt(short,long, parse(from_os_str))]
    input: PathBuf,

    #[structopt(short,long)]
    date: Option<String>,

    #[structopt(long)]
    smtp_server: Option<String>,

    #[structopt(long)]
    smtp_port: Option<u16>,

    #[structopt(long)]
    smtp_username: Option<String>,
    
    #[structopt(long)]
    smtp_password: Option<String>,
    
    #[structopt(long)]
    from: String,
    
    #[structopt(long)]
    dry_run: bool
    
}
