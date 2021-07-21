use crate::graph::path::Path as Path;
use crate::graph::city::City as City;
use crate::algorithms::kruskal::kruskal as kruskal;

pub fn approximation_tsp(cities: &mut Vec<City>, paths: &mut Vec<Path>) -> Vec<City>{
    let mut new_cities : Vec<City> = Vec::new();
    let minimum_spanning_tree = kruskal::kruskal_algorithm(cities, paths);
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
    stack.insert(0, root.city_1.id);
    while stack.len() > 0 {
        let current = stack.remove(0);
        let is_marked = marked_cities.iter().find(|&&x| x == current);
        match is_marked {
            Some(_) => continue,
            None => {},
        }
        let mut neighbors_path = tree.iter().filter(|x| x.city_1.id == current || x.city_2.id == current);
        let mut iter = neighbors_path.next();
        while iter != None {
            let path = iter.unwrap();
            let is_marked = maked_paths.iter().find(|&x|x == path);
            match is_marked {
                Some(_) => {},
                None => maked_paths.push(path.clone()),
            }
            if current != path.city_2.id {
                stack.insert(0, path.city_2.id);
            } else {
                stack.insert(0, path.city_1.id);
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
    use crate::graph::city::City as City;
    use crate::graph::path::Path as Path;
    use crate::algorithms::approx_tsp::approx_tsp as approx_tsp;

    #[test]
    fn test_approximation_tsp(){
        let a = City{id:0, name: String::from("a"), population:1, country: String::from("A"), latitude: 0.00, longitude: 0.00};
        let b = City{id:1, name: String::from("b"), population:1, country: String::from("B"), latitude: 0.00, longitude: 0.00};
        let c = City{id:2, name: String::from("c"), population:1, country: String::from("C"), latitude: 0.00, longitude: 0.00};
        let d = City{id:3, name: String::from("d"), population:1, country: String::from("D"), latitude: 0.00, longitude: 0.00};
        let e = City{id:4, name: String::from("e"), population:1, country: String::from("E"), latitude: 0.00, longitude: 0.00};
        let f = City{id:5, name: String::from("f"), population:1, country: String::from("F"), latitude: 0.00, longitude: 0.00};
        let g = City{id:6, name: String::from("g"), population:1, country: String::from("G"), latitude: 0.00, longitude: 0.00};
        let h = City{id:7, name: String::from("h"), population:1, country: String::from("H"), latitude: 0.00, longitude: 0.00};
        let i = City{id:8, name: String::from("i"), population:1, country: String::from("I"), latitude: 0.00, longitude: 0.00};
        let mut cities : Vec<City> = vec![a.clone(),b.clone(),c.clone(),d.clone(),e.clone(),f.clone(),g.clone(),h.clone(),i.clone()];
        let ab = Path{city_1:a.clone(), city_2:b.clone(), distance:4.00};
        let ah = Path{city_1:a.clone() ,city_2:h.clone(), distance:8.00};
        let bc = Path{city_1:b.clone(), city_2:c.clone(), distance:8.00};
        let bh = Path{city_1:b.clone(), city_2:h.clone(), distance:11.00};
        let cd = Path{city_1:c.clone(), city_2:d.clone(), distance:7.00};
        let cf = Path{city_1:c.clone(), city_2:f.clone(), distance:4.00};
        let ci = Path{city_1:c.clone(), city_2:i.clone(), distance:2.00};
        let de = Path{city_1:d.clone(), city_2:e.clone(), distance:9.00};
        let df = Path{city_1:d.clone(), city_2:f.clone(), distance:14.00};
        let ef = Path{city_1:e.clone(), city_2:f.clone(), distance:10.00};
        let fg = Path{city_1:f.clone(), city_2:g.clone(), distance:2.00};
        let gh = Path{city_1:g.clone(), city_2:h.clone(), distance:1.00};
        let gi = Path{city_1:g.clone(), city_2:i.clone(), distance:6.00};
        let hi = Path{city_1:h.clone(), city_2:i.clone(), distance:7.00};
        let mut paths : Vec<Path> = vec![ab.clone(),ah.clone(),bc.clone(),bh.clone(),cd.clone(),cf.clone(),ci.clone(),de.clone(),df.clone(),ef.clone(),fg.clone(),gh.clone(),gi.clone(),hi.clone()];
        let tour : Vec<City> = vec![g,f,c,d,e,i,h,a,b];
        let result = approx_tsp::approximation_tsp(&mut cities, &mut paths);
        assert_eq!(tour.len(), result.len());
        for i in 0..tour.len() {
            assert_eq!(tour[i], result[i]);
        }
    }
}
