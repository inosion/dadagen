
use rand::Rng;
use std::collections::HashMap;
use regex::Regex;

pub enum CountryCode {
    GBR,
    AUS,
    USA,
    NZD,
    IND,
}

impl CountryCode {
    fn country_code(&self) -> &str {
        match self {
            CountryCode::GBR => "GBR",
            CountryCode::AUS => "AUS",
            CountryCode::USA => "USA",
            CountryCode::NZD => "NZD",
            CountryCode::IND => "IND",
        }
    }
}


pub struct Context {
    // Define the context fields here
    data_field_state: HashMap<String, String>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            data_field_state: HashMap::new(),
        }
    }

    pub fn insert_data_field_state(&mut self, key: String, value: String) {
        self.data_field_state.insert(key, value);
    }

    pub fn get_data_field_state(&self, key: &str) -> Option<&String> {
        self.data_field_state.get(key)
    }
}

pub enum AddressStyle {
    StreetWithNumber,
    StreetOnly,
    SuburbCityDistrict,
    SuburbOnly,
    CityOnly,
    DistrictOnly,
    SuburbCity,
    PostCodeOnly,
    CountryOnly,
    FullAddress,
}

pub struct AddressGenerator {
    name: String,
    style: AddressStyle,
    country_field: Option<CountryCode>, // Add the country_field    
}

impl AddressGenerator {
    pub fn new(name: String, style: AddressStyle, country_field: Option<CountryCode>) -> Self {
        Self { name, style, country_field }
    }

    pub fn internal_generate(&self, _context: &Context) -> String {
        match self.style {
            AddressStyle::StreetWithNumber => self.street_line(),
            AddressStyle::StreetOnly => self.street_name(),
            AddressStyle::SuburbCityDistrict => self.suburb_city_district(),
            AddressStyle::SuburbOnly => self.get_random_value("address.suburb"),
            AddressStyle::CityOnly => self.get_random_value("places.cities"),
            AddressStyle::DistrictOnly => self.get_random_value("address.district"),
            AddressStyle::SuburbCity => self.suburb_city(),
            AddressStyle::PostCodeOnly => self.postcode(),
            AddressStyle::CountryOnly => self.get_random_value("places.countries"),
            AddressStyle::FullAddress => {
                format!(
                    "{}, {} {} {}",
                    self.street_line(),
                    self.suburb_city_district(),
                    self.get_random_value("places.countries").to_uppercase(),
                    self.postcode()
                )
            }
        }
    }

    fn suburb_city(&self) -> String {
        format!(
            "{}, {}",
            self.get_random_value("address.suburb"),
            self.get_random_value("places.cities")
        )
    }

    fn suburb_city_district(&self) -> String {
        format!(
            "{}, {}",
            self.suburb_city(),
            self.get_random_value("address.district")
        )
    }

    fn street_name(&self) -> String {
        format!(
            "{} {}",
            self.get_random_value("address.street.name"),
            self.get_random_value("address.street.suffix")
        )
    }

    fn street_line(&self) -> String {
        format!("{} {}", self.house_name_number(), self.street_name())
    }

    fn house_name_number(&self) -> String {
        match rand::thread_rng().gen_range(1..=20) {
            1 => format!("{},", self.get_random_value("address.housename")),
            _ => format!(
                "{}{}",
                rand::thread_rng().gen_range(1..=500),
                match rand::thread_rng().gen_range(1..=40) {
                    1 => rand::thread_rng().gen_range('A'..='C').to_string(),
                    _ => "".to_string(),
                }
            ),
        }
    }

    fn postcode(&self) -> String {
        match self.country_field {
            Some(CountryCode::GBR) => self.uk_postcode_gen(),
            _ => rand::thread_rng().gen_range(55000..=56000).to_string(),
        }
    }

    fn uk_postcode_gen(&self) -> String {
        let re = Regex::new(r"[A-HJ-NP-Z]{2}[1-9][0-9] [0-9][A-HJ-NP-Z]{2}").unwrap();
        re.captures(rand::thread_rng().gen_range(0..9999).to_string().as_str())
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .to_string()
    }

    fn get_random_value(&self, _key: &str) -> String {
        // You need to implement a function to get random values based on the key.
        // For simplicity, we'll return an empty string for now.
        "".to_string()
    }
}
