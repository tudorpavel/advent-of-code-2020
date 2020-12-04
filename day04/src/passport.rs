use regex::Regex;

pub struct Passport {
    pub value: String,
}

impl Passport {
    const REQUIRED_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    pub fn has_required_fields(&self) -> bool {
        Passport::REQUIRED_FIELDS
            .iter()
            .all(|s| self.value.contains(s))
    }

    pub fn is_valid(&self) -> bool {
        self.valid_year("byr", 1920, 2002)
            && self.valid_year("iyr", 2010, 2020)
            && self.valid_year("eyr", 2020, 2030)
            && self.valid_height()
            && self.valid_hair_color()
            && self.valid_eye_color()
            && self.valid_passport_id()
    }

    fn valid_year(&self, key: &str, min: usize, max: usize) -> bool {
        let re = Regex::new(&format!(r"{}:(\d+)", key)).unwrap();

        if !re.is_match(&self.value) {
            return false;
        }

        let cap = re.captures(&self.value).unwrap();
        let year: usize = cap[1].parse().unwrap();

        min <= year && year <= max
    }

    fn valid_height(&self) -> bool {
        let re = Regex::new(r"hgt:(\d+)(in|cm)").unwrap();

        if !re.is_match(&self.value) {
            return false;
        }

        let cap = re.captures(&self.value).unwrap();
        let height: usize = cap[1].parse().unwrap();
        let unit: String = cap[2].to_string();

        if unit == "cm" {
            150 <= height && height <= 193
        } else {
            59 <= height && height <= 76
        }
    }

    fn valid_hair_color(&self) -> bool {
        let re = Regex::new(r"hcl:#([\da-f]+)").unwrap();

        if !re.is_match(&self.value) {
            return false;
        }

        let cap = re.captures(&self.value).unwrap();
        let hex: String = cap[1].to_string();

        hex.len() == 6
    }

    fn valid_eye_color(&self) -> bool {
        let re = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)").unwrap();

        re.is_match(&self.value)
    }

    fn valid_passport_id(&self) -> bool {
        let re = Regex::new(r"pid:(\d+)").unwrap();

        if !re.is_match(&self.value) {
            return false;
        }

        let cap = re.captures(&self.value).unwrap();
        let pid: String = cap[1].to_string();

        pid.len() == 9
    }
}
