extern crate chrono;

use chrono::Datelike;
use chrono::Local;
use structopt::StructOpt;

use license_generator::create_license;
use license_generator::write_license;
use std::process;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "INPUT")]
    input: String,
    #[structopt(long = "author")]
    author: String,
    // TODO: --year support
    // #[structopt(long = "year")]
    // year: u32,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let opt = Opt::from_args();
    let dt = Local::now();
    let current_year = dt.year();
    let license = create_license(
        opt.input.as_str(),
        opt.author.as_str(),
        current_year);
    let license = license.unwrap_or_else(|| {
        eprintln!("Not found match license: {}", opt.input);
        process::exit(1);
    });
    write_license(&license, "LICENSE").unwrap_or_else(|error| {
        eprintln!("Can not write LICENSE file: {}", error);
        process::exit(1);
    });
    Ok(())
}