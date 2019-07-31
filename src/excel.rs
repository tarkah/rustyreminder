use calamine::{DataType, Range, RangeDeserializerBuilder, Reader, Xlsx};
use chrono::{Local, NaiveDate};
use failure::{format_err, Fallible, ResultExt};
use rustyreminder::errors::AppError;
use std::{fs::File, io::BufReader};
use time::Duration;

const FILE_PATH: &str = "reminders.xlsx";
const SHEET_NAME: &str = "Sheet1";

#[derive(Debug)]
pub struct Entry {
    pub date: NaiveDate,
    pub is_today: bool,
    pub message: String,
}

type XlsxFile = Fallible<Xlsx<BufReader<File>>>;
type SheetRange = Fallible<Range<DataType>>;
type Rows = Fallible<Vec<(f64, String)>>;
type Entries = Fallible<Vec<Entry>>;

pub fn process_workbook() -> Entries {
    let workbook = open_workbook(FILE_PATH).context(AppError::ExcelLoad { path: FILE_PATH })?;
    let range = get_range(workbook).context(AppError::ExcelDeser)?;
    let data = deserialize(range).context(AppError::ExcelDeser)?;

    let today = Local::today().naive_local();
    let base_date = NaiveDate::from_ymd(1900, 1, 1);
    println!("Today is: {}", today);

    println!("Total entries: {}", data.len());

    let todays_entries: Vec<Entry> = data
        .iter()
        .filter_map(|(date, message)| {
            let days: i64 = date.floor() as i64 - 2;
            let date = base_date + Duration::days(days);
            let is_today = today == date;
            let entry = Entry {
                date,
                is_today,
                message: message.clone(),
            };
            if entry.is_today {
                Some(entry)
            } else {
                None
            }
        })
        .collect();
    println!("Todays entries: {:?}", todays_entries.len());

    Ok(todays_entries)
}

fn open_workbook(path: &str) -> XlsxFile {
    let workbook: Xlsx<_> = calamine::open_workbook(path)?;
    Ok(workbook)
}

fn get_range(mut workbook: Xlsx<BufReader<File>>) -> SheetRange {
    let range = workbook
        .worksheet_range(&SHEET_NAME)
        .ok_or(AppError::ExcelNoSheet { sheet: SHEET_NAME });
    Ok(range??)
}

fn deserialize(range: Range<DataType>) -> Rows {
    let iter = RangeDeserializerBuilder::new()
        .has_headers(true)
        .from_range(&range)?;

    let mut data: Vec<(f64, String)> = Vec::new();
    for (i, item) in iter.enumerate() {
        let _data: (f64, String) = item.context(format_err!("Incorrect value on row {}", i + 2))?;
        data.push(_data);
    }

    Ok(data)
}
