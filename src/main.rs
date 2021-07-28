mod heuristics;
mod graph;
use std::env;
use std::fs;
use std::time::Instant;
use std::io::Write;
use rusqlite::{Connection, Result};
use crate::graph::city::City as City;
use crate::graph::path::Path as Path;
use crate::heuristics::state::State as State;
use crate::heuristics::threshold_accepting as th_acp;

struct Best<'a>(State<'a>, u64, u64);
fn main() {
    let args: Vec<String> = env::args().collect();
    let initial_seed = args[1].parse::<u64>().unwrap();
    let final_seed = args[2].parse::<u64>().unwrap();
    let path_file = &args[3];
    let instance = read_file(path_file.to_string());
    let mut paths : Vec<Path> = Vec::new();
    let mut cities : Vec<City> = Vec::new();
    let read = read(&mut paths, &mut cities);
    match read {
        Ok(_)  => {},
        Err(e) => { println!("{:?}", e) }
    }

    let cities : Vec<City> = cities.into_iter().filter(|x| instance.iter().any(|&y| y == x.id) ).collect();
    let paths : Vec<Path> = paths.into_iter().filter(|x| cities.iter().any(|y| *y == x.city_1) && cities.iter().any(|y| *y == x.city_2) ).collect();
    let seeds : Vec<u64> = (initial_seed..=final_seed).collect();

    let init = State::new(&paths,  cities.clone(), initial_seed);
    let mut current_best : Best = Best(init, initial_seed, 0);

    let temperature = 2.0;
    let iterations = 100;
    let decrement = 0.99;
    let epsilon = 0.0011;

    for seed in seeds {
        let initial = State::new(&paths,  cities.clone(), seed);
        let start = Instant::now();
        let current = th_acp::threshold_accepting(initial, iterations, temperature, decrement, epsilon);
        let seconds = start.elapsed().as_secs();
        println!("\n >>>>>>>>>>>>>>>>>>>>>>>>>>>");
        println!(" Solucion usando {} como semilla : \n {:?} \n Costo: {:?}", seed, current.to_string(), current.cost());
        let minutes = seconds/60;
        let hours = minutes/60;
        println!(" Tiempo: {}:{}:{} hh:mm:ss", hours, minutes%60, seconds%60);
        if current.cost() <= current_best.0.cost() {
            current_best.0 = current.clone();
            current_best.1 = seed;
            current_best.2 = seconds;
            write_log(&current_best);
        }
    }
    println!("--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ");
    println!(" Solucion mejor encontrada: \n {:?} \n Costo: {:?} \n Semilla {:?} \n Tiempo {:?}", current_best.0.to_string(), current_best.0.cost(), current_best.1, current_best.2);
    write_file(current_best.0);
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

fn write_log(sol: &Best){
    let mut content  = String::new();
    content.push_str("\n >>>>>>>>>>> Instancia: \n");
    content.push_str(&sol.0.to_string());
    content.push('\n');
    content.push_str("Costo: ");
    content.push_str(&sol.0.cost().to_string());
    content.push(' ');
    content.push_str("Semilla: ");
    content.push_str(&sol.1.to_string());
    content.push(' ');
    content.push_str("Tiempo: ");
    content.push_str(&sol.2.to_string());
    let path = "log/log.dat";
    if !std::path::Path::new(path).is_file() {
        fs::File::create(path).expect("No se pudó crear un archivo");
        fs::write(path, content.as_bytes()).expect("No se pudó escribir un archivo");
    } else {
        let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

        write!(file, "{}", content).expect("No se pudó escribir un archivo");
    }
}
