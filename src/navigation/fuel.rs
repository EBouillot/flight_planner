pub struct FuelProperties {
    pub name: String,
    pub density: f64,
}

#[derive(Debug)]
pub enum Fuel {
    AVGAS100LL,
    AVGAS100,
    AVGAS82,
    AVGAS80,
    JET_A,
    JET_A1,
    JET_B,
    MOGAS,
}

impl Fuel {
    pub fn properties(&self) -> FuelProperties {
        match self {
            Fuel::AVGAS100LL => FuelProperties {
                name: "AVGAS 100LL".to_string(),
                density: 0.721,
            },
            Fuel::AVGAS100 => FuelProperties {
                name: "AVGAS 100".to_string(),
                density: 0.721,
            },
            Fuel::AVGAS80 => FuelProperties {
                name: "AVGAS 80".to_string(),
                density: 0.721,
            },
            Fuel::AVGAS82 => FuelProperties {
                name: "AVGAS 82".to_string(),
                density: 0.690,
            },
            Fuel::JET_A => FuelProperties {
                name: "JET A".to_string(),
                density: 0.804,
            },
            Fuel::JET_A1 => FuelProperties {
                name: "JET A-1".to_string(),
                density: 0.804,
            },
            Fuel::JET_B => FuelProperties {
                name: "JET B".to_string(),
                density: 0.820,
            },
            Fuel::MOGAS => FuelProperties {
                name: "MOGAS".to_string(),
                density: 0.745,
            },
        }
    }
}


