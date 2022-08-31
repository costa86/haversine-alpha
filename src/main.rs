use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "Gets the distance (Km) between multiple coordinates using Haversine formula"
)]
struct Args {
    #[clap(
        short,
        long,
        help = "Comma-separated pairs of latitudes and longitudes. Accepts more than 2 coordinates",
        value_name = "LATITUDE,LONGITUDE",
        allow_hyphen_values = true
    )]
    coordinates: String,
}

///Gets total distance in kilometers
fn calculate_total_distance(coordinates_list: Vec<&[f64]>) -> f64 {
    let mut distance: f64 = 0.0;

    for i in 0..coordinates_list.len() - 1 {
        distance += calculate_distance(
            *coordinates_list.get(i).unwrap(),
            *coordinates_list.get(i + 1).unwrap(),
        );
    }
    distance
}

///Checks whether latitude (index `0`) and longitude (index `1`) are within their limits
fn validate_coordinates(coordinates: &[f64]) -> bool {
    coordinates[0] <= 90.0
        && coordinates[0] >= -90.0
        && coordinates[1] <= 180.0
        && coordinates[1] >= -180.0
}

///Computes the distance in kilometers between two points on the Earth with the `Haversine formula`
fn calculate_distance(coord_1: &[f64], coord_2: &[f64]) -> f64 {
    let earth_radius_kilometer = 6371.0_f64;
    let (lat_1_degrees, lng_1_degrees) = (coord_1.get(0).unwrap(), coord_1.get(1).unwrap());
    let (lat_2_degrees, lng_2_degrees) = (coord_2.get(0).unwrap(), coord_2.get(1).unwrap());

    let lat_1 = lat_1_degrees.to_radians();
    let lat_2 = lat_2_degrees.to_radians();

    let delta_latitude = (lat_1_degrees - lat_2_degrees).to_radians();
    let delta_longitude = (lng_1_degrees - lng_2_degrees).to_radians();

    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
        + lat_1.cos() * lat_2.cos() * (delta_longitude / 2.0).sin().powi(2);
    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    let distance = earth_radius_kilometer * central_angle;
    distance
}

fn main() {
    let args = Args::parse();

    let coordinates_list: Vec<f64> = args
        .coordinates
        .split(",")
        .map(|x| {
            x.trim()
                .parse::<f64>()
                .expect("Could not generate a float value")
        })
        .collect();

    let coordinates_list: Vec<&[f64]> = coordinates_list.chunks_exact(2).collect();

    for i in &coordinates_list {
        if !validate_coordinates(i) {
            panic!("invalid coordinate: {:#?}", i);
        }
    }
    println!("{:.2}", calculate_total_distance(coordinates_list));
}

#[cfg(test)]
mod tests {
    use crate::{calculate_distance, validate_coordinates};

    #[test]
    fn test_validate_coordinates() {
        assert_eq!(validate_coordinates(&vec![0.0, 0.0]), true);
        assert_eq!(validate_coordinates(&vec![90.0, 180.0]), true);
        assert_eq!(validate_coordinates(&vec![-90.0, -180.0]), true);
        assert_eq!(validate_coordinates(&vec![91.0, 0.0]), false);
        assert_eq!(validate_coordinates(&vec![-91.0, 0.0]), false);
        assert_eq!(validate_coordinates(&vec![0.0, 181.0]), false);
        assert_eq!(validate_coordinates(&vec![0.0, -181.0]), false);
        assert_eq!(validate_coordinates(&vec![-50.0, -50.0]), true);
    }
    #[test]
    fn test_calculate_distance() {
        assert_eq!(
            calculate_distance(&vec![48.85341, -2.34880], &vec![51.50853, -0.12574]).ceil(),
            335.0
        );
    }
}
