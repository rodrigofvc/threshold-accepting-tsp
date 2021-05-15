use rand::Rng;
use crate::heuristics::city::City as City;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct State {
    pub parent: *const State,
    pub tour: Vec<City>
}


impl State {
    pub fn new(parent: *const State, tour: Vec<City>) -> State {
        State { parent, tour }
    }

    /**
    * Create a new neighbor using LIN-2-OPT procedure.
    * return the new state.
    **/
    pub fn get_neighbor(&self) -> State {
        let mut i = rand::thread_rng().gen_range(0, self.tour.len());
        let mut j = rand::thread_rng().gen_range(0, self.tour.len());
        while i >= j {
            i = rand::thread_rng().gen_range(0, self.tour.len());
            j = rand::thread_rng().gen_range(0, self.tour.len());
        }
        let mut tour_neighbor = self.tour.clone();
        for k in 0..tour_neighbor.len() {
            if k > i && k <= j {
                tour_neighbor[k] = self.tour[i+j+1-k].clone();
            }
        }
        let neighbor = State { parent: self, tour: tour_neighbor };
        neighbor
    }
}


#[cfg(test)]
 mod tests {
     use crate::heuristics::city::City as City;
     use crate::heuristics::state::State as State;

     #[test]
     fn test_get_neighbor() {
         let initial = init_state();
         let neighbor = initial.get_neighbor();
         assert_eq!(initial.tour.len(), initial.tour.len());
         // Check if the neighbor is a valid state
         for city in initial.tour {
             let iter = neighbor.tour.iter();
             let times : Vec<&City> = iter.filter(|&x| x.id == city.id).collect();
             // Check if city appears one time
             assert_eq!(times.len(), 1);
        }
     }

     fn init_state() -> State {
        let a = City::new(1, 34.4, 54.6);
        // 1 -> 2 42.242277
        let b = City::new(2, 12.3, 18.6);
        // 2 -> 3 87.184001
        let c = City::new(3, 96.0, 43.0);
        // 3 -> 4 94.681044
        let d = City::new(4, 03.7, 21.9);
        // 4 -> 5 75.700066
        let e = City::new(5, 76.4, 43.0);
        // 5 -> 6 63.724799
        let f = City::new(6, 14.1, 29.6);
        // 6 -> 7  29.441637
        let g = City::new(7, 23.2, 01.6);
        // 7 -> 8 81.973715
        let h = City::new(8, 32.0, 83.1);
        // 8 -> 9 82.186374
        let i = City::new(9, 88.8, 23.7);
        // 9 -> 10 92.93374
        let j = City::new(10, 12.6, 76.9);
        // 10 -> 11 31.185413
        let cities = vec![a,b,c,d,e,f,g,h,i,j];
        let initial = State::new(std::ptr::null(), cities);
        return initial;
    }

 }
