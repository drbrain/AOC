use anyhow::Result;

use aoc2020::read;

use std::convert::From;

fn main() -> Result<()> {
    let input = read("./04.input")?;

    println!("part A: {}", day_4_a(&input)?);
    println!("part B: {}", day_4_b(&input)?);

    Ok(())
}

fn day_4_a(passports: &str) -> Result<usize> {
    let valid = passports
        .split("\n\n")
        .map(Passport::from)
        .filter(|passport| passport.is_valid())
        .count();

    Ok(valid)
}

fn day_4_b(passports: &str) -> Result<usize> {
    let valid = passports
        .split("\n\n")
        .map(Passport::from)
        .filter(|passport| passport.is_extra_valid())
        .count();

    Ok(valid)
}

#[derive(Debug)]
enum Unit {
    Inches,
    Centimeters,
    None,
}

#[derive(Debug, Default)]
struct Passport {
    birth_year: Option<u32>,
    issue_year: Option<u32>,
    expiration_year: Option<u32>,
    height: Option<(u32, Unit)>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<u32>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn is_extra_valid(&self) -> bool {
        if let Some(birth_year) = self.birth_year {
            if !(1920..2003).contains(&birth_year) {
                return false;
            }
        } else {
            return false;
        }

        if let Some(issue_year) = self.issue_year {
            if !(2010..2021).contains(&issue_year) {
                return false;
            }
        } else {
            return false;
        }

        if let Some(expiration_year) = self.expiration_year {
            if !(2020..2031).contains(&expiration_year) {
                return false;
            }
        } else {
            return false;
        }

        if let Some((measurement, unit)) = &self.height {
            match unit {
                Unit::Centimeters => {
                    if !(150..194).contains(measurement) {
                        return false;
                    }
                }
                Unit::Inches => {
                    if !(59..77).contains(measurement) {
                        return false;
                    }
                }
                Unit::None => {
                    return false;
                }
            }
        } else {
            return false;
        }

        if let Some(hair_color) = &self.hair_color {
            if hair_color.len() != 7
                || &hair_color[0..1] != "#"
                || !hair_color[1..7].chars().all(|c| c.is_ascii_hexdigit())
            {
                return false;
            }
        } else {
            return false;
        }

        if let Some(eye_color) = &self.eye_color {
            match eye_color.as_str() {
                "amb" => (),
                "blu" => (),
                "brn" => (),
                "gry" => (),
                "grn" => (),
                "hzl" => (),
                "oth" => (),
                _ => {
                    return false;
                }
            }
        } else {
            return false;
        }

        if let Some(passport_id) = &self.passport_id {
            if passport_id.len() != 9 || !passport_id[1..9].chars().all(|c| c.is_ascii_digit()) {
                return false;
            }
        } else {
            return false;
        }

        true
    }
}

impl From<&str> for Passport {
    fn from(string: &str) -> Self {
        let mut passport = Passport::default();

        let pairs: Vec<&str> = string.split(|c| c == ' ' || c == '\n').collect();

        for pair in pairs {
            let parts: Vec<&str> = pair.split(':').collect();
            let key = parts[0];
            let value = parts[1];

            match key {
                "byr" => {
                    passport.birth_year = Some(value.parse::<u32>().unwrap());
                }
                "iyr" => {
                    passport.issue_year = Some(value.parse::<u32>().unwrap());
                }
                "eyr" => {
                    passport.expiration_year = Some(value.parse::<u32>().unwrap());
                }
                "hgt" => {
                    let value = match value.find(char::is_alphabetic) {
                        Some(index) => {
                            let (height, unit) = value.split_at(index);

                            let height = height.parse::<u32>().unwrap();
                            let unit = match unit {
                                "cm" => Unit::Centimeters,
                                "in" => Unit::Inches,
                                "" => Unit::None,
                                _ => Unit::None,
                            };

                            (height, unit)
                        }
                        None => (value.parse::<u32>().unwrap(), Unit::None),
                    };

                    passport.height = Some(value);
                }
                "hcl" => {
                    passport.hair_color = Some(value.into());
                }
                "ecl" => {
                    passport.eye_color = Some(value.into());
                }
                "pid" => {
                    passport.passport_id = Some(value.into());
                }
                "cid" => {
                    passport.country_id = Some(value.parse::<u32>().unwrap());
                }
                _ => {
                    panic!("unknown field {:?}", key);
                }
            }
        }

        passport
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_4_a() {
        let input = String::from(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in",
        );

        assert_eq!(2, day_4_a(&input).unwrap());
    }

    #[test]
    fn test_is_extra_valid() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f";

        let passport = Passport::from(input);

        assert_eq!(true, passport.is_extra_valid());

        let input = "eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm";

        let passport = Passport::from(input);

        assert_eq!(true, passport.is_extra_valid());

        let input = "hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022";

        let passport = Passport::from(input);

        assert_eq!(true, passport.is_extra_valid());

        let input = "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let passport = Passport::from(input);

        assert_eq!(true, passport.is_extra_valid());

        let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926";

        let passport = Passport::from(input);

        assert_eq!(false, passport.is_extra_valid());

        let input = "iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946";

        let passport = Passport::from(input);

        assert_eq!(false, passport.is_extra_valid());

        let input = "hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277";

        let passport = Passport::from(input);

        assert_eq!(false, passport.is_extra_valid());

        let input = "hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        let passport = Passport::from(input);

        assert_eq!(false, passport.is_extra_valid());
    }
}
