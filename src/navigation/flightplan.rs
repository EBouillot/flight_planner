use super::geographics::NavPoint;
use reqwest::Error;
use serde::Deserialize;

pub struct Wind {
    pub speed: f64,
    pub direction: f64,
}
pub struct NavBranch {
    pub from: Option<NavPoint>,
    pub to: Option<NavPoint>,
    pub distance: Option<f64>,
    pub course: Option<f64>,
    pub wind: Wind,
    pub time: f64,
    pub fuel: f64,
}

/// Creates a new `NavBranch` instance.
///
/// # Parameters
/// - `from`: An optional `NavPoint` representing the starting point of the navigation branch.
/// - `to`: An optional `NavPoint` representing the ending point of the navigation branch.
/// - `distance`: An optional `f64` representing the distance between the `from` and `to` points. If not provided, it will be calculated.
/// - `course`: An optional `f64` representing the course (bearing) from the `from` point to the `to` point. If not provided, it will be calculated.
/// - `wind`: A `Wind` struct representing the wind conditions.
/// - `time`: A `f64` representing the time taken for the navigation.
/// - `fuel`: A `f64` representing the fuel consumed during the navigation.
///
/// # Returns
/// A `NavBranch` instance with the provided or calculated values.
///
/// # Calculations
/// If either `distance` or `course` is not provided:
/// - The distance is calculated using the Haversine formula, which determines the great-circle distance between two points on a sphere given their longitudes and latitudes.
/// - The course is calculated using the initial bearing formula, which determines the angle between the north direction and the line connecting the two points.
///
/// # Example
/// ```
/// let from = Some(NavPoint { latitude: 34.0522, longitude: -118.2437 });
/// let to = Some(NavPoint { latitude: 40.7128, longitude: -74.0060 });
/// let wind = Wind { speed: 10.0, direction: 90.0 };
/// let nav_branch = NavBranch::new(from, to, None, None, wind, 5.0, 100.0);
/// ```
impl NavBranch {
    pub fn new(
        from: Option<NavPoint>,
        to: Option<NavPoint>,
        distance: Option<f64>,
        course: Option<f64>,
        wind: Wind,
        time: f64,
        fuel: f64,
    ) -> NavBranch {
        let (calculated_distance, calculated_course) = if distance.is_none() || course.is_none() {
            if let (Some(from), Some(to)) = (&from, &to) {
                Self::calculate_distance_and_course(from, to)
            } else {
                (None, None)
            }
        } else {
            (distance, course)
        };

        NavBranch {
            from,
            to,
            distance: distance.or(calculated_distance),
            course: course.or(calculated_course),
            wind,
            time,
            fuel,
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
}

pub struct Aircraft {
    pub aircraft_type: String,
    pub weight: f64,
    pub cruise_speed: f64,
}
