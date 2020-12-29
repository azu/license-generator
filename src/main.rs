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
    inputs: Vec<String>,
    #[structopt(long = "author")]
    author: String,
    #[structopt(long = "project")]
    project: Option<String>,
    #[structopt(long = "year")]
    year: Option<u32>,
    #[structopt(long = "output")]
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let dt = Local::now();
    let current_year = dt.year();
    let year = opt.year.unwrap_or_else(|| {
        current_year as u32
    });
    let author = opt.author.as_str();
    // TODO: want to remove clone
    let project = opt.project.clone().unwrap_or_else(|| {
        env::current_dir()
            .expect("use --project: Not found current dir")
            .file_name()
            .expect("use --project: Not found directory name")
            .to_os_string()
            .into_string()
            .expect("use --project: Fail to unwrap os_string")
    });
    let output = opt.output.unwrap_or_else(|| {
        "LICENSE".to_string()
    });
    let multi_license = opt.inputs.len() > 1;

    opt.inputs
        .iter()
        .for_each(|s| {
            let license = create_license(s.as_str()).unwrap_or_else(|| {
                eprintln!("Not found match license: {}", s);
                process::exit(1);
            });
            let license_text = license.notice(
                year,
                &author,
                &project
            );
            let output = if multi_license { format!("{}_{}", output, s.to_uppercase()) }
                         else { output.clone() };
            write_license(&license_text, &output).unwrap_or_else(|error| {
                eprintln!("Can not write license text to \"{}\": {}", output, error);
                process::exit(1);
            });
        });
    Ok(())
}
