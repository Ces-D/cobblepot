use std::fs::File;
use std::io::BufReader;

use serde::de::DeserializeOwned;

/// Filters ok values from an iterator of type Result<D,Error>
pub fn deserialize_csv<D>(reader: BufReader<File>) -> Vec<D>
where
    D: DeserializeOwned,
{
    let mut reader = csv::Reader::from_reader(reader);
    let rows = reader.deserialize::<D>();
    let mut data: Vec<D> = vec![];
    for row in rows {
        match row {
            Ok(entry) => data.push(entry),
            Err(error) => {
                // TODO: better csv row desearlize handling
                print!("{}", error)
            },
        }
    }
    data
}
