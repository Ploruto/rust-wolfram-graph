use super::super::point::point::Point;
use super::super::rule::rule::{Rule, Variable};
use std::collections::{HashMap, HashSet};

pub struct Graph {
    points_with_relations: HashMap<Box<Point>, HashSet<Box<Point>>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            points_with_relations: HashMap::new(),
        }
    }

    pub fn add_point(&mut self, point: Box<Point>) {
        self.points_with_relations.insert(point, HashSet::new());
    }

    pub fn add_relation(&mut self, point1: Box<Point>, point2: Box<Point>) {
        if self.points_with_relations.contains_key(&point1) {
            self.points_with_relations
                .get_mut(&point1)
                .unwrap()
                .insert(point2);
        }
    }

    pub fn remove_relation(&mut self, point1: Box<Point>, point2: Box<Point>) {
        if self.points_with_relations.contains_key(&point1) {
            self.points_with_relations
                .get_mut(&point1)
                .unwrap()
                .remove(&point2);
        }
    }

    pub fn create_point(&mut self) -> Box<Point> {
        let point = Box::new(Point::new());
        self.add_point(point.clone());
        point
    }

    pub fn is_point_connected_to_point(&self, point1: Box<Point>, point2: Box<Point>) -> bool {
        if self.points_with_relations.contains_key(&point1) {
            self.points_with_relations
                .get(&point1)
                .unwrap()
                .contains(&point2)
        } else {
            false
        }
    }

    pub fn get_points(&self) -> &HashMap<Box<Point>, HashSet<Box<Point>>> {
        &self.points_with_relations
    }

    /* Given all the points in the graph, find all combinations of points that satisfy the requirements
       the requirements are given as a HashMap<Variable, Vec<Variable>>, where the key is the variable
       and it must be connected to all the variables in the Vec<Variable>
       every variable will map to a distinc element

       for ex.: requirements = {a: [b, c], b: [c, d], c: [d, e]}
       then we are trying to find a point (a) that is connected to a point (b) and a point (c)
       ,where b is connected to c and d, and c is connected to d and e

    */
    pub fn get_possible_variable_resolutions(&self, find_pattern: Vec<Vec<Variable>>) -> Vec<HashMap<Variable, Box<Point>>> {
        let mut possible_resolutions: Vec<HashMap<Variable, Box<Point>>> = Vec::new();
        let mut point_not_suitable_for_variable: HashSet<(Box<Point>, Variable)> = HashSet::new();
        let requirements = Rule::get_find_pattern_requirements(&find_pattern);
        let distinct_variables: Vec<Variable> = Rule::get_distinct_variables_in_pattern(find_pattern);
        for (point, point_connections) in self.points_with_relations.iter() {
            let mut possible_resolution: HashMap<Variable, Box<Point>>= HashMap::new();
            while possible_resolution.len() < distinct_variables.len() {
                let mut variable = distinct_variables[possible_resolution.len()];

            }
            
        }
        possible_resolutions
    }
}
