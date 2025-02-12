
mod navigation;
use navigation::airport::Airport;
use navigation::geographics::NavPoint;
use navigation::flightplan::NavBranch;
use navigation::aircraft::{self, Aircraft};
use plotters::data::float;

#[tokio::main]
async fn main() {
    //let stcyr = NavPoint::from_oaci_code("LFPZ").await.unwrap();
    //println!("Let's try : {}, lat : {}, lon : {}", stcyr.name, stcyr.latitude, stcyr.longitude);
    let stcyr = Airport::from_oaci_code("LFPZ").unwrap();
    let toussus = Airport::from_oaci_code("LFPN").unwrap();
    println!("Let's try : {}, lat : {}, lon : {}", stcyr.name, stcyr.latitude, stcyr.longitude);
    println!("Let's try : {}, lat : {}, lon : {}", toussus.name, toussus.latitude, toussus.longitude);

    let nav_start= NavPoint::new("lfpz".to_string(), stcyr.latitude, stcyr.longitude);
    let nav_end = NavPoint::new("lfpn".to_string(), toussus.latitude, toussus.longitude);

    let (dist, course) = NavBranch::calculate_distance_and_course(&nav_start, &nav_end);
    println!("Distance : {} NM, course : {}°", dist.unwrap()*0.539957, course.unwrap());

    println!("**** Mass and balance ****");
    let balance = aircraft::BalanceChart {
        front_limit: 0.205,
        back_limit: 0.564,
        stab: 0.428,
        mindless_weight: 750.0,
        max_weight: 900.0,
    };
    let mut max_weight: f64;

    for i in (180..=580).map(|x| x as f64 * 0.001) {
        max_weight = aircraft::max_allowed_weight(balance.clone(), i);
    }

    aircraft::plot_max_allowed_weight_curve(balance, Some(890.0), Some(0.208)).unwrap();

}
