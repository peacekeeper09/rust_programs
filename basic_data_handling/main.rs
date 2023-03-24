use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("src/data.csv")?;

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    for result in reader.records() {
        let record = result?;
        let sales: f64 = record[3].parse()?;
        println!("Item: {}, Sales: {}", record[1].to_string(), sales);

    }

    Ok(())
}
