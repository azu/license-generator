extern crate chrono;

use chrono::Datelike;
use chrono::Local;
use structopt::StructOpt;

use license_generator::create_license;
use license_generator::write_license;
use std::process;
use std::env;

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
    let project = opt.project.clone().unwrap_or_else(|| {
        env::current_dir()
            .expect("--project: Not found current dir")
            .file_name()
            .expect("--project: Not found directory name")
            .to_os_string()
            .into_string()
            .expect("--project: Fail to wnwrap os_string")
    });

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