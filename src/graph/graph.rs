use std::collections::HashMap;
use super::super::point::point::Point;

pub struct Graph {
    // A Hashmap for storing pointers to the points of graphs with their id as key
    points: HashMap<u128, Box<Point>>,
    // A Hashmap for storing the relations between a point and all related points with the id of the point as key
    relations: HashMap<u128, Vec<u128>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            points: HashMap::new(),
            relations: HashMap::new(),
        }
    }

    /// Adds a point to the graph
    pub fn create_point(&mut self) -> u128 {
        let point = Point::new();
        let id = point.get_id();
        self.points.insert(id, Box::new(point));
        id
    }

    /// Allows for duplicate relations or 'multi-edges'
    pub fn create_relation(&mut self, point1: u128, point2: u128) {
        if self.points.contains_key(&point1) && self.points.contains_key(&point2) {
            if self.relations.contains_key(&point1) {
                self.relations.get_mut(&point1).unwrap().push(point2);
            } else {
                self.relations.insert(point1, vec![point2]);
            }
        } else {
            panic!("You are trying to create a relation between two points,
             where at least one of the points does not exist");
        }
    }

    /// Returns a vector of all the points' ids in the graph
    pub fn get_points(&self) -> Vec<u128> {
        let mut points = Vec::new();
        for point in self.points.keys() {
            points.push(*point);
        }
        points
    }

    pub fn get_relations_of_point(&self, point: u128) -> Vec<u128> {
        if self.relations.contains_key(&point) {
            self.relations.get(&point).unwrap().clone()
        } else {
            Vec::new()
        }
    }

    
}