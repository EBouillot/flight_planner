use std::error::Error;

struct Balance {
    pub station: String,
    pub weigth: f64,
    pub arm: f64,
    pub moment: f64,
}

pub struct Aircraft {
    pub aircraft_type: String,
    pub call_sign: String,
    pub weight: f64,
    pub cruise_speed: f64,
}
#[derive(Debug, Clone)]
pub struct BalanceChart {
    pub front_limit: f64,
    pub back_limit: f64,
    pub stab: f64,
    pub mindless_weight: f64,
    pub max_weight: f64,
}

pub fn max_allowed_weight(balance: BalanceChart, arm: f64) -> f64{
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

use plotters::prelude::*;

pub fn plot_max_allowed_weight_curve(balance: BalanceChart, plane_weight: Option<f64>, plane_arm: Option<f64>) -> Result<(), Box<dyn Error>> {
    let root_area = BitMapBackend::new("max_allowed_weight_curve.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Loading of DR400-120 F-HFCG", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d((balance.front_limit - 0.2)..(balance.back_limit+0.2), 500.0..(balance.max_weight+50.0))?;

    chart.configure_mesh().draw()?;

    let mut series = Vec::new();
    for arm in (180..=580).map(|x| x as f64 * 0.001) {
        let weight = max_allowed_weight(balance.clone(), arm);
        series.push((arm, weight));
    }

    chart.draw_series(LineSeries::new(series, &BLUE))?
        .label("Max Allowed Weight")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    if let (Some(weight), Some(arm)) = (plane_weight, plane_arm) {
        let mut color = &RED;
        if weight < max_allowed_weight(balance, arm) {
            println!("Weight {} at arm {} is within the envelope", weight, arm);
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


impl Aircraft {


    fn enveloppe_calculator(weight: f64, arm: f64, front_limit: f64, back_limit: f64, stab: f64, mindless_weight: f64, max_weight: f64) -> Result<bool, Box<dyn Error>> {
        if arm < front_limit || arm > back_limit {
            return Ok(false);
        }

        let max_allowed_weight = if arm <= stab {
            mindless_weight + (max_weight - mindless_weight) * (arm - front_limit) / (stab - front_limit)
        } else {
            max_weight
        };

        Ok(weight <= max_allowed_weight)
    }


}

