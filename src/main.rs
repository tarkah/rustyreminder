extern crate self as rustyreminder;

mod config;
mod email;
mod errors;
mod excel;

use rustyreminder::config::get_config;
use rustyreminder::email::send_emails;
use rustyreminder::errors::*;
use rustyreminder::excel::process_workbook;

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr).expect(errmsg);
        writeln!(stderr, "ERROR: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "Caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "Backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let config = get_config()?;
    let entries = process_workbook()?;
    send_emails(config.smtp, entries)?;
    Ok(())
}
