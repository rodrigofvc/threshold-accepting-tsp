use crate::heuristics::path::Path as Path;
use crate::heuristics::city::City as City;

pub fn approximation_tsp(cities: &Vec<City>, minimum_spanning_tree: &Vec<Path>) -> Vec<City>{
    let mut new_cities : Vec<City> = Vec::new();
    let mut tree = minimum_spanning_tree.clone();
    while tree.len() != 0 {
        get_component(cities, &mut new_cities, &mut tree);
    }
    if new_cities.len() != cities.len() {
        let mut remaining : Vec<City> = cities.clone().into_iter().filter(|x| !new_cities.iter().any(|y| y == x)).collect();
        new_cities.append(&mut remaining);
    }
    return new_cities;
}

fn get_component(cities: &Vec<City>, new_cities: &mut Vec<City>, tree: &mut Vec<Path>) {
    let mut maked_paths : Vec<Path> = Vec::new();
    let mut marked_cities :Vec<u32> = Vec::new();
    let root = tree[0].clone();

    let mut stack : Vec<u32> = Vec::new();
    stack.insert(0, root.id_city_1);
    while stack.len() > 0 {
        let current = stack.remove(0);
        let is_marked = marked_cities.iter().find(|&&x| x == current);
        match is_marked {
            Some(_) => continue,
            None => {},
        }
        let mut neighbors_path = tree.iter().filter(|x| x.id_city_1 == current || x.id_city_2 == current);
        let mut iter = neighbors_path.next();
        while iter != None {
            let path = iter.unwrap();
            let is_marked = maked_paths.iter().find(|&x|x == path);
            match is_marked {
                Some(_) => {},
                None => maked_paths.push(path.clone()),
            }
            if current != path.id_city_2 {
                stack.insert(0, path.id_city_2);
            } else {
                stack.insert(0, path.id_city_1);
            }
            iter = neighbors_path.next();
        }
        marked_cities.push(current);
    }

    for path in maked_paths {
        let i = tree.iter().position(|x| *x == path).unwrap();
        tree.remove(i);
    }

    for id in marked_cities {
        let city = cities.iter().find(|x| x.id == id).unwrap().clone();
        let contain = new_cities.iter().find(|x| x.id == id);
        match contain {
            Some(_) => {},
            None => { new_cities.push(city); }
        }
    }

}

#[cfg(test)]
mod test {
    use crate::heuristics::city::City as City;
    use crate::heuristics::path::Path as Path;
    use crate::algorithms::approx_tsp::approx_tsp as approx_tsp;

    #[test]
    fn test_approximation_tsp(){
        let mut cities : Vec<City> = Vec::new();
        let mut paths : Vec<Path> = Vec::new();
        let a = City{id:0, name: String::from("a"), population:1, country: String::from("A"), latitude: 0.00, longitude: 0.00};
        let b = City{id:1, name: String::from("b"), population:1, country: String::from("B"), latitude: 0.00, longitude: 0.00};
        let c = City{id:2, name: String::from("c"), population:1, country: String::from("C"), latitude: 0.00, longitude: 0.00};
        let d = City{id:3, name: String::from("d"), population:1, country: String::from("D"), latitude: 0.00, longitude: 0.00};
        let e = City{id:4, name: String::from("e"), population:1, country: String::from("E"), latitude: 0.00, longitude: 0.00};
        let f = City{id:5, name: String::from("f"), population:1, country: String::from("F"), latitude: 0.00, longitude: 0.00};
        let g = City{id:6, name: String::from("g"), population:1, country: String::from("G"), latitude: 0.00, longitude: 0.00};
        let h = City{id:7, name: String::from("h"), population:1, country: String::from("H"), latitude: 0.00, longitude: 0.00};
        let i = City{id:8, name: String::from("i"), population:1, country: String::from("I"), latitude: 0.00, longitude: 0.00};
        cities = vec![a.clone(),b.clone(),c.clone(),d.clone(),e.clone(),f.clone(),g.clone(),h.clone(),i.clone()];
        let ab = Path{id_city_1:0 ,id_city_2:1, distance:4.00};
        let ah = Path{id_city_1:0 ,id_city_2:7, distance:8.00};
        let bc = Path{id_city_1:1, id_city_2:2, distance:8.00};
        let bh = Path{id_city_1:1, id_city_2:7, distance:11.00};
        let cd = Path{id_city_1:2, id_city_2:3, distance:7.00};
        let cf = Path{id_city_1:2, id_city_2:5, distance:4.00};
        let ci = Path{id_city_1:2, id_city_2:8, distance:2.00};
        let de = Path{id_city_1:3, id_city_2:4, distance:9.00};
        let df = Path{id_city_1:3, id_city_2:5, distance:14.00};
        let ef = Path{id_city_1:4, id_city_2:5, distance:10.00};
        let fg = Path{id_city_1:5, id_city_2:6, distance:2.00};
        let gh = Path{id_city_1:6, id_city_2:7, distance:1.00};
        let gi = Path{id_city_1:6, id_city_2:8, distance:6.00};
        let hi = Path{id_city_1:7, id_city_2:8, distance:7.00};
        paths = vec![ab.clone(),ah.clone(),bc.clone(),bh.clone(),cd.clone(),cf.clone(),ci.clone(),de.clone(),df.clone(),ef.clone(),fg.clone(),gh.clone(),gi.clone(),hi.clone()];
        let minimum_spanning_tree : Vec<Path> = vec![gh,ci,fg,ab,cf,cd,ah,de];
        let tour : Vec<City> = vec![g,f,c,d,e,i,h,a,b];
        let result = approx_tsp::approximation_tsp(&cities, &minimum_spanning_tree);
        assert_eq!(tour.len(), result.len());
        for i in 0..tour.len() {
            assert_eq!(tour[i], result[i]);
        }
    }
}
