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
