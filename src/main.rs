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
    #[structopt(long = "project")]
    project: Option<String>,
    // TODO: --year support
    // #[structopt(long = "year")]
    // year: u32,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let opt = Opt::from_args();
    let license = create_license(
        opt.input.as_str()
    );
    let license = license.unwrap_or_else(|| {
        eprintln!("Not found match license: {}", opt.input);
        process::exit(1);
    });
    let dt = Local::now();
    let current_year = dt.year();
    let author = opt.author.as_str();
    // TODO: want to remove clone
    // TODO: if --project is missing, throw error?
    let project = opt.project.clone().unwrap_or("The project".to_string());

    let license_text = license.notice(
        current_year,
        &author,
        &project);
    write_license(&license_text, "LICENSE").unwrap_or_else(|error| {
        eprintln!("Can not write LICENSE file: {}", error);
        process::exit(1);
    });
    Ok(())
}