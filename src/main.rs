mod navigation;
use navigation::aircraft::{self, Aircraft, BalanceChart};
use navigation::airport::Airport;
use navigation::flightplan::NavBranch;
use navigation::geographics::NavPoint;
use navigation::database;
use serde::de::value::U128Deserializer;

#[tokio::main]
async fn main() {
    let stcyr = Airport::from_oaci_code("LFPZ").unwrap();
    let toussus = Airport::from_oaci_code("LFPN").unwrap();
    

    println!(
        "Let's try : {}, lat : {}, lon : {}",
        stcyr.name, stcyr.latitude, stcyr.longitude
    );
    println!(
        "Let's try : {}, lat : {}, lon : {}",
        toussus.name, toussus.latitude, toussus.longitude
    );

    let nav_start = NavPoint::new("lfpz".to_string(), stcyr.latitude, stcyr.longitude);
    let nav_end = NavPoint::new("lfpn".to_string(), toussus.latitude, toussus.longitude);

    let (dist, course) = NavBranch::calculate_distance_and_course(&nav_start, &nav_end);
    println!(
        "Distance : {} NM, course : {}°",
        dist.unwrap() * 0.539957,
        course.unwrap()
    );

    let cg = Aircraft::import("F-HFCG").unwrap();
    let rv = Aircraft::import("F-HARV").unwrap();
    println!("Aircraft : {:?}", cg);
    println!("Aircraft : {:?}", rv);

    cg.plot_max_allowed_weight_curve(Some(870.0), Some(0.51))
        .unwrap();
    rv.plot_max_allowed_weight_curve(Some(920.0), Some(0.48))
        .unwrap();


}