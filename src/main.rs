use fake::locales::{EN};
use fake::Fake;
use std::error::Error;
use std::process;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Lead {
    nome: String,
    email: String,
}

fn write_to_stdout(path: &str) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path(path)?;

    use fake::faker::name::raw::*;
    use fake::faker::internet::raw::*;

    for _ in 0..10000 {
        writer.serialize(Lead {
            nome: Name(EN).fake(),
            email: SafeEmail(EN).fake(),
        })?;
    }
    writer.flush()?;

    Ok(())
}
fn main() {
    if let Err(err) = write_to_stdout("teste.csv") {
        println!("error running example: {}", err);
        process::exit(1);
    }
}