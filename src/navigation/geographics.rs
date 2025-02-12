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