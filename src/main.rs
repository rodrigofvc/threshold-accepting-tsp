mod heuristics;
mod algorithms;
mod graph;
use std::env;
use std::fs;
use std::time::Instant;
use rusqlite::{Connection, Result};
use crate::graph::city::City as City;
use crate::graph::path::Path as Path;
use crate::heuristics::state::State as State;
use crate::heuristics::threshold_accepting as th_acp;
use crate::algorithms::approx_tsp::approx_tsp as approx_tsp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let seed = &args[1].parse::<u64>().unwrap();
    let path_file = &args[2];
    let instance = read_file(path_file.to_string());
    let mut paths : Vec<Path> = Vec::new();
    let mut cities : Vec<City> = Vec::new();
    let read = read(&mut paths, &mut cities);
    match read {
        Ok(_)  => {},
        Err(e) => { println!("{:?}", e) }
    }

    let mut cities : Vec<City> = cities.into_iter().filter(|x| instance.iter().any(|&y| y == x.id) ).collect();
    let mut paths : Vec<Path> = paths.into_iter().filter(|x| cities.iter().any(|y| *y == x.city_1) && cities.iter().any(|y| *y == x.city_2) ).collect();
    let new_cities = approx_tsp::approximation_tsp(&mut cities, &mut paths);
    let initial = State::new(&paths, new_cities.clone(), *seed);
    /*
    println!(" N {:#?}", initial.normalizer());
    println!(" C {:#?}", initial.cost());
    println!(" M {:#?}", initial.maximum_distance());
    */
    let iterations = 100;
    let temperature = 1000.0;
    let decrement = 0.9;
    let epsilon = 0.90;
    let start = Instant::now();
    let best = th_acp::threshold_accepting(initial, iterations, temperature, decrement, epsilon);
    let duration = start.elapsed().as_secs();
    println!("\n >>>>>>>>>>>>>>>>>>>>>>>>>>>", );
    println!(" Solucion mejor encontrada: \n {:?} \n Costo: {:?}", best.to_string(), best.cost());
    println!(" Tiempo: {:?} segundos", duration);
    write_file(best);
}

fn read (paths: &mut Vec<Path>, cities: &mut Vec<City>) -> Result<()> {
    let conn = Connection::open("tsp_data.db")?;

    let mut stmt_cities =
    conn.prepare(
        "SELECT id, name, country, population, latitude, longitude FROM cities"
    )?;

    let cities_ = stmt_cities.query_map([], |row| {
        Ok(
            City {
                id: row.get(0)?,
                name: row.get(1)?,
                country: row.get(2)?,
                population: row.get(3)?,
                latitude: row.get(4)?,
                longitude: row.get(5)?,
            }
        )
    })?;
    for city in cities_ {
        cities.push(city.unwrap());
    }
    let mut stmt_paths =
    conn.prepare(
        "SELECT id_city_1, id_city_2, distance FROM connections"
    )?;

    let paths_ = stmt_paths.query_map([], |row| {
        Ok(
            get_path(&cities, row.get(0)?, row.get(1)?, row.get(2)?).unwrap()
        )
    })?;
    for path in paths_ {
        paths.push(path.unwrap());
    }
    Ok(())
}

fn get_path(cities: &Vec<City>, id_city_1: u32, id_city_2: u32, distance: f64) -> Result<Path,> {
    let city_1 = cities.iter().find(|&x| x.id == id_city_1).unwrap();
    let city_2 = cities.iter().find(|&x| x.id == id_city_2).unwrap();
    let path = Path {
        city_1: city_1.clone(),
        city_2: city_2.clone(),
        distance: distance,
    };
    Ok(path)
}

fn read_file(path_file: String) -> Vec<u32> {
    let mut cities : Vec<u32> = vec![];
    let content = fs::read_to_string(path_file).expect("No se encontró el archivo");
    let chunks : Vec<String> = content.split("\n").map(str::to_string).collect();
    let chunks : Vec<String> = chunks.iter().map(|x|x.replace(' ',"")).collect();
    let chunks : Vec<String> = chunks.iter().map(|x|x.replace('\r',"")).collect();
    let chunks : Vec<String> = chunks.iter().map(|x|x.replace('\t',"")).collect();
    for chunk in chunks {
        if chunk.len() == 0 {
            continue;
        }
        let ct : Vec<String> = chunk.split(",").map(str::to_string).collect();
        for n in ct {
            if n.len() == 0 {
                continue;
            }
            let m : u32 = n.parse::<u32>().unwrap();
            cities.push(m);
        }
    }
    return cities;
}

/**
* Given a state, create a file with every coordenade of every state,
* the file can be read using a Gnuplot script.
* state: the state from which generate the file.
*/
fn write_file(state : State)  {
    let content = state.get_coordinates();
    let path = "data/data.dat";
    fs::File::create(path).expect("No se pudó crear un archivo");
    fs::write(path, content.as_bytes()).expect("No se pudó escribir un archivo");
}
