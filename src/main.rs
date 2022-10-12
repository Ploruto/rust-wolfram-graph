mod point;
use std::collections::HashMap;
mod graph;
mod rule;
use rand::Rng;
use rule::rule::{Rule, Variable};
use std::fs::File;
use std::io::prelude::*;

fn main(){
    // create a graph with 2000 points and 4000 random relations between the existing points
    let mut graph = graph::graph::Graph::new();
    for _ in 0..40 {
        graph.create_point();
    }
    let mut rng = rand::thread_rng();

    let all_points = graph.get_points();
    let mut points1 = Vec::new();
    let mut points2 = Vec::new();
    for _ in 0..200 {
        let index1 = rng.gen_range(0..all_points.len());
        let index2 = rng.gen_range(0..all_points.len());
        let point1 = all_points.keys().nth(index1).unwrap().clone();
        let point2 = all_points.keys().nth(index2).unwrap().clone();
        points1.push(point1);
        points2.push(point2)
    }
    graph.add_relations(points1, points2);

    // connect the first point to the 2nd, 3rd and 4th point
    let first_point = graph.get_points().keys().nth(0).unwrap().clone();
    let second_point = graph.get_points().keys().nth(1).unwrap().clone();
    let third_point = graph.get_points().keys().nth(2).unwrap().clone();
    let fourth_point = graph.get_points().keys().nth(3).unwrap().clone();
    graph.add_relation(first_point.clone(), second_point.clone());
    graph.add_relation(first_point.clone(), third_point);
    graph.add_relation(first_point, fourth_point.clone());
    graph.add_relation(second_point, fourth_point);

    let find_pattern: Vec<Vec<Variable>> = vec![vec![Variable::a, Variable::b], vec![Variable::a, Variable::c], vec![Variable::a, Variable::d], vec![Variable::b, Variable::d]];
    let find_pattern2 = find_pattern.clone();
    let r = graph.get_possible_variable_resolutions(find_pattern);

    println!("Found {:?} possible resolutions", r.len());
    // print the first resolution



    

    // print the find_pattern2
    println!("Find pattern: {:?}", find_pattern2);
    

    let mut w = File::create("D:\\Coding\\Rust\\graph_creation\\connections.txt").unwrap();
    writeln!(&mut w, "formatted {:?}", graph.get_all_connections()).unwrap();

    let mut w = File::create("D:\\Coding\\Rust\\graph_creation\\results.txt").unwrap();
    writeln!(&mut w, "formatted {:?}", r).unwrap();
    

}
