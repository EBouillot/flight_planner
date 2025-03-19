use super::error::ParseError;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rusqlite::Connection;
use super::database;

#[derive(Debug)]
pub enum AirportType {
    small_airport,
    medium_airport,
    large_airport,
    helliport,
    closed,
    unknown,
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
    pub fn new(
        oaci_code: String,
        name: String,
        airport_type: AirportType,
        latitude: f64,
        longitude: f64,
    ) -> Airport {
        Airport {
            oaci_code,
            name,
            airport_type,
            latitude,
            longitude,
        }
    }

    pub fn from_oaci_code(oaci_code: &str) -> Result<Airport, rusqlite::Error> {
        Self::from_db("../../data/airports.db", oaci_code)?
            .pop().ok_or(rusqlite::Error::QueryReturnedNoRows)
    }

    fn airport_mapper(row: &rusqlite::Row) -> rusqlite::Result<Airport> {
        Ok(Airport {
            oaci_code: row.get(0)?,
            name: row.get(1)?,
            airport_type: match row.get::<_, String>(2)?.as_str() {
                "small_airport" => AirportType::small_airport,
                "medium_airport" => AirportType::medium_airport,
                "large_airport" => AirportType::large_airport,
                "helliport" => AirportType::helliport,
                "closed" => AirportType::closed,
                _ => AirportType::unknown,
            },
            latitude: row.get(3)?,
            longitude: row.get(4)?,
        })
    }

    pub fn from_db(db_path: &str, ident: &str) -> Result<Vec<Airport>, rusqlite::Error> {
        database::parse_database(db_path, "airports", "ident", ident, "ident, name, type, latitude_deg, longitude_deg", Self::airport_mapper)
    }
}