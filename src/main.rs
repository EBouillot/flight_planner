mod navigation;
use navigation::{database, aircraft::{self, Aircraft}, airport::Airport, flightplan::NavBranch, geographics::{calculate_distance_and_course, NavPoint}};
/*
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


}*/

slint::include_modules!();

use std::rc::Rc;
use std::cell::RefCell;
use slint::{ModelRc, SharedString, VecModel};


fn main () {
    let main_window = MainWindow::new().unwrap();
    let list_aircraft = database::list_entries("../../data/airports.db", "aircrafts", "immat").unwrap();
    let aircraft_shrd: Vec<SharedString> = list_aircraft.into_iter().map(SharedString::from).collect();
    let aircraft_model = ModelRc::new(VecModel::from(aircraft_shrd.clone()));
    main_window.set_aircraft_list(aircraft_model);
    let main_window_weak = main_window.as_weak();

    let aircraft_view = Rc::new(RefCell::new(main_window.get_aircraft()));
    if let Some(first_aircraft) = aircraft_shrd.first() {
        main_window.set_selected_aircraft(first_aircraft.clone());
    }
    let aircraft = Aircraft::import(&main_window.get_selected_aircraft()).unwrap();

    main_window.on_aircraft_changed(move || {
        println!("Aircraft changed");
        let new_aircraft = main_window_weak.unwrap().get_selected_aircraft();
        println!("Selected Aircraft: {:?}", new_aircraft);
    });
    
    let main_window_weak = main_window.as_weak();
    main_window.on_balance(move || {
        println!("Balance clicked");
        let toto = Aircraft::import(&main_window_weak.unwrap().get_selected_aircraft()).unwrap();
        
        
    });


    
/*
    main_window.on_select_aircraft(move || {
        let main_window = main_window_weak.unwrap();
        let txt = main_window.get_aircraft_name();
        println!("Selected Aircraft: {}", txt);
        let aircraft = Aircraft::import(&txt).unwrap();
        println!("Aircraft data : {:?}", aircraft);
        let mut aircraft_view_mut = aircraft_view.borrow_mut();
        aircraft_view_mut.name = aircraft.immatriculation.into();
        aircraft_view_mut.power = aircraft.horse_power;
        aircraft_view_mut.aircraft_type = aircraft.aircraft_type.into();
        main_window.set_aircraft(aircraft_view_mut.clone());
}));
*/

    main_window.run().unwrap();
}