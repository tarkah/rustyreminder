use calamine::{DataType, Range, RangeDeserializerBuilder, Reader, Xlsx};
use chrono::{Local, NaiveDate};
use rustyreminder::errors::*;
use time::Duration;

#[derive(Debug)]
pub struct Entry {
    pub date: NaiveDate,
    pub is_today: bool,
    pub message: String,
}

pub fn process_workbook() -> Result<Vec<Entry>> {
    let range = open_workbook()?;
    let data = deserialize(range)?;

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

fn open_workbook() -> Result<Range<DataType>> {
    let path = "reminders.xlsx";
    let mut workbook: Xlsx<_> =
        calamine::open_workbook(path).map_err(|_| ErrorKind::ExcelLoad(path.into()))?;
    let range = workbook
        .worksheet_range("Sheet1")
        .ok_or(ErrorKind::ExcelDeser)?
        .map_err(|_| ErrorKind::ExcelDeser)?;
    Ok(range)
}

fn deserialize(range: Range<DataType>) -> Result<Vec<(f64, String)>> {
    let iter = RangeDeserializerBuilder::new()
        .has_headers(true)
        .from_range(&range)
        .map_err(|_| ErrorKind::ExcelDeser)?;

    let mut data: Vec<(f64, String)> = Vec::new();
    for item in iter {
        let _data: (f64, String) = item.map_err(|_| ErrorKind::ExcelDeser)?;
        data.push(_data);
    }

    Ok(data)
}
