use crate::graph::city::City as City;

#[derive(Debug, Clone)]
pub struct DisjointSet {
    pub parent: i32,
    pub city: City,
}
