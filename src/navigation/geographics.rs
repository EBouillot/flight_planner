/// Represents a navigation point with a name, latitude, and longitude.
///
/// # Fields
/// - `name`: A `String` representing the name of the navigation point.
/// - `latitude`: A `f64` representing the latitude of the navigation point in degrees.
/// - `longitude`: A `f64` representing the longitude of the navigation point in degrees.
///
/// # Methods
/// - `new(name: String, latitude: f64, longitude: f64) -> NavPoint`: Creates a new `NavPoint` instance with the given name, latitude, and longitude in radians.
pub struct NavPoint {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl NavPoint {
    pub fn new(name: String, latitude: f64, longitude: f64) -> NavPoint {
        NavPoint {
            name,
            latitude,
            longitude,
        }
    }

    pub fn from_radians(name: String, latitude: f64, longitude: f64) -> NavPoint {
        NavPoint {
            name,
            latitude: latitude.to_degrees(),
            longitude: longitude.to_degrees(),
        }
    }
}

pub fn calculate_distance_and_course(
    from: &NavPoint,
    to: &NavPoint,
) -> (Option<f64>, Option<f64>) {
    let lat1 = from.latitude.to_radians();
    let lon1 = from.longitude.to_radians();
    let lat2 = to.latitude.to_radians();
    let lon2 = to.longitude.to_radians();

    let dlat = lat2 - lat1;
    let dlon = lon2 - lon1;

    let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    let earth_radius_km = 6371.0;
    let calculated_distance = earth_radius_km * c;

    let y = dlon.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * dlon.cos();
    let calculated_course = y.atan2(x).to_degrees();

    (Some(calculated_distance), Some(calculated_course))
}