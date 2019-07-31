extern crate self as rustyreminder;

mod config;
mod email;
mod errors;
mod excel;

use colored::*;
use failure::{format_err, Error, ResultExt};
use rustyreminder::{config::get_config, email::send_emails, excel::process_workbook};
use std::io::Write;

fn main() {
    if let Err(error) = run() {
        print_error(error)
    }
}

fn run() -> Result<(), failure::Error> {
    let config = get_config()?;
    let entries = process_workbook()?;
    if !entries.is_empty() {
        send_emails(config.smtp, entries).context(format_err!("Failed to send email(s)."))?;
    } else {
        println!("No entries to email...");
    }
    Ok(())
}

fn print_error(error: Error) {
    let stderr = &mut ::std::io::stderr();
    let errmsg = "Error writing to stderr";

    let error_colored = "ERROR".red();
    let cause_by_colored = "Caused by".yellow();

    writeln!(stderr, "\n{}: {}\n", error_colored, error).expect(errmsg);

    for cause in error.iter_causes() {
        writeln!(stderr, "{}: {}", cause_by_colored, cause).expect(errmsg);
    }

    if error.backtrace().to_string() != "" {
        writeln!(stderr, "Backtrace: {}", error.backtrace()).expect(errmsg);
    }

    ::std::process::exit(1);
}
