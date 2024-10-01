pub trait License {
    fn notice(&self, year: u32, name: &str, project: &str) -> String;
}

pub const LICENSES: [&str; 17] = [
    "AGPL",
    "Apache",
    "BSD",
    "CC0",
    "CC-BY",
    "CC-BY-NC",
    "CC-BY-NC-SA",
    "CC-BY-SA",
    "EUPL-1.2",
    "GPL-2",
    "GPL-3",
    "ISC",
    "LGPL-3",
    "MIT",
    "MPL-2",
    "Unlicense",
    "WTFPL"
];

// agpl-3.0.txt
pub struct AGPL {}

impl License for AGPL {
    fn notice(&self, year: u32, name: &str, project: &str) -> String {
        format!(
            include_str!("../files/agpl-3.0.txt"),
            YEAR = year,
            AUTHOR = name,
            PROJECT = project
        )
    }
}

// apache-2.0.txt
pub struct Apache {}

impl License for Apache {
    fn notice(&self, year: u32, name: &str, _project: &str) -> String {
        format!(
            include_str!("../files/apache-2.0.txt"),
            YEAR = year,
            AUTHOR = name
        )
    }
}

// bsd-3.0.txt
pub struct BSD {}

impl License for BSD {
    fn notice(&self, year: u32, name: &str, _project: &str) -> String {
        format!(
            include_str!("../files/bsd-3.0.txt"),
            YEAR = year,
            AUTHOR = name
        )
    }
}

// cc-by-4.0.txt
pub struct CcBy {}

impl License for CcBy {
    fn notice(&self, _year: u32, name: &str, project: &str) -> String {
        format!(
            include_str!("../files/cc-by-4.0.txt"),
            AUTHOR = name,
            PROJECT = project
        )
    }
}

// cc-by-nc-4.0.txt
pub struct CcByNc {}

impl License for CcByNc {
    fn notice(&self, _year: u32, name: &str, project: &str) -> String {
        format!(
            include_str!("../files/cc-by-nc-4.0.txt"),
            AUTHOR = name,
            PROJECT = project
        )
    }
}

// cc-by-nc-sa-4.0.txt
pub struct CcByNcSa {}

impl License for CcByNcSa {
    fn notice(&self, _year: u32, name: &str, project: &str) -> String {
        format!(
            include_str!("../files/cc-by-nc-sa-4.0.txt"),
            AUTHOR = name,
            PROJECT = project
        )
    }
}

// cc-by-sa-4.0.txt
pub struct CcBySa {}

impl License for CcBySa {
    fn notice(&self, _year: u32, name: &str, project: &str) -> String {
        format!(
            include_str!("../files/cc-by-sa-4.0.txt"),
            AUTHOR = name,
            PROJECT = project
        )
    }
}

// cc-zero-1.0.txt
pub struct CCZero {}

impl License for CCZero {
    fn notice(&self, _year: u32, name: &str, _project: &str) -> String {
        format!(include_str!("../files/cc-zero-1.0.txt"), AUTHOR = name)
    }
}

// gpl-3.0.txt
pub struct GPL {}

impl License for GPL {
    fn notice(&self, year: u32, name: &str, project: &str) -> String {
        format!(
            include_str!("../files/gpl-3.0.txt"),
            YEAR = year,
            AUTHOR = name,
            PROJECT = project
        )
    }
}

// gpl-2.0.txt
pub struct GPL2 {}

impl License for GPL2 {
    fn notice(&self, year: u32, name: &str, project: &str) -> String {
        format!(
            include_str!("../files/gpl-2.0.txt"),
            YEAR = year,
            AUTHOR = name,
            PROJECT = project
        )
    }
}

// isc.txt
pub struct ISC {}

impl License for ISC {
    fn notice(&self, year: u32, name: &str, _project: &str) -> String {
        format!(include_str!("../files/isc.txt"), YEAR = year, AUTHOR = name)
    }
}

// lgpl-3.0.txt
pub struct LGPL {}

impl License for LGPL {
    fn notice(&self, _year: u32, _name: &str, _project: &str) -> String {
        format!(
            include_str!("../files/lgpl-3.0.txt")
        )
    }
}

// mit.txt
pub struct Mit {}

impl License for Mit {
    fn notice(&self, year: u32, name: &str, _project: &str) -> String {
        format!(include_str!("../files/mit.txt"), YEAR = year, AUTHOR = name)
    }
}

// mpl-2.0.txt
pub struct MPL {}

impl License for MPL {
    fn notice(&self, year: u32, name: &str, _project: &str) -> String {
        format!(
            include_str!("../files/mpl-2.0.txt"),
            YEAR = year,
            AUTHOR = name,
        )
    }
}

// unlicense.txt
pub struct UNLICENSE {}

impl License for UNLICENSE {
    fn notice(&self, _year: u32, _name: &str, _project: &str) -> String {
        include_str!("../files/unlicense.txt").to_string()
    }
}

// eupl-1.2_en.txt
pub struct EUPL {}

impl License for EUPL {
    fn notice(&self, _year: u32, _name: &str, _project: &str) -> String {
        include_str!("../files/eupl-1.2_en.txt").to_string()
    }
}

// WTFPL.txt
pub struct WTFPL {}

impl License for WTFPL {
    fn notice(&self, year: u32, name: &str, _project: &str) -> String {
        format!(include_str!("../files/WTFPL.txt"), YEAR = year, AUTHOR = name)
    }
}
