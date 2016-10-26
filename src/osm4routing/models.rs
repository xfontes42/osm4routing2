extern crate num_traits;
use std;
use categorize::EdgeProperties;

pub trait Float: std::convert::From<f32> {}

// Coord are coordinates in decimal degress WGS84
#[derive(Copy, Clone)]
pub struct Coord<T: Float> {
    pub lon: T,
    pub lat: T,
}

// Node is the OpenStreetMap node
#[derive(Copy, Clone)]
pub struct Node<T: Float> {
    pub id: i64,
    pub coord: Coord<T>,
    pub uses: i16,
}

impl<T: Float> Node<T> {
    pub fn new() -> Self {
        Node {
            id: 0,
            coord: Coord::<T> {
                lon: T::from(0.).unwrap(),
                lat: T::from(0.).unwrap(),
            },
            uses: 0,
        }
    }
}

// Edge is a topological representation with only two extremities and no geometry
pub struct Edge<T: Float> {
    pub id: i64,
    pub source: i64,
    pub target: i64,
    pub geometry: Vec<Coord<T>>,
    pub properties: EdgeProperties,
}

impl<T: Float + std::fmt::Display> Edge<T> {
    // Geometry in the well known format
    pub fn as_wkt(&self) -> String {
        let coords: Vec<String> = self.geometry
            .iter()
            .map(|coord| format!("{:.7} {:.7}", coord.lon, coord.lat))
            .collect();

        format!("LINESTRING({})", coords.as_slice().join(", "))
    }

    // Length in meters of the edge
    pub fn length(&self) -> T {
        self.geometry.windows(2).map(|coords| distance(coords[0], coords[1])).sum()
    }
}

pub fn distance<T: Float>(start: Coord<T>, end: Coord<T>) -> T {
    let r = T::from(6378100.0).unwrap();

    let d_lon = (end.lon - start.lon).to_radians();
    let d_lat = (end.lat - start.lat).to_radians();
    let lat1 = (start.lat).to_radians();
    let lat2 = (end.lat).to_radians();

    let a = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin()) +
            ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());
    let c = 2.0 as T * ((a.sqrt()).atan2((1.0 as T - a).sqrt()));

    return r * c;
}


#[test]
fn test_as_wkt() {
    let edge = Edge {
        id: 0,
        source: 0,
        target: 0,
        geometry: vec![Coord<f64> { lon: 0., lat: 0. },
                       Coord<f64> { lon: 1., lat: 0. },
                       Coord<f64> { lon: 0., lat: 1. }],
        properties: EdgeProperties::new(),
    };
    assert!("LINESTRING(0.0000000 0.0000000, 1.0000000 0.0000000, 0.0000000 1.0000000)" ==
            edge.as_wkt());
}


#[test]
fn test_distance() {
    let a = Coord::<f64> { lon: 0., lat: 0. };
    let b = Coord::<f64> { lon: 1., lat: 0. };

    assert!((1. - (distance(a, b) / (1853. * 60.))).abs() < 0.01);
}
