mod navigation;
use navigation::{aircraft::Aircraft, airport::Airport, flightplan::NavBranch, geographics::{NavPoint, calculate_distance_and_course}};

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

    let (dist, course) = calculate_distance_and_course(&nav_start, &nav_end);
    println!(
        "Distance : {} NM, course : {}°",
        dist.unwrap() * 0.539957,
        course.unwrap()
    );

    let mut cg = Aircraft::import("F-HFCG").unwrap();
    let rv = Aircraft::import("F-HARV").unwrap();

    if let Err(e) = cg.load_fuel(110.0) {
        eprintln!("Error loading fuel: {}", e);
    }
    cg.load_crew(150.0);
    cg.load_passengers(0.0);
    cg.load_lugguage(0.0);

    println!("Aircraft : {:?}", cg);

    cg.plot_max_allowed_weight_curve(Some(cg.loading.total_weight()), Some(cg.loading.center_of_gravity()))
        .unwrap();


}