use crate::heuristics::state::State as State;

/**
* Threshold Accepting for TSP.
* Return the best solution found and log of fitness
* of each state during execution.
*/
pub fn threshold_accepting(initial_state: State, iterations: u32) -> (State, Vec<String>) {
        let mut log : Vec<String> = vec![];
        let thresholds = vec![0.13,0.12,0.11,0.10,
                              0.095,0.09,0.085,
                              0.08,0.075,0.075,
                              0.075,0.07,0.07,
                              0.07,0.065,0.065,
                              0.065,0.06,0.06,
                              0.055,0.055,0.05,
                              0.05,0.05,0.04,
                              0.04,0.03,0.02,0.0];
        let mut current_state : State;
        current_state = initial_state;
        log.push(current_state.fitness().to_string());
        for threshold in thresholds {
                for _ in 0..iterations {
                    let new_state = current_state.get_neighbor();
                    if new_state.fitness() <= current_state.fitness() + threshold {
                        current_state = new_state;
                        log.push(current_state.fitness().to_string());
                    }
                }
        }

        return (current_state,log);
}
