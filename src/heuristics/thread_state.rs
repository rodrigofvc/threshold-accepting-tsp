use crate::heuristics::state::State as State;

#[derive(Clone, Debug)]
pub struct ThreadState {
    pub state: State,
    pub seed: u64,
    seconds: u64,
    pub log: Vec<String>,
}

impl ThreadState {

    pub fn new(state: State, seed: u64, seconds: u64, log: Vec<String>) -> ThreadState {
        ThreadState{state, seed, seconds, log}
    }

    pub fn get_time(&self) -> String {
        let mut time = String::new();
        let minutes = self.seconds/60;
        let hours =  minutes/60;
        let minutes = minutes % 60;
        let seconds = self.seconds % 60;
        time.push_str(&hours.to_string());
        time.push(':');
        time.push_str(&minutes.to_string());
        time.push(':');
        time.push_str(&seconds.to_string());
        time.push_str(" hh:mm:ss");
        return time;
    }
}
