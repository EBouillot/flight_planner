use std::{
    collections::HashMap, error::Error, fs::File, io::{BufRead, BufReader}, path
};
use super::{
    error::ParseError,
    fuel::{self, Fuel},
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
    pub tank: i32,
    pub balance_chart: BalanceChart,
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

#[derive(Debug, Clone)]
pub struct BalanceSheet {
    pub elements: HashMap<String, Balance>,
}

impl BalanceSheet {
    pub fn new() -> BalanceSheet {
        BalanceSheet {
            elements: HashMap::new(),
        }
    }

    pub fn load(&mut self, name: String, weight: f64, arm: f64) {
        self.elements.insert(name, Balance {
            weigth: weight,
            arm: arm,
            moment: weight * arm,
        });
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

use plotters::{prelude::*, style::full_palette::{LIGHTBLUE, ORANGE, ORANGE_400}};



impl Aircraft {
    pub fn new(immatriculation: String, aircraft_type: String, horse_power: i32, cruise_speed: f64, fuel: Fuel, consomatation: f64, tank: i32, balance_chart: BalanceChart) -> Aircraft {
        Aircraft {
            immatriculation,
            aircraft_type,
            horse_power,
            cruise_speed,
            fuel,
            consomatation,
            tank,
            balance_chart,
        }
    }

    pub fn import(immatriculation: String) -> Result<Aircraft, ParseError> {
        Aircraft::from_csv("../../aircrafts.csv", &immatriculation)

    }
    pub fn from_csv(file_name: &str, name: &str) -> Result<Aircraft, ParseError>{
        println!("*** DEBUG  : file_name : {}, name : {}", file_name, name);
        let file = File::open(file_name).map_err(|_| ParseError::file_not_found)?;
        let reader = BufReader::new(file);

        let mut immat: String = String::new();
        let mut aircraft_type: String = String::new();
        let mut horse_power: i32 = 0;
        let mut cruise_speed: f64 = 0.0;
        let mut fuel: Fuel = Fuel::AVGAS100LL;
        let mut conso: f64 = 0.0;
        let mut tank: i32 = 0;
        let mut cat_n = BalanceCat::new();
        let mut cat_u = BalanceCat::new();
        let mut cat_a = BalanceCat::new();
        let mut balance_chart = BalanceChart {
            catN: cat_n,
            catU: cat_u,
            catA: cat_a,
        };

        for line in reader.lines() {
            let line = line.map_err(|_| ParseError::unknown_error)?;
            let fields: Vec<&str> = line.split(',').collect();

            if fields[0] == name {
                println!("Found aircraft : {}", name);
                println!("Fields : {:?}", fields);
                immat = fields[0].to_string();
                aircraft_type = fields[1].to_string();
                horse_power = fields[2].parse::<i32>().map_err(|_| ParseError::wrong_format)?;
                cruise_speed = fields[3].parse::<f64>().map_err(|_| ParseError::wrong_format)?;
                fuel = match fields[4] {
                    "AVGAS100LL" => Fuel::AVGAS100LL,
                    "AVGAS100" => Fuel::AVGAS100,
                    "AVGAS82" => Fuel::AVGAS82,
                    "AVGAS80" => Fuel::AVGAS80,
                    "JET_A" => Fuel::JET_A,
                    "JET_A1" => Fuel::JET_A1,
                    "JET_B" => Fuel::JET_B,
                    "MOGAS" => Fuel::MOGAS,
                    _ => Fuel::AVGAS100LL,
                };
                conso = fields[5].parse::<f64>().map_err(|_| ParseError::wrong_format)?;
                tank = fields[6].parse::<i32>().map_err(|_| ParseError::wrong_format)?;
                balance_chart.catN = BalanceCat {
                    front_limit: fields[7].parse::<f64>().map_err(|_| ParseError::wrong_format)?,
                    back_limit: fields[8].parse::<f64>().map_err(|_| ParseError::wrong_format)?,
                    stab: fields[9].parse::<f64>().map_err(|_| ParseError::wrong_format)?,
                    mindless_weight: fields[10].parse::<f64>().map_err(|_| ParseError::wrong_format)?,
                    max_weight: fields[11].parse::<f64>().map_err(|_| ParseError::wrong_format)?,
                };
                balance_chart.catU = BalanceCat {
                    front_limit: fields[12].parse::<f64>().map_err(|_| ParseError::wrong_format)?,
                    back_limit: fields[13].parse::<f64>().map_err(|_| ParseError::wrong_format)?,
                    stab: fields[14].parse::<f64>().map_err(|_| ParseError::wrong_format)?,
                    mindless_weight: fields[15].parse::<f64>().map_err(|_| ParseError::wrong_format)?,
                    max_weight: fields[16].parse::<f64>().map_err(|_| ParseError::wrong_format)?,
                };
                balance_chart.catA = BalanceCat {
                    front_limit: fields[17].parse::<f64>().map_err(|_| ParseError::wrong_format)?,
                    back_limit: fields[18].parse::<f64>().map_err(|_| ParseError::wrong_format)?,
                    stab: fields[19].parse::<f64>().map_err(|_| ParseError::wrong_format)?,
                    mindless_weight: fields[20].parse::<f64>().map_err(|_| ParseError::wrong_format)?,
                    max_weight: fields[21].parse::<f64>().map_err(|_| ParseError::wrong_format)?,
                };
            }
        }
        Ok (Aircraft::new(immat, aircraft_type, horse_power, cruise_speed, fuel, conso, tank, balance_chart))
    }

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
            else if weight < max_allowed_weight(self.balance_chart.catN.clone(), arm) {
                println!("Weight {} at arm {} is within the envelope", weight, arm);
                color = &ORANGE_400;
            } else if weight < max_allowed_weight(self.balance_chart.catU.clone(), arm) {
                color = &GREEN;
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

