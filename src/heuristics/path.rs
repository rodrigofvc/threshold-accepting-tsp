use core::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Path {
    pub id_city_1: u32,
    pub id_city_2: u32,
    pub distance: f64,
}

impl Path {
    #[allow(dead_code)]
    pub fn new(id_city_1: u32, id_city_2: u32, distance: f64) -> Path {
        Path {id_city_1, id_city_2, distance}
    }
}

impl PartialEq for Path {
    fn eq(&self, other : &Self) -> bool {
        self.id_city_1 == other.id_city_1 &&
        self.id_city_2 == other.id_city_2 &&
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
