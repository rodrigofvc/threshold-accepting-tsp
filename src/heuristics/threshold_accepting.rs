use crate::heuristics::state::State as State;

/**
* Threshold Accepting for TSP.
* Return the best solution found.
*/
pub fn threshold_accepting(initial_state: State, iterations: u32, mut temperature: f64, decrement: f64, mut seed: u64, epsilon: f64) -> State {
    let mut current_state = initial_state.clone();
    let mut p : f64 = 0.0;
    let mut q : f64;
    while temperature > epsilon {
        q = f64::INFINITY;
        while p <= q {
            q = p;
            let (a, new_state) = get_lot(temperature, current_state, iterations, &mut seed);
            p = a;
            current_state = new_state;
        }
        println!("\n >>>>>>>>>>>>>> \n Temperatura actual: {:?}", temperature);
        println!(" Instancia: {:?}", current_state.to_string());
        println!(" Costo: {:?}",current_state.cost());
        temperature *= decrement;
    }
    return current_state;
}


fn get_lot<'a>(temperature: f64, initial: State<'a>, iterations: u32, seed: &mut u64) -> (f64, State<'a>) {
    let mut c : u32 = 0;
    let mut r : f64 = 0.0;
    let mut attemps = iterations * 2;
    let mut current = initial.clone();
    while c < iterations && attemps > 0 {
        *seed += 1;
        let next_cost = current.get_neighbor(*seed).cost();
        if next_cost <= current.cost() + temperature {
            current.set_neighbor(*seed);
            c += 1;
            r += current.cost();
        }
        attemps -= 1;
    }
    let prom = r/(iterations as f64);
    return (prom, current);
}
