use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit: u8,
}
pub fn process_csv(opts: &crate::CsvOpt) -> Result<()> {
    let mut reader = Reader::from_path(opts.input.clone())?;
    let mut ret = Vec::with_capacity(100);
    for result in reader.deserialize() {
        let record: Player = result?;
        ret.push(record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    std::fs::write(opts.output.clone(), json)?;
    Ok(())
}
