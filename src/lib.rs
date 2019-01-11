extern crate license;

use license::License;
use std::fs::File;
use std::io::Write;
use std::io;


pub fn write_license(license: &Box<License>, output_path: &str) -> Result<(), io::Error> {
    let mut file = File::create(output_path)?;
    file.write(license.text().as_bytes())?;
    Ok(())
}

pub fn create_license<'a>(license_type: &'a str, author: &'a str, year: i32) -> Option<Box<License>> {
    match license_type {
        // MIT
        "mit" => Some(Box::new(license::Mit::new(year, &author))),
        "MIT" => Some(Box::new(license::Mit::new(year, &author))),
        // APGL
        "apgl" => Some(Box::new(license::Agpl3)),
        "APGL" => Some(Box::new(license::Agpl3)),
        "apgl-3" => Some(Box::new(license::Agpl3)),
        "APGL-3" => Some(Box::new(license::Agpl3)),
        // Apache
        "apache" => Some(Box::new(license::Apache2)),
        "Apache" => Some(Box::new(license::Apache2)),
        // CC0
        "cc0" => Some(Box::new(license::Cc01)),
        "CC0" => Some(Box::new(license::Cc01)),
        // LGPL
        "lgpl" => Some(Box::new(license::Lgpl3)),
        "LGPL" => Some(Box::new(license::Lgpl3)),
        "lgpl-3" => Some(Box::new(license::Lgpl3)),
        "LGPL-3" => Some(Box::new(license::Lgpl3)),
        // MPL
        "mpl" => Some(Box::new(license::Mpl2)),
        "MPL" => Some(Box::new(license::Mpl2)),
        // Unlicense
        "unlinse" => Some(Box::new(license::Unlicense)),
        "Unlinse" => Some(Box::new(license::Unlicense)),
        "UNLICENSE" => Some(Box::new(license::Unlicense)),
        _ => None
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use license::*;

    #[test]
    fn not_found_license() {
        let license = create_license("NONONONO", "azu", 2018);
        assert!(license.is_none());
    }

    #[test]
    fn create_mit_license() {
        let license = create_license("MIT", "azu", 2018);
        let l = license.unwrap();
        assert_eq!(l.name(), Mit::new(2018, "azu").name());
        assert_eq!(l.id(), Mit::new(2018, "azu").id());
    }

    #[test]
    fn create_other_license() {
        let license = create_license("cc0", "azu", 2018);
        let l = license.unwrap();
        assert_eq!(l.name(), Cc01.name());
        assert_eq!(l.id(), Cc01.id());
    }
}