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

    writer.serialize(Lead {
        nome: "Ailey".to_string(),
        email: "abenstead0@state.gov".to_string(),
    })?;

    writer.serialize(Lead {
        nome: "Ninnette".to_string(),
        email: "nwasmuth1@washington.edu".to_string(),
    })?;

    writer.flush()?;

    Ok(())
}
fn main() {
    if let Err(err) = write_to_stdout("teste.csv") {
        println!("error running example: {}", err);
        process::exit(1);
    }
}