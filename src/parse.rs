use csv::ReaderBuilder;
use csv::StringRecordsIntoIter;
use std::error::Error;
use std::fs::File;

pub fn file_to_iter(file_path: &str) -> Result<StringRecordsIntoIter<File>, Box<dyn Error>> {
    let rdr = ReaderBuilder::new().delimiter(b'\t').from_path(file_path)?;

    let iter = rdr.into_records();

    Ok(iter)
}

pub fn build_trace(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut trace = Vec::new();

    let mut iter = file_to_iter(file_path).unwrap();

    while let Some(record) = iter.next() {
        let val = record?;
        trace.push(val[1].to_string());
    }

    Ok(trace)
}
