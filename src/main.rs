mod heuristics;
use std::fs;
use std::env;
use std::time::Instant;
use crate::heuristics::state::State as State;
use crate::heuristics::city::City as City;
use crate::heuristics::threshold_accepting as th_acp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path_file = args[1].clone();
    let initial_state = get_initial_state(path_file);
    let start = Instant::now();
    let iter = 100;
    let best = th_acp::threshold_accepting(initial_state, iter);
    let duration = start.elapsed();
    println!("El mejor resultado posible obtenido: \n {:#?}", best);
    println!("Longitud: {}", best.fitness());
    println!("Tiempo: {:?}", duration);
    get_file(best);
}

// example.txt Optimal: 6656
// example1.txt Optimal: 9352
fn get_initial_state(path_file: String) -> State {
    let mut cities : Vec<City> = vec![];
    let content = fs::read_to_string(path_file).expect("No se encontró el archivo");
    let chunks : Vec<String> = content.split("\n").map(str::to_string).collect();
    let chunks : Vec<String> = chunks.iter().map(|x|x.replace('\r',"")).collect();
    for chunk in chunks {
        if chunk.len() == 0 {
            continue;
        }
        let tokens : Vec<String> = chunk.split_whitespace().map(str::to_string).collect();
        let new_city = City::new(tokens[0].parse::<u32>().unwrap(),tokens[1].parse::<f32>().unwrap(),tokens[2].parse::<f32>().unwrap());
        cities.push(new_city);
    }
    let initial = State::new(std::ptr::null(), cities);
    return initial;
}

/**
* Given a state, create a file with every coordenade of every state,
* the file can be read using a Gnuplot script.
* state: the state from which generate the file.
*/
fn get_file(state : State)  {
    let content = state.get_coordinates();
    let path = "data/data.dat";
    fs::File::create(path).expect("No se pudó crear un archivo");
    fs::write(path, content.as_bytes()).expect("No se pudó escribir un archivo");
}
