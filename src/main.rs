// Calculates the distance between two cities and allows the user to specify a unit of distance.
// This calculates the distance as the crow flies.

extern crate reqwest;
use serde_json::Value;
use std::f64;
#[macro_use] extern crate text_io;


fn main() -> Result<(), Box<std::error::Error>> {
    println!("What is the first city name?");
    let city1: String = read!("{}\n");
    println!("What is the second city name?");
    let city2: String = read!("{}\n");
    println!("Units: Miles (mi) or Kilometers (km)");
    let units: String = read!("{}\n");

    let url1 = &format!("https://graphhopper.com/api/1/geocode?q={}&limit=1&key=245d4a8b-070c-43f8-901e-95fe8c62b35e", city1);
    let url2 = &format!("https://graphhopper.com/api/1/geocode?q={}&limit=1&key=245d4a8b-070c-43f8-901e-95fe8c62b35e", city2);

    let res = reqwest::get(url1)?.text()?;
    let res2 = reqwest::get(url2)?.text()?;

    let v: Value = serde_json::from_str(&res)?;
    let v2: Value = serde_json::from_str(&res2)?;

    let place1 = &v["hits"][0];
    let place2 = &v2["hits"][0];

    let lat_a: f64 = place1["point"]["lat"].as_f64().unwrap();
    let lng_a: f64 = place1["point"]["lng"].as_f64().unwrap();

    let lat_b: f64 = place2["point"]["lat"].as_f64().unwrap();
    let lng_b: f64 = place2["point"]["lng"].as_f64().unwrap();

    let distance: f64 = calc_distance(lat_a, lng_a, lat_b, lng_b, &units);

    print!("Distance between {} and {}: {}{}", place1["name"], place2["name"], distance.round(), units);

    Ok(())
}

fn calc_distance(lat_a: f64, lng_a: f64, lat_b: f64, lng_b: f64, units: &String) -> f64 {
    // Use the haversine formula
    let earth_rad = 3959.0; // in miles
    let dlat = (lat_b - lat_a).to_radians();
    let dlng = (lng_b - lng_a).to_radians();
    let a = (dlat / 2.0).sin() * (dlat / 2.0).sin() + lat_a.to_radians().cos() * lat_b.to_radians().cos() * (dlng / 2.0).sin() * (dlng / 2.0).sin();
    let c = 2.0 * a.sqrt().asin();
    let distance = earth_rad * c;

    if units.trim() == &String::from("mi") {
        return distance;
    } else {
        return convert_mi_to_km(distance);
    }
}

fn convert_mi_to_km(mi: f64) -> f64{
    return mi * 1.609344;
}