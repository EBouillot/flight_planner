use std::error::{self, Error};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;

#[derive(Debug)]
pub enum AirportType {
    small_airport,
    medium_airport,
    large_airport,
    helliport,
    closed,
    unknown,
}
#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("File not found")]
    file_not_found,
    #[error("Wrong file format")]
    wrong_format,
    #[error("Field not found")]
    field_not_found,
    #[error("Unknown error")]
    unknown_error,
}

#[derive(Debug)]
pub struct Airport {
    pub oaci_code: String,
    pub name: String,
    pub airport_type: AirportType,
    pub latitude: f64,
    pub longitude: f64,
}

impl Airport {
    pub fn new(oaci_code: String, name: String, airport_type: AirportType, latitude: f64, longitude: f64) -> Airport {
        Airport {
            oaci_code,
            name,
            airport_type,
            latitude,
            longitude,
        }
    }

    pub fn from_oaci_code(oaci_code: &str) -> Result<Airport, ParseError> {
        Self::from_csv("../../fr-airports.csv", oaci_code)
    }

    fn from_csv(file_path: &str, ident: &str) -> Result<Airport, ParseError> {
        let file = File::open(file_path).map_err(|_| ParseError::file_not_found)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.map_err(|_| ParseError::unknown_error)?;
            let fields: Vec<&str> = line.split(',').collect();

            if fields[1] == ident {
                let oaci_code = fields[1].to_string();
                let name = fields[3].to_string();
                let airport_type = match fields[2] {
                    "small_airport" => AirportType::small_airport,
                    "medium_airport" => AirportType::medium_airport,
                    "large_airport" => AirportType::large_airport,
                    "helliport" => AirportType::helliport,
                    "closed" => AirportType::closed,
                    _ => AirportType::unknown,
                };
                let latitude = fields[4].parse::<f64>().map_err(|_| ParseError::wrong_format)?;
                let longitude = fields[5].parse::<f64>().map_err(|_| ParseError::wrong_format)?;
                return Ok(Airport::new(oaci_code, name, airport_type, latitude, longitude));
            }
        }

        Err(ParseError::field_not_found)
    }
}