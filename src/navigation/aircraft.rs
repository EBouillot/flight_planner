use std::{
    collections::HashMap, error::Error, path
};
use super::{
    database::{self, parse_database}, error::BalanceError, fuel::{self, Fuel}
};


#[derive(Debug, Clone)]
pub struct Balance {
    pub weigth: f64,
    pub arm: f64,
    pub moment: f64,
}

#[derive(Debug)]
pub struct Aircraft {
    pub immatriculation: String,
    pub aircraft_type: String,
    pub horse_power: i32,
    pub cruise_speed: f64,
    pub fuel: Fuel,
    pub consomatation: f64,
    pub nb_tank: i32,
    pub total_tank: i32,
    pub empty_weight: f64,
    pub balance_chart: BalanceChart,
    pub loading: BalanceSheet,
}
#[derive(Debug, Clone)]
pub struct BalanceCat {
    pub front_limit: f64,
    pub back_limit: f64,
    pub stab: f64,
    pub mindless_weight: f64,
    pub max_weight: f64,
}

impl BalanceCat {
    pub fn new() -> BalanceCat {
        BalanceCat {
            front_limit: 0.0,
            back_limit: 0.0,
            stab: 0.0,
            mindless_weight: 0.0,
            max_weight: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BalanceChart {
    pub catN: BalanceCat, //Normal operation
    pub catU: BalanceCat, //Utility operation
    pub catA: BalanceCat, //Acrobatic operation
}

impl BalanceChart {
    fn balance_mapper(row: &rusqlite::Row) -> rusqlite::Result<BalanceChart> {
        Ok(BalanceChart {
            catN: BalanceCat {
                front_limit: row.get(0)?,
                back_limit: row.get(1)?,
                stab: row.get(2)?,
                mindless_weight: row.get(3)?,
                max_weight: row.get(4)?,
            },
            catU: BalanceCat {
                front_limit: row.get(5)?,
                back_limit: row.get(6)?,
                stab: row.get(7)?,
                mindless_weight: row.get(8)?,
                max_weight: row.get(9)?,
            },
            catA: BalanceCat {
                front_limit: row.get(10)?,
                back_limit: row.get(11)?,
                stab: row.get(12)?,
                mindless_weight: row.get(13)?,
                max_weight: row.get(14)?,
            },
        })
    }

    pub fn from_database(database: &str, immatriculation: &str) -> Result<Vec<BalanceChart>, rusqlite::Error> {
        parse_database(&database, "balance_chart", "aircraft", &immatriculation, 
        "front_limit_N, back_limit_N, stab_N, mindless_weight_N, max_weight_N,
        front_limit_U, back_limit_U, stab_U, mindless_weight_U, max_weight_U,
        front_limit_A, back_limit_A, stab_A, mindless_weight_A, max_weight_A",
        Self::balance_mapper)
    }

}

#[derive(Debug, Clone)]
pub struct BalanceElement {
    pub arm: f64,
    pub weight: f64,
}

#[derive(Debug, Clone)]
pub struct BalanceSheet {
    pub elements: HashMap<String, BalanceElement>,
}


impl BalanceSheet {
    pub fn new() -> BalanceSheet {
        BalanceSheet {
            elements: HashMap::new(),
        }
    }

    pub fn load(&mut self, name: String,  weight: f64) -> Result<(), BalanceError> {
        if self.elements.get(&name).is_some() {
            if let Some(element) = self.elements.get_mut(&name) {
                element.weight = weight;
                Ok(())
            } else {
                Err(BalanceError::not_in_balance)
            }
        } else {
            Err(BalanceError::not_in_balance)
        }
    }

    pub fn total_weight(&self) -> f64 {
        self.elements.values().map(|b| b.weight).sum()
    }

    pub fn total_moment(&self) -> f64 {
        self.elements.iter().map(|(name, b)| b.weight * b.arm).sum()
    }

    pub fn center_of_gravity(&self) -> f64 {
        self.total_moment() / self.total_weight()
    }

    fn weight_mapper(row: &rusqlite::Row) -> rusqlite::Result<BalanceSheet> {
        let mut elements = HashMap::new();
        elements.insert("Empty".to_string(), BalanceElement{ arm: row.get(0)?, weight: 0.0});
        elements.insert("PIL".to_string(), BalanceElement{ arm: row.get(1)?, weight: 0.0});
        elements.insert("PAX".to_string(), BalanceElement{ arm: row.get(2)?, weight: 0.0});
        elements.insert("cargo".to_string(), BalanceElement{ arm: row.get(3)?, weight: 0.0});
        elements.insert("tank".to_string(), BalanceElement{ arm: row.get(4)?, weight: 0.0});
        elements.insert("tank2".to_string(), BalanceElement{ arm: row.get(5)?, weight: 0.0});
        elements.insert("tank3".to_string(), BalanceElement{ arm: row.get(6)?, weight: 0.0});
        Ok(BalanceSheet {
            elements,
        })
    }

    pub fn from_database(database: &str, immatriculation: &str) -> Result<Vec<BalanceSheet>, rusqlite::Error> {
        parse_database(&database, "weight_sheet", "Aircraft", &immatriculation, 
        "Empty, PIL, PAX, cargo, tank, tank2, tank3", Self::weight_mapper)

    }
}

pub fn max_allowed_weight(balance: BalanceCat, arm: f64) -> f64{
    if arm < balance.front_limit || arm > balance.back_limit {
        return 0.0;
    }
    let max_allowed_weight = if arm <= balance.stab {
        balance.mindless_weight + (balance.max_weight - balance.mindless_weight) * (arm - balance.front_limit) / (balance.stab - balance.front_limit)
    } else {
        balance.max_weight
    };
    max_allowed_weight
}

use plotters::{prelude::*, style::full_palette::{LIGHTBLUE, ORANGE_400}};



impl Aircraft {
    pub fn new(immatriculation: String, aircraft_type: String, horse_power: i32, cruise_speed: f64, fuel: Fuel, consomatation: f64, nb_tank: i32, total_tank: i32, empty_weight: f64, balance_chart: BalanceChart, loading: BalanceSheet) -> Aircraft {
        Aircraft {
            immatriculation,
            aircraft_type,
            horse_power,
            cruise_speed,
            fuel,
            consomatation,
            nb_tank,
            total_tank,
            empty_weight,
            balance_chart,
            loading,
        }
    }

    pub fn import(immatriculation: &str) -> Result<Aircraft, rusqlite::Error> {
        let mut plane = Aircraft::from_database("../../data/airports.db", immatriculation)?.pop().ok_or(rusqlite::Error::QueryReturnedNoRows)?;
        let mut balance = BalanceChart::from_database("../../data/airports.db", immatriculation)?;
        let mut weight = BalanceSheet::from_database("../../data/airports.db", immatriculation)?;
        if !weight.is_empty() {
            plane.loading = weight.pop().unwrap();
            plane.loading.load("Empty".to_string(), plane.empty_weight).unwrap();

        }
        if !balance.is_empty() {
            plane.balance_chart = balance.pop().unwrap();
        }
        Ok(plane)
    }


    
    fn aircraft_mapper(row: &rusqlite::Row) -> rusqlite::Result<Aircraft> {
        Ok(Aircraft {
            immatriculation: row.get(0)?,
            aircraft_type: row.get(1)?,
            horse_power: row.get(2)?,
            cruise_speed: row.get(3)?,
            fuel: match row.get::<_, String>(4)?.as_str() {
                "AVGAS100LL" => Fuel::AVGAS100LL,
                "AVGAS100" => Fuel::AVGAS100,
                "AVGAS82" => Fuel::AVGAS82,
                "AVGAS80" => Fuel::AVGAS80,
                "JET_A" => Fuel::JET_A,
                "JET_A1" => Fuel::JET_A1,
                "JET_B" => Fuel::JET_B,
                "MOGAS" => Fuel::MOGAS,
                _ => Fuel::AVGAS100LL,
            },
            consomatation: row.get(5)?,
            nb_tank: row.get(6)?,
            total_tank: row.get(7)?,
            empty_weight: row.get(8)?,
            balance_chart: BalanceChart {
                catN: BalanceCat::new(),
                catU: BalanceCat::new(),
                catA: BalanceCat::new(),
            },
            loading: BalanceSheet::new(),
        })
    }

    pub fn from_database(database: &str, immatriculation: &str) -> Result<Vec<Aircraft>, rusqlite::Error> {
        parse_database(&database, "aircrafts", "immat", &immatriculation, 
        "immat, type, horse_power, cruise_speed, fuel, conso, nb_tank, total_tank, empty_weight", Self::aircraft_mapper)
    }

    pub fn load_fuel(&mut self, quantity: f64) -> Result<(), BalanceError> {
        if self.total_tank < quantity as i32 {
            return Err(BalanceError::tank_capacity_exceeded(self.total_tank));
        }
        let fuel_weight = quantity * self.fuel.properties().density;
        self.loading.load("tank".to_string(), fuel_weight)?;
        Ok(())
    }

    pub fn load_crew(&mut self, weight: f64) -> Result<(), BalanceError> {
        self.loading.load("PIL".to_string(), weight)?;
        Ok(())
    }

    pub fn load_passengers(&mut self, weight: f64) -> Result<(), BalanceError> {
        self.loading.load("PAX".to_string(), weight)?;
        Ok(())
    }

    pub fn load_lugguage(&mut self, weight: f64) -> Result<(), BalanceError> {
        self.loading.load("cargo".to_string(), weight)?;
        Ok(())
    }

    /// Plot the maximum allowed weight curve for the aircraft
    /// Leave plane_weight and plane_arm to None will only print the envelope
    /// If plane_weight and plane_arm are provided, the calculated weight&balance will be plotted
    pub fn plot_max_allowed_weight_curve(&self, plane_weight: Option<f64>, plane_arm: Option<f64>) -> Result<(), Box<dyn Error>> {
        let file_name = format!("{}.png", self.immatriculation);
        let path = path::Path::new(&file_name);
        let root_area = BitMapBackend::new(path, (640, 480)).into_drawing_area();
        root_area.fill(&WHITE)?;
    
        let mut chart = ChartBuilder::on(&root_area)
            .caption(format!("Loading of {}  {}", self.aircraft_type, self.immatriculation), ("sans-serif", 50).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d((self.balance_chart.catN.front_limit - 0.2)..(self.balance_chart.catN.back_limit+0.2), 500.0..(self.balance_chart.catN.max_weight+50.0))?;
    
        chart.configure_mesh().draw()?;
    

        //Cat A
        let mut series = Vec::new();
        for arm in (180..=580).map(|x| x as f64 * 0.001) {
            let weight = max_allowed_weight(self.balance_chart.catA.clone(), arm);
            series.push((arm, weight));
        }

        chart.draw_series(LineSeries::new(series, &BLUE))?
            .label("Category A")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
        
        //Cat U
        let mut series = Vec::new();
        for arm in (180..=580).map(|x| x as f64 * 0.001) {
            let weight = max_allowed_weight(self.balance_chart.catU.clone(), arm);
            series.push((arm, weight));
        }

        chart.draw_series(LineSeries::new(series, &GREEN))?
            .label("Category U")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

        //Cat N
        let mut series = Vec::new();
        for arm in (180..=580).map(|x| x as f64 * 0.001) {
            let weight = max_allowed_weight(self.balance_chart.catN.clone(), arm);
            series.push((arm, weight));
        }

        chart.draw_series(LineSeries::new(series, &RED))?
            .label("Category N")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    
        if let (Some(weight), Some(arm)) = (plane_weight, plane_arm) {
            let mut color = &RED;
            if weight < max_allowed_weight(self.balance_chart.catA.clone(), arm) {
                color = &LIGHTBLUE;
            } 
            else if weight < max_allowed_weight(self.balance_chart.catU.clone(), arm) {
                println!("Weight {} at arm {} is within the envelope", weight, arm);
                color = &GREEN;
            } else if weight < max_allowed_weight(self.balance_chart.catN.clone(), arm) {
                color = &ORANGE_400;
            } else {
                println!("Weight {} at arm {} is outside the envelope", weight, arm);
            }
            chart.draw_series(PointSeries::of_element(vec![(plane_arm.unwrap(), weight)], 5, color, &|c, s, st| {
                return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
            }))?;
        }
    
        chart.configure_series_labels().background_style(&WHITE.mix(0.8)).border_style(&BLACK).draw()?;
    
        Ok(())
    }
}