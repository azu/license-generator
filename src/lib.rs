use std::fs::File;
use std::io::Write;
use std::io;

pub mod license;

pub fn write_license(license_text: &str, output_path: &str) -> Result<(), io::Error> {
    let mut file = File::create(output_path)?;
    file.write_all(license_text.as_bytes())?;
    Ok(())
}

pub fn create_license(license_type: &str) -> Option<Box<dyn license::License>> {
    match license_type {
        // BSD
        "bsd" => Some(Box::new(license::BSD {})),
        "BSD" => Some(Box::new(license::BSD {})),
        // MIT
        "mit" => Some(Box::new(license::Mit {})),
        "MIT" => Some(Box::new(license::Mit {})),
        // GPL
        "gpl" => Some(Box::new(license::GPL {})),
        "GPL" => Some(Box::new(license::GPL {})),
        "gpl-3" => Some(Box::new(license::GPL {})),
        "GPL-3" => Some(Box::new(license::GPL {})),
        // APGL
        "apgl" => Some(Box::new(license::AGPL {})),
        "APGL" => Some(Box::new(license::AGPL {})),
        "apgl-3" => Some(Box::new(license::AGPL {})),
        "APGL-3" => Some(Box::new(license::AGPL {})),
        // Apache
        "apache" => Some(Box::new(license::Apache {})),
        "Apache" => Some(Box::new(license::Apache {})),
        // CC_BY
        "CC-BY" => Some(Box::new(license::CcBy {})),
        "CCBY" => Some(Box::new(license::CcBy {})),
        "ccby" => Some(Box::new(license::CcBy {})),
        // CC_BY_NC
        "CC-BY-NC" => Some(Box::new(license::CcByNc {})),
        "CCBYNC" => Some(Box::new(license::CcByNc {})),
        "ccbync" => Some(Box::new(license::CcByNc {})),
        // CC_BY_SA
        "CC-BY-SA" => Some(Box::new(license::CcBySa {})),
        "CCBYSA" => Some(Box::new(license::CcBySa {})),
        "ccbysa" => Some(Box::new(license::CcBySa {})),
        // CC_BY_NC_SA
        "CC-BY-NC-SA" => Some(Box::new(license::CcByNcSa {})),
        "CCBYNCSA" => Some(Box::new(license::CcByNcSa {})),
        "ccbyncsa" => Some(Box::new(license::CcByNcSa {})),
        // CC0
        "cc0" => Some(Box::new(license::CCZero {})),
        "CC0" => Some(Box::new(license::CCZero {})),
        // LGPL
        "lgpl" => Some(Box::new(license::LGPL {})),
        "LGPL" => Some(Box::new(license::LGPL {})),
        "lgpl-3" => Some(Box::new(license::LGPL {})),
        "LGPL-3" => Some(Box::new(license::LGPL {})),
        // MPL
        "mpl" => Some(Box::new(license::MPL {})),
        "MPL" => Some(Box::new(license::MPL {})),
        // Unlicense
        "unlicense" => Some(Box::new(license::UNLICENSE {})),
        "Unlicense" => Some(Box::new(license::UNLICENSE {})),
        "UNLICENSE" => Some(Box::new(license::UNLICENSE {})),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn not_found_license() {
        let license = create_license("_NO_LICENSE_");
        assert!(license.is_none());
    }

    #[test]
    fn create_mit_license() {
        let mit = create_license("mit");
        let license_text = mit.unwrap().notice(2018, "azu", "license-generator");
        assert!(license_text.contains("Permission is hereby granted"));
        assert!(license_text.contains("azu"));
        assert!(license_text.contains("2018"));
    }

    #[test]
    fn create_gpl_license() {
        let gpl = create_license("gpl");
        let license_text = gpl.unwrap().notice(2018, "azu", "license-generator");
        assert!(license_text.contains("GNU GENERAL PUBLIC LICENSE"));
        assert!(license_text.contains("azu"));
        assert!(license_text.contains("2018"));
    }
    #[test]
    fn create_ccby_license() {
        let ccby = create_license("ccby");
        let license_text = ccby.unwrap().notice(2018, "azu", "license-generator");
        assert!(license_text.contains("Attribution 4.0 International"));
        assert!(license_text.contains("azu"));
        assert!(license_text.contains("license-generator"));
    }
}
