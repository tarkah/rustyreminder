use error_chain::error_chain;
use std::path::PathBuf;

error_chain! {
    errors {
        ConfigLoad(path: PathBuf) {
            description("failed to load config file")
            display("Failed to load config file: '{}'", path.display())
        }
        ConfigDeser {
            description("failed to deserialize config file")
            display(
"Config file is missing required fields. Example:

\t[smtp]
\thost=smtp.office365.com
\tport=587
\tusername=myemail@domain.tld
\tpassword=mypassword
")
        }
        ExcelLoad(path: PathBuf) {
            description("failed to load Excel file")
            display("Failed to load Excel file: '{}'\n\n\t- Make sure file exists and have correct permissions", path.display())
        }
        ExcelDeser {
            description("failed to deserialize Excel file")
            display("Excel file is not properly formatted.\n\n\t- Make sure 'Sheet1' exists with two colums, Date & Message\n\t- Date column must have all valid dates")
        }
     }
}
