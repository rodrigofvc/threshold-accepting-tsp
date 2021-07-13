use core::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct City {
    pub id: u32,
    pub name: String,
    pub country: String,
    pub population: u32,
    pub latitude: f64,
    pub longitude: f64,
}

impl City {
    /**
    * Create a new City.
    */
    pub fn new(id: u32, name: String, country: String, population: u32, latitude: f64, longitude: f64) -> City {
        City {id, name, country, population, latitude,longitude}
    }


    /**
    * Get natural distance.
    */
    pub fn get_distance(&self, other: City) -> f64 {
        let r = 6373000.0;
        let difference_latitude = (self.latitude.to_radians() - other.latitude.to_radians())/2.0;
        let product_lattitude = self.latitude.to_radians().cos() * other.latitude.to_radians().cos();
        let difference_longitude = (self.longitude.to_radians() - other.longitude.to_radians())/2.0;
        let a = difference_latitude.sin().powf(2.0) + product_lattitude * difference_longitude.sin().powf(2.0);
        let c = 2.0 * a.sqrt().atan2((1.0-a).sqrt());
        return r*c;
    }

    /**
    * Return latitude and longitude.
    * # X Y
    * 1.23 3.45
    */
    pub fn to_string(&self) -> String {
        let mut content = String::new();
        content.push(' ');
        content.push_str(&self.latitude.to_string());
        content.push(' ');
        content.push_str(&self.longitude.to_string());
        content.push('\n');
        content
    }

}


impl PartialEq for City {
    fn eq(&self, other : &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for City {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Eq for City {}

impl Ord for City {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.id == other.id {
            return Ordering::Equal;
        }
        return Ordering::Less;
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_distance(){
        let a = crate::heuristics::city::City::new(1, String::from("Tokyo"), String::from("Japan"), 31480498,35.685000000000002273,139.75100000000000477);
        let b = crate::heuristics::city::City::new(7, String::from("Manila"), String::from("Philippines"), 10443877,14.604200000000000514,120.98199999999999931);
        let c = crate::heuristics::city::City::new(9, String::from("Seoul"), String::from("South Korea"), 10323448,37.598500000000001364,126.97799999999999443);
        let mut distance = a.get_distance(b);
        assert!((2000000.0..3100000.0).contains(&distance));
        distance = a.get_distance(c);
        assert!((1150000.0..1170000.0).contains(&distance));
    }
}
