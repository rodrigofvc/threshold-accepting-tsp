use core::cmp::Ordering;
use super::city::City as City;

#[derive(Clone, Debug)]
pub struct Path {
    pub city_1: City,
    pub city_2: City,
    pub distance: f64,
}

impl Path {
    #[allow(dead_code)]
    pub fn new(city_1: City, city_2: City, distance: f64) -> Path {
        Path {city_1, city_2, distance}
    }
}

impl PartialEq for Path {
    fn eq(&self, other : &Self) -> bool {
        self.city_1 == other.city_1 &&
        self.city_2 == other.city_2 &&
        self.distance == other.distance
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Eq for Path {}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance == other.distance {
            return Ordering::Equal;
        } else if self.distance < other.distance {
            return Ordering::Less;
        }
        return Ordering::Greater;
    }
}
