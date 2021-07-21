use rand::{Rng, SeedableRng, StdRng};
use crate::graph::city::City as City;
use crate::graph::path::Path as Path;

#[derive(Clone, Debug)]
pub struct State<'a> {
    pub paths: &'a Vec<Path>,
    pub tour: Vec<City>,
    rng: StdRng,
}


impl<'a> State<'a> {
    pub fn new(paths: &'a Vec<Path>,  tour: Vec<City>, seed: u64) -> State<'a> {
        State { paths:paths, tour, rng: SeedableRng::seed_from_u64(seed) }
    }

    /**
    * Create a new neighbor changing two random positions.
    */
    pub fn get_neighbor(&mut self) -> (State, (usize,usize)) {
        let mut i = self.rng.gen_range(0, self.tour.len());
        let mut j = self.rng.gen_range(0, self.tour.len());
        while i == j {
            i = self.rng.gen_range(0, self.tour.len());
            j = self.rng.gen_range(0, self.tour.len());
        }
        let mut tour_neighbor = self.tour.clone();
        let tmp = self.tour[i].clone();
        tour_neighbor[i] = self.tour[j].clone();
        tour_neighbor[j] = tmp;
        let neighbor = State { paths:self.paths, tour: tour_neighbor, rng: self.rng.clone()};
        (neighbor,(i,j))
    }

    pub fn set_neighbor(&mut self, i: usize, j: usize) {
        let tmp = self.tour[i].clone();
        self.tour[i] = self.tour[j].clone();
        self.tour[j] = tmp;
    }


    pub fn cost(&self) -> f64 {
        let cost;
        let mut sum :f64 = 0.0;
        let normalizer = self.normalizer();
        let max_distance = self.maximum_distance();
        for i in 1..self.tour.len() {
            let is_contained = self.paths.iter().find(|&x| x.city_1 == self.tour[i-1] && x.city_2 == self.tour[i] );
            match is_contained {
                Some(path) => {
                    sum += path.distance;
                },
                None => {
                    let natural_distance = self.tour[i-1].clone().get_distance(self.tour[i].clone());
                    sum += natural_distance * max_distance;
                },
            }
        }
        cost = sum / normalizer;
        cost
    }

    pub fn normalizer(&self) -> f64 {
        let normalizer;
        let mut greater_weights = vec![];
        for i in 0..self.tour.len() {
            for j in i+1..self.tour.len() {
                let is_contained = self.paths.iter()
                                        .find(|&x| (x.city_1 == self.tour[i] && x.city_2 == self.tour[j]) ||
                                                   (x.city_2 == self.tour[i] && x.city_1 == self.tour[j]) );
                match is_contained {
                    Some(path) => {
                        greater_weights.push(path.distance);
                    },
                    None => {},
                }
            }
        }
        greater_weights.sort_by(|a, b| a.partial_cmp(b).unwrap());
        greater_weights.reverse();
        let greater_weights = &greater_weights[0..self.tour.len()-1];
        normalizer = greater_weights.iter().fold(0.0, |acc, x| acc + x);
        normalizer
    }

    pub fn maximum_distance(&self) -> f64{
        let iter = self.paths.iter().
                        filter(|&x| self.tour.iter().any(|y| *y == x.city_1 ) &&
                                    self.tour.iter().any(|z| *z == x.city_2 ) );
        let max_path = iter.max_by(|x,y|x.cmp(y)).unwrap();
        max_path.distance
    }

    #[allow(dead_code)]
    pub fn is_feasible(&self) -> bool {
        for i in 1..self.tour.len() {
            let is_path = self.paths.iter().find(|&x| x.city_1 == self.tour[i-1] && x.city_2 == self.tour[i]);
            if is_path == None {
                return false;
            }
        }
        true
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s.push('[');
        for city in &self.tour {
            s.push(' ');
            s.push_str(&city.id.to_string());
            s.push(' ');
        }
        s.push(']');
        return s;
    }


     /**
     * Get a string with every coordenade of every city.
     * Firts column is x axis, second column is y axis.
     * #X #Y
     * 1.23 4.56 # First city
     * .........
     *
     */
     pub fn get_coordinates(&self) -> String {
         let mut content = String::new();
         for city in &self.tour {
             content.push_str(&city.to_string());
         }
         content
     }

}


#[cfg(test)]
 mod tests {
     use crate::graph::city::City as City;
     use crate::graph::path::Path as Path;
     use crate::heuristics::state::State as State;

     #[test]
     fn test_get_neighbor() {
         let (cities, paths) = init();
         let mut initial = State::new(&paths, cities, 10);
         let initial_copy = initial.clone();
         let (neighbor, _) = initial.get_neighbor();
         let changed = neighbor.tour.iter()
                                .filter(|&x|
                                    neighbor.tour.iter().position(|y| *y == *x).unwrap() !=
                                    initial_copy.tour.iter().position(|z| *z == *x).unwrap());
         assert_eq!(changed.count(), 2);
     }

     #[test]
     fn test_maximum_distance() {
        let (cities, paths) = init();
        let initial = State::new(&paths, cities, 10);
        let max = initial.maximum_distance();
        assert_eq!(max, 1124687.0);
     }

     #[test]
     fn test_feasible_solution() {
        let (cities, paths) = init();
        let mut initial = State::new(&paths, cities, 10);
        let mut feasible = initial.is_feasible();
        assert!(feasible);
        let other = City::new(10, String::from("other"), String::from("Other"), 0, 6.4, 41.0);
        initial.tour.push(other);
        feasible = initial.is_feasible();
        assert!(!feasible);
        let mut paths_ = paths.clone();
        let e = City::new(5, String::from("e"), String::from("E"), 0, 76.4, 43.0);
        let j = City::new(10, String::from("j"), String::from("J"), 0, 16.3, 32.0);
        let a = Path::new(e, j, 1223565.3);
        paths_.push(a);
        initial.paths = &paths_;
        feasible = initial.is_feasible();
        assert!(feasible);
     }

     #[test]
     fn test_normalizer() {
         let (cities, paths) = init();
         let mut initial = State::new(&paths, cities, 10);
         let mut nm = 1124687.0 + 42353.6 + 23467.5 + 16498.7;
         assert_eq!(initial.normalizer(), nm);
         let e = City::new(5, String::from("e"), String::from("E"), 0, 76.4, 43.0);
         let f = City::new(6, String::from("f"), String::from("F"), 0, 36.5, 63.7);
         let ef = Path::new(e, f.clone(), 17934576.5);
         let mut paths_ = initial.paths.clone();
         paths_.push(ef);
         initial.tour.push(f);
         initial.paths = &paths_;
         nm += 17934576.5;
         assert_eq!(initial.normalizer(), nm);
     }

     #[test]
     fn test_cost() {
         let (cities, paths) = init();
         let mut initial = State::new(&paths, cities, 10);
         let mut nm = 1124687.0 + 42353.6 + 23467.5 + 16498.7;
         let mut cost;
         cost = 12345.3 + 5383.9 + 3426.6 + 23467.5;
         cost /= nm;
         assert_eq!(initial.cost(), cost);
         let e = City::new(5, String::from("e"), String::from("E"), 0, 76.4, 43.0);
         let f = City::new(6, String::from("f"), String::from("F"), 0, 36.5, 63.7);
         let ef = Path::new(e, f.clone(), 17934576.5);
         let mut paths_ = initial.paths.clone();
         paths_.push(ef);
         initial.tour.push(f);
         initial.paths = &paths_;
         nm = 1124687.0 + 42353.6 + 23467.5 + 16498.7 + 17934576.5;
         cost = 12345.3 + 5383.9 + 3426.6 + 23467.5 + 17934576.5;
         cost /= nm;
         assert_eq!(initial.cost(), cost);

         let z = City::new(12, String::from("z"), String::from("Z"), 0, 326.5, 633.72);
         initial.tour.push(z.clone());
         nm = 1124687.0 + 42353.6 + 23467.5 + 16498.7 + 17934576.5 + 12345.3;
         cost = 12345.3 + 5383.9 + 3426.6 + 23467.5 + 17934576.5;
         cost += z.get_distance(initial.tour[initial.tour.len()-2].clone()) * 17934576.5;
         cost /= nm;
         assert_eq!(initial.cost(), cost);
     }

     fn init() -> (Vec<City>, Vec<Path>) {
        let a = City::new(1, String::from("a"), String::from("A"), 0, 34.4, 54.6);
        let b = City::new(2, String::from("b"), String::from("B"), 0, 12.3, 18.6);
        let c = City::new(3, String::from("c"), String::from("C"), 0, 96.0, 43.0);
        let d = City::new(4, String::from("d"), String::from("D"), 0, 03.7, 21.9);
        let e = City::new(5, String::from("e"), String::from("E"), 0, 76.4, 43.0);
        let f = City::new(6, String::from("f"), String::from("F"), 0, 23.3, 12.5);
        let g = City::new(7, String::from("g"), String::from("G"), 0, 12.5, 42.0);
        let h = City::new(8, String::from("h"), String::from("H"), 0, 63.6, 34.2);
        let i = City::new(9, String::from("i"), String::from("I"), 0, 75.2, 85.6);
        let cities = vec![a.clone(),b.clone(),c.clone(),d.clone(),e.clone()];
        let ab = Path::new(a.clone(), b.clone(), 12345.3);
        let ac = Path::new(a.clone(), c.clone(), 42353.6);
        let ad = Path::new(a.clone(), d.clone(), 16498.7);
        let ae = Path::new(a.clone(), e.clone(), 2322.8);
        let bc = Path::new(b.clone(), c.clone(), 5383.9);
        let bd = Path::new(b.clone(), d.clone(), 3858.1);
        let be = Path::new(b.clone(), e.clone(), 1124687.0);
        let cd = Path::new(c.clone(), d.clone(), 3426.6);
        let ce = Path::new(c.clone(), e.clone(), 2347.4);
        let de = Path::new(d.clone(), e.clone(), 23467.5);
        let fg = Path::new(f.clone(), g.clone(), 23467112.5);
        let fh = Path::new(f.clone(), h.clone(), 762346456.2);
        let fi = Path::new(f.clone(), i.clone(), 92341007.43);
        let hi = Path::new(h.clone(), i.clone(), 497945123.5);
        let paths = vec![ab,ac,ad,ae,bc,bd,be,cd,ce,de,fg,fh,fi,hi];
        (cities, paths)
    }
 }
