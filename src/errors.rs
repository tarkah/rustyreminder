use failure::Fail;

#[derive(Debug, Fail)]
pub enum AppError {
    #[fail(display = "Failed to load config file: '{}'", path)]
    ConfigLoad { path: &'static str },
    #[fail(display = "Config file has invalid or missing field. Example: \
                      \n \
                      \n\t[smtp] \
                      \n\thost=smtp.office365.com \
                      \n\tport=587 \
                      \n\tusername=myemail@domain.tld \
                      \n\tpassword=mypassword \
                      ")]
    ConfigDeser,
    #[fail(
        display = "Failed to load Excel file: '{}' \
                   \n \
                   \n\t- Make sure file exists and has correct permissions \
                   ",
        path
    )]
    ExcelLoad { path: &'static str },
    #[fail(display = "Excel file is not properly formatted. \
                      \n \
                      \n\t- Make sure 'Sheet1' exists with two colums, Date & Message \
                      \n\t- Date column must have all valid dates \
                      ")]
    ExcelDeser,
    #[fail(display = "Sheet {:?} doesn't exist", sheet)]
    ExcelNoSheet { sheet: &'static str },
}
