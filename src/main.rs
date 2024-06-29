use csv::Reader;
use std::error::Error;
use std::fs::File;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;

    let mut reader = Reader::from_reader(file);

    for result in reader.records() {
        let record = result?;

        print!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("./demo.csv") {
        eprint!("{}", e);
    }
}
