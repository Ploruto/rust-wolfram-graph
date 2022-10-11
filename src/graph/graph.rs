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

    pub fn add_relations(&mut self, points1: Vec<Box<Point>>, points2: Vec<Box<Point>>) {
        if points1.len() != points2.len() {
            panic!("The number of points in the first vector must be equal to the number of points in the second vector");
        }
        for i in 0..points1.len() {
            self.add_relation(points1[i].clone(), points2[i].clone());
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

    pub fn is_point_connected_to_points(&self, point: Box<Point>, points: Vec<Box<Point>>) -> bool {
        for point2 in points {
            if !self.is_point_connected_to_point(point.clone(), point2) {
                return false;
            }
        }
        true
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
    pub fn get_possible_variable_resolutions(
        &self,
        find_pattern: Vec<Vec<Variable>>,
    ) -> Vec<HashMap<Variable, Box<Point>>> {
        let requirements = Rule::get_find_pattern_requirements(&find_pattern);
        let distinct_variables: Vec<Variable> =
            Rule::get_distinct_variables_in_pattern(find_pattern);
        // points_that_fit_level_one_requirements .: points that have the same number of connections as the variables
        let mut points_that_fit_level_one_requirements: HashMap<Variable, Vec<Box<Point>>> =
            HashMap::new();
        for variable in &distinct_variables {
            let mut points_that_fit_level_one_requirements_for_variable: Vec<Box<Point>> =
                Vec::new();
            for (point, _) in self.points_with_relations.iter() {
                if self
                    .points_with_relations
                    .get(point)
                    .unwrap_or(&HashSet::new())
                    .len()
                    >= requirements.get(&variable).unwrap_or(&Vec::new()).len()
                {
                    points_that_fit_level_one_requirements_for_variable.push(point.clone());
                }
            }
            points_that_fit_level_one_requirements.insert(
                variable.clone(),
                points_that_fit_level_one_requirements_for_variable.clone(),
            );
        }
        // for ex .: when the pattern is [a,b] [a, c] [a, d]: a is relatively small because it's only the Points that have 3 connections
        // that means be resolving the variables in order from lowest possible points to highest possible points saves computation time

        // sort the points_that_fit_level_one_requirements by the length of the Vec<Box<Point>> into the a Vec<(Variable, Vec<Box<Point>>)>
        let mut sorted_points_that_fit_level_one_requirements: Vec<(Variable, Vec<Box<Point>>)> =
            Vec::new();
        for (variable, points) in points_that_fit_level_one_requirements.iter() {
            sorted_points_that_fit_level_one_requirements.push((variable.clone(), points.clone()));
        }
        sorted_points_that_fit_level_one_requirements.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
        println!(
            "sorted at the beginning: {:?}",
            sorted_points_that_fit_level_one_requirements[0].1.len()
        );

        // at this point we have to start resolving the variables
        // we will start with the variable that has the least number of possible points
        // we will then try to resolve the other variables by checking if the points that are connected to the point that we are trying to resolve
        // are also connected to the other points that we are trying to resolve
        // if they are, then we can add the point to the possible resolutions
        // if they are not, then we can't add the point to the possible resolutions

        let mut possible_resolutions: Vec<HashMap<Variable, Box<Point>>> = Vec::new();
        // we will use a Mapping between the variable and the point that we are trying to resolve
        let mut current_resolution: HashMap<Variable, Box<Point>> = HashMap::new();
        
        self.resolve_variable(&sorted_points_that_fit_level_one_requirements, &distinct_variables, &current_resolution, &mut possible_resolutions);
        

        return Vec::new();
    }

    /* for every variable in order of the sorted_points_that_fit_level_one_requirements
    we'll need a loop that goes like this:
    for every point in the points_that_fit_level_one_requirements_for_variable
        if the point is not already in the current_resolution
            if the point is connected to all the points in the current_resolution
                add the point to the current_resolution
                if the current_resolution has the same number of elements as the distinct_variables
                    add the current_resolution to the possible_resolutions
                else
                    call the function recursively with the next variable
    */

    fn resolve_variable(
        &self,
        sorted_points_that_fit_level_one_requirements: &Vec<(Variable, Vec<Box<Point>>)>,
        distinct_variables: &Vec<Variable>,
        current_resolution: &HashMap<Variable, Box<Point>>,
        possible_resolutions: &mut Vec<HashMap<Variable, Box<Point>>>,
    ) {
        for (variable, points_that_fit_level_one_requirements_for_variable) in
            sorted_points_that_fit_level_one_requirements
        {
            for point in points_that_fit_level_one_requirements_for_variable {
                if !current_resolution.contains_key(&variable) {
                    if self.is_point_connected_to_points(
                        point.clone(),
                        current_resolution
                        .clone()
                            .iter()
                            .map(|(variable, point)| point.clone())
                            .collect(),
                    ) {
                        let mut new_current_resolution = current_resolution.clone();
                        new_current_resolution.insert(variable.clone(), point.clone());
                        if new_current_resolution.len() == distinct_variables.len() {
                            possible_resolutions.push(new_current_resolution);
                        } else {
                            self.resolve_variable(
                                &sorted_points_that_fit_level_one_requirements.clone(),
                                &distinct_variables,
                                &new_current_resolution,
                                possible_resolutions,
                            );
                        }
                    }
                }
            }
        }
    }

    fn check_if_resulution_is_valid(
        &self,
        current_resolution: &HashMap<Variable, Box<Point>>,
        requirements: &HashMap<Variable, Vec<Variable>>,
    ) -> bool {
        for (variable, point) in current_resolution.iter() {
            for required_variable in requirements.get(variable).unwrap_or(&Vec::new()) {
                if !self.is_point_connected_to_point(
                    point.clone(),
                    current_resolution
                        .get(required_variable)
                        .unwrap_or(&Box::new(Point::new()))
                        .clone(),
                ) {
                    return false;
                }
            }
        }
        true
    }
}
