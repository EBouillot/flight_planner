use super::geographics::{NavPoint, calculate_distance_and_course};
use reqwest::Error;
use serde::Deserialize;

pub struct Wind {
    pub speed: f64,
    pub direction: f64,
}

pub struct Navigation {
    pub start: NavPoint,
    pub end: NavPoint,
    pub branches: Vec<NavBranch>,
}
pub struct NavBranch {
    pub from: Option<NavPoint>,
    pub to: Option<NavPoint>,
    pub distance: Option<f64>,
    pub course: Option<f64>,
    pub wind: Wind,
    pub time: f64,
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
    ) -> NavBranch {
        let (calculated_distance, calculated_course) = if distance.is_none() || course.is_none() {
            if let (Some(from), Some(to)) = (&from, &to) {
                calculate_distance_and_course(from, to)
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
        }
    }
}