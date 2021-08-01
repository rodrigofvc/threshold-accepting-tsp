use crate::heuristics::state::State as State;

/**
* Threshold Accepting for TSP.
* Return the best solution found.
*/
pub fn threshold_accepting(initial_state: State, iterations: u32, mut temperature: f64, decrement: f64, epsilon: f64) -> State {
    let mut current_state = initial_state;
    let probability : f64 = 0.89;
    temperature = initial_temperature(&mut current_state, temperature, probability);
    let mut p : f64 = 0.0;
    let mut q : f64;
    while temperature > epsilon {
        q = f64::INFINITY;
        let mut max_local_batch = 0;
        'inner: while p <= q {
            q = p;
            let (a, new_state) = get_batch(temperature, current_state.clone(), iterations);
            if max_local_batch == 20 {
                break 'inner;
            }
            p = a;
            current_state = new_state;
            if a == -1.0 {
                max_local_batch += 1;
            } else {
                max_local_batch = 0;
            }
        }
        println!("\n >>>>>>>>>>>>>> \n Temperatura actual: {:?}", temperature);
        println!(" Instancia: {:?}", current_state.to_string());
        println!(" Costo: {:?}",current_state.cost());
        temperature *= decrement;
    }
    return current_state;
}

pub fn initial_temperature(current_state: &mut State, mut temperature: f64, probability: f64) -> f64{
    let mut p : f64 = accepted_percent(current_state, temperature);
    let temperature_1 : f64;
    let temperature_2 : f64;
    let epsilon_p = 0.50;
    if (probability - p).abs() <= epsilon_p {
        return temperature;
    }
    if p <= probability {
        while p <= probability {
            temperature *= 2.0;
            p = accepted_percent(current_state, temperature);
        }
        temperature_1 = temperature/2.0;
        temperature_2 = temperature;
    } else {
        while p > probability {
            temperature /= 2.0;
            p = accepted_percent(current_state, temperature);
        }
        temperature_1 = temperature;
        temperature_2 = temperature * 2.0;
    }
    return binary_search(current_state, temperature_1, temperature_2, probability);
}

fn accepted_percent(current_state: &mut State, temperature: f64)  -> f64 {
    let mut c = 0;
    let n = 500;
    for _ in 0..n {
        let (neighbor, (i,j)) = current_state.get_neighbor();
        if neighbor.cost() <= current_state.cost() + temperature {
            c += 1;
            current_state.set_neighbor(i,j)
        }
    }
    let prom = c as f64 / n as f64;
    return prom;
}


fn binary_search(current_state: &mut State, temperature_1: f64, temperature_2: f64, probability: f64) -> f64 {
    let middle = (temperature_1 + temperature_2) / 2.00;
    let epsilon_p = 0.50;
    if temperature_2 - temperature_1 <  epsilon_p {
        return middle;
    }
    let p = accepted_percent(current_state, middle);
    if (probability - p).abs() < epsilon_p {
        return middle;
    }
    if p > probability {
        return binary_search(current_state, temperature_1, middle, probability);
    } else {
        return binary_search(current_state, middle, temperature_2, probability);
    }
}


fn get_batch(temperature: f64, initial: State, iterations: u32) -> (f64, State) {
    let mut c : u32 = 0;
    let mut r : f64 = 0.0;
    let mut attemps = iterations * 2;
    let mut current = initial;
    while c < iterations && attemps > 0 {
        let (neighbor, (i,j)) = current.get_neighbor();
        if neighbor.cost() <= current.cost() + temperature {
            current.set_neighbor(i,j);
            c += 1;
            r += current.cost();
        }
        attemps -= 1;
    }
    let mut prom = r/(iterations as f64);
    if c == 0 {
        prom = -1.0;
    }
    return (prom, current);
}
