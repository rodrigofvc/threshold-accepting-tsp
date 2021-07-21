use crate::heuristics::city::City as City;
use crate::heuristics::path::Path as Path;
use super::disjoint_set::DisjointSet as DisjointSet;

pub fn kruskal_algorithm(cities: &mut Vec<City>, paths: &mut Vec<Path>) -> Vec<Path> {
    let mut disjoint_sets : Vec<DisjointSet> = Vec::new();
    let mut tree : Vec<Path> = Vec::new();

    for city in cities.clone() {
        disjoint_sets.push(make_set(city.clone()));
    }
    paths.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for path in paths.clone() {
        let city_1 = cities.iter().find(|x| x.id == path.id_city_1).unwrap();
        let city_2 = cities.iter().find(|x| x.id == path.id_city_2).unwrap();
        let i = disjoint_sets.iter().position(|x| x.city.id == city_1.id).unwrap();
        let j = disjoint_sets.iter().position(|x| x.city.id == city_2.id).unwrap();

        let disjoint_set_1 = &disjoint_sets[i];
        let disjoint_set_2 = &disjoint_sets[j];

        if find_set(disjoint_set_1.clone(), &disjoint_sets).city.id != find_set(disjoint_set_2.clone(), &disjoint_sets).city.id {
            tree.push(path.clone());
            union(&mut disjoint_sets, i, j);
        }
    }
    return tree;
}

fn union(disjoint_sets: &mut Vec<DisjointSet>, i: usize, j: usize) {
    let represent = find_set(disjoint_sets[j].clone(), disjoint_sets);
    let represent_pos = disjoint_sets.iter().position(|x| x.city == represent.city).unwrap();
    let mut new = disjoint_sets[represent_pos as usize].clone();
    new.parent = i as i32;
    disjoint_sets[represent_pos as usize] = new;
}

fn find_set(current: DisjointSet, disjoint_sets: &Vec<DisjointSet>) -> DisjointSet {
    match current.parent {
        -1 => return current,
        n => find_set(disjoint_sets[n as usize].clone(), disjoint_sets),
    }
}

fn make_set(city: City) -> DisjointSet {
    DisjointSet { parent: -1, city: city }
}


#[cfg(test)]
mod test {
    use crate::heuristics::city::City as City;
    use crate::heuristics::path::Path as Path;
    use crate::algorithms::kruskal::disjoint_set::DisjointSet as DisjointSet;
    use crate::algorithms::kruskal::kruskal as kruskal;

    #[test]
    fn test_find_set() {
        let a = City{id:0, name: String::from("a"), population:1, country: String::from("A"), latitude: 0.00, longitude: 0.00};
        let b = City{id:1, name: String::from("b"), population:1, country: String::from("B"), latitude: 0.00, longitude: 0.00};
        let c = City{id:2, name: String::from("c"), population:1, country: String::from("C"), latitude: 0.00, longitude: 0.00};
        let d = City{id:3, name: String::from("d"), population:1, country: String::from("D"), latitude: 0.00, longitude: 0.00};
        let e = City{id:4, name: String::from("e"), population:1, country: String::from("E"), latitude: 0.00, longitude: 0.00};
        let s_a = kruskal::make_set(a);
        let s_b = kruskal::make_set(b);
        let s_c = kruskal::make_set(c);
        let s_d = kruskal::make_set(d);
        let s_e = kruskal::make_set(e);
        let mut sets : Vec<DisjointSet> = vec![s_a,s_b,s_c,s_d,s_e];
        kruskal::union(&mut sets, 0, 1);
        kruskal::union(&mut sets, 2, 3);
        kruskal::union(&mut sets, 3, 4);

        let s_a = &sets[0];
        let s_b = &sets[1];
        let s_c = &sets[2];
        let s_d = &sets[3];
        let s_e = &sets[4];

        let f_1 = kruskal::find_set(s_c.clone(), &sets);
        assert!( f_1.city == s_c.city );

        let f_2 = kruskal::find_set(s_d.clone(), &sets);
        assert!( f_2.city == s_c.city );

        let f_3 = kruskal::find_set(s_e.clone(), &sets);
        assert!( f_3.city == s_c.city );

        kruskal::union(&mut sets, 1, 4);

        let s_a = &sets[0];
        let s_b = &sets[1];
        let s_c = &sets[2];
        let s_d = &sets[3];
        let s_e = &sets[4];

        let f_1 = kruskal::find_set(s_c.clone(), &sets);
        assert!( f_1.city == s_a.city );

        let f_2 = kruskal::find_set(s_d.clone(), &sets);
        assert!( f_2.city == s_a.city );

        let f_3 = kruskal::find_set(s_e.clone(), &sets);
        assert!( f_3.city == s_a.city );
    }

    #[test]
    fn test_union_set() {
        let a = City{id:0, name: String::from("a"), population:1, country: String::from("A"), latitude: 0.00, longitude: 0.00};
        let b = City{id:1, name: String::from("b"), population:1, country: String::from("B"), latitude: 0.00, longitude: 0.00};
        let c = City{id:2, name: String::from("c"), population:1, country: String::from("C"), latitude: 0.00, longitude: 0.00};
        let d = City{id:3, name: String::from("d"), population:1, country: String::from("D"), latitude: 0.00, longitude: 0.00};
        let e = City{id:4, name: String::from("e"), population:1, country: String::from("E"), latitude: 0.00, longitude: 0.00};
        let s_a = kruskal::make_set(a);
        let s_b = kruskal::make_set(b);
        let s_c = kruskal::make_set(c);
        let s_d = kruskal::make_set(d);
        let s_e = kruskal::make_set(e);
        let mut sets : Vec<DisjointSet> = vec![s_a,s_b,s_c,s_d,s_e];
        kruskal::union(&mut sets, 0, 1);
        kruskal::union(&mut sets, 0, 2);
        kruskal::union(&mut sets, 2, 3);
        kruskal::union(&mut sets, 3, 4);
        let s_a = &sets[0];
        let s_b = &sets[1];
        let s_c = &sets[2];
        let s_d = &sets[3];
        let s_e = &sets[4];

        assert!(s_a.parent == -1);
        assert!((&sets[s_b.parent as usize]).city == s_a.city);
        assert!((&sets[s_c.parent as usize]).city == s_a.city);
        assert!((&sets[s_d.parent as usize]).city == s_c.city);
        assert!((&sets[s_e.parent as usize]).city == s_d.city);

    }

    #[test]
    fn test_minimum_spanning_tree(){
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
        cities = vec![a,b,c,d,e,f,g,h,i];
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
        let minimum_spanning_tree : Vec<Path> = vec![ab,ah,ci,gh,fg,cf,cd,de];
        let result : Vec<Path> = kruskal::kruskal_algorithm(&mut cities, &mut paths);
        for path in &minimum_spanning_tree {
            let is_contain = result.iter().find(|&x| x == path);
            match is_contain {
                Some(_) => {},
                None => panic!(),
            }
        }
        assert_eq!(minimum_spanning_tree.len(), result.len());
    }

}
