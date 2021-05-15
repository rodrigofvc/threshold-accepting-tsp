use core::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct City {
    pub id: u32,
    pub x_axis: f32,
    pub y_axis: f32
}

impl City {
    /**
    * Create a new City.
    * name: name of city.
    * x_axis: position in X axis.
    * y_axis: position in Y axis.
    */
    pub fn new(id: u32, x_axis: f32, y_axis: f32) -> City {
        City {id, x_axis, y_axis}
    }


    /**
    * Get euclidean distance with other city.
    */
    pub fn get_distance(&self, other: City) -> f32 {
        let x_subs = other.x_axis - self.x_axis;
        let y_subs = other.y_axis - self.y_axis;
        let x_subs = x_subs.powf(2.0);
        let y_subs = y_subs.powf(2.0);
        let adds = x_subs + y_subs;
        return adds.sqrt();
    }

}


impl PartialEq for City {
    fn eq(&self, other : &Self) -> bool {
        self.x_axis == other.x_axis &&
        self.y_axis == other.y_axis
    }
}

impl PartialOrd for City {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x_axis.partial_cmp(&other.x_axis)
    }
}

impl Eq for City{}

impl Ord for City{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x_axis == other.x_axis &&
           self.y_axis == other.y_axis {
            return Ordering::Equal;
        } else if self.x_axis >= other.x_axis &&
                  self.y_axis >= other.y_axis {
            return Ordering::Greater;
        }
        return Ordering::Less;
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_euclidean_distance(){
        let a = crate::heuristics::city::City::new(1, 34.4, 54.6);
        let b = crate::heuristics::city::City::new(2, 12.3, 18.6);
        let distance = a.get_distance(b);
        assert!((42.0..43.0).contains(&distance));
    }
}
