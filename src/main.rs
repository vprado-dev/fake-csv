use fake::locales::{EN};
use fake::Fake;
use std::error::Error;
use std::process;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Person {
  nome: String,
  email: String,
}

fn write_csv_file(path: &str) -> Result<(), Box<dyn Error>> {
  let mut writer = csv::Writer::from_path(path)?;

  use fake::faker::name::raw::*;
  use fake::faker::internet::raw::*;

  for _ in 0..10000 {
    writer.serialize(Person {
      nome: Name(EN).fake(),
      email: SafeEmail(EN).fake(),
    })?;
  }
  writer.flush()?;

  Ok(())
}
fn main() {
  if let Err(err) = write_csv_file("teste.csv") {
    println!("error running example: {}", err);
    process::exit(1);
  }
}