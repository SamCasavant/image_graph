/*
WIP:
Draw edges to an image
Try connecting nodes with no neighbors to adjacent nodes by default 
OR use relative association method

*/

extern crate image;
extern crate petgraph;

use image::{GenericImageView};

use petgraph::graph::{Graph};
use petgraph::dot::{Dot, Config};
use petgraph::algo::{kosaraju_scc};

use std::collections::HashMap;
use clap::{Arg, App};


type G = petgraph::Graph<u32, u32, petgraph::Undirected, u32>;

fn main() {
    let args = App::new("Image Graph")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("image to operate on")
            .required(true))
        .arg(Arg::with_name("scan range")
            .short("r")
            .long("range")
            .takes_value(true)
            .help("range to search for edges (unsigned int 32)")
            .default_value("5"))
        .arg(Arg::with_name("threshhold")
            .short("t")
            .long("threshhold")
            .takes_value(true)
            .help("maximum distance to draw an edge (float 64)")
            .default_value("30.0"))
        .arg(Arg::with_name("spatial factor")
            .short("s")
            .long("space")
            .takes_value(true)
            .help("multiplier for spatial distances (i64)")
            .default_value("10"))
        .arg(Arg::with_name("color factor")
            .short("c")
            .long("color")
            .takes_value(true)
            .help("multiplier for color distances (i64)")
            .default_value("1"))
        .get_matches();

    let path = args.value_of("file").unwrap();
    let img = image::open(path).unwrap();
    let mut img_graph: G = Graph::new_undirected();

    let range_arg = args.value_of("scan range");
    let scan_range = match range_arg.unwrap_or("5").parse::<u32>() {
        Ok(n) => n,
        Err(_) => panic!("Invalid scan range: {}", range_arg.unwrap())
    };

    let thresh_arg = args.value_of("thresshold");
    let threshhold = match thresh_arg.unwrap_or("30.0").parse::<f64>() {
        Ok(n) => n,
        Err(_) => panic!("Invalid threshhold: {}", thresh_arg.unwrap())
    };

    let dimension_arg = args.value_of("spatial factor");
    let dimension_factor = match dimension_arg.unwrap_or("10").parse::<i64>() {
        Ok(n) => n,
        Err(_) => panic!("Invalid spatial factor: {}", dimension_arg.unwrap())
    };

    let color_arg = args.value_of("color factor");
    let color_factor = match color_arg.unwrap_or("1").parse::<i64>() {
        Ok(n) => n,
        Err(_) => panic!("Invalid color factor: {}", color_arg.unwrap())
    };
    

    let mut node_map = HashMap::new();
    let mut pixel_node_map = HashMap::new();
    for pixel in img.pixels(){
        //Generate graph from pixel data
        let pixel_index = pixel.0 + img.height()*pixel.1;
        let base_node = img_graph.add_node(pixel_index);
        let (base_x, base_y, base_rgba) = pixel;
        let (base_r, base_g, base_b) = (base_rgba[0], base_rgba[1], base_rgba[2]);
        node_map.entry((base_x, base_y)).or_insert(base_node); 
        pixel_node_map.entry(base_node).or_insert(pixel);
        let mut neighbors = vec![];
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = img.width();
        let mut max_y = img.height();
        if base_x > scan_range {
            min_x = base_x - scan_range;
        }
        if base_y > scan_range {
            min_y = base_y - scan_range;
        }
        if base_x + scan_range < img.width(){
            max_x = base_x + scan_range;
        }
        if base_y + scan_range < img.height(){
            max_y = base_y + scan_range;
        }
        for x in min_x..max_x{
            for y in min_y..max_y{
                let neighbor = node_map.get(&(x,y));
                if neighbor != None && (x, y) != (base_x, base_y){
                    neighbors.push(neighbor.unwrap());
                }
            }
        }
        for link_node in neighbors{
            if img_graph.contains_edge(base_node, *link_node) == false{
                let (link_x, link_y, link_rgba) = pixel_node_map.get(link_node).unwrap();
                let (link_r, link_g, link_b) = (link_rgba[0], link_rgba[1], link_rgba[2]);
                let raw_distance =  ((base_x as i64 - *link_x as i64) * dimension_factor).pow(2) +
                                    ((base_y as i64 - *link_y as i64) * dimension_factor).pow(2) +
                                    ((base_r as i64 - link_r as i64) * color_factor).pow(2) +
                                    ((base_g as i64 - link_g as i64) * color_factor).pow(2)  + 
                                    ((base_b as i64 - link_b as i64) * color_factor).pow(2);
                let linear_distance = (raw_distance as f64).sqrt();
                if linear_distance < threshhold{
                    let linear_weight = (10000.0/linear_distance) as u32;
                    img_graph.add_edge(base_node, *link_node, linear_weight);
                }
            }
        }
    }

    //Merge nodes that have high shared connectivity
    // let mut removed_nodes: Vec<N> = vec![];
    // for base in img_graph.node_indices(){
    //     if removed_nodes.contains(&base) == false{
    //         let base_neighbors: Vec<N> = img_graph.neighbors(base).collect();
    //         for link in &base_neighbors{
    //             if removed_nodes.contains(link) == false{
    //                 let link_neighbors: Vec<N> = img_graph.neighbors(*link).collect();
    //                 let shared_neighbors: Vec<&N> = base_neighbors.iter().filter(|n| link_neighbors.contains(n)).collect();
    //                 println!("base_neighbors: {:?}\nlink_neighbors: {:?}\nshared neighbors: {:?}", base_neighbors.len(), link_neighbors.len(), shared_neighbors.len());
    //                 //if shared_neighbors.len() > 0{
    //                     merge_nodes(&mut img_graph, base, *link);
    //                     removed_nodes.push(*link);
    //                 //}
    //             }
    //         }
    //     }
    // }
    let subgraphs = kosaraju_scc(&img_graph);

    use std::fs::File;
    use std::io::Write;

    for subgraph in &subgraphs {
        // left and right HashMaps map(y) = x, top and bottom map(x) = y
        let mut left_edge = HashMap::new();
        // let mut right_edge = HashMap::new();
        // let mut top_edge = HashMap::new();
        // let mut bottom_edge = HashMap::new();
        for node in subgraph {
            let (x, y, rgb) = pixel_node_map.get(&node).unwrap();
            if left_edge.get(y).is_some() {
                if x < left_edge.get(y).unwrap(){
                    let map_x =  left_edge.entry(y).or_insert(*x);
                    *map_x = *x;
                }   
            } else {
                left_edge.entry(y).or_insert(*x);
            }
            // if right_edge.get(y).is_some() {
            //     if x < right_edge.get(y).unwrap(){
            //         let map_x =  right_edge.entry(y).or_insert(*x);
            //         *map_x = *x;
            //     }   
            // } else {
            //     right_edge.entry(y).or_insert(*x);
            // }
            // if top_edge.get(y).is_some() {
            //     if x < top_edge.get(y).unwrap(){
            //         let map_x =  top_edge.entry(y).or_insert(*x);
            //         *map_x = *x;
            //     }   
            // } else {
            //     top_edge.entry(y).or_insert(*x);
            // }
            // if bottom_edge.get(y).is_some() {
            //     if x < bottom_edge.get(y).unwrap(){
            //         let map_x =  bottom_edge.entry(y).or_insert(*x);
            //         *map_x = *x;
            //     }   
            // } else {
            //     bottom_edge.entry(y).or_insert(*x);
            // }
        }
    }

    for (i, subgraph) in subgraphs.iter().enumerate(){
        let mut temp_subgraph: G = Graph::new_undirected();
        let mut added_nodes_map = HashMap::new();
        for node in subgraph{
            let new_index = temp_subgraph.add_node(node.index() as u32);
            added_nodes_map.entry(*node).or_insert(new_index);
            for neighbor in img_graph.neighbors(*node){
                let new_connection = added_nodes_map.get(&neighbor);
                if new_connection != None {
                    if temp_subgraph.contains_edge(new_index, *new_connection.unwrap()) == false{
                        temp_subgraph.add_edge(new_index, *new_connection.unwrap(), 1);
                    }
                }
            }
        }
        if temp_subgraph.edge_count() > 2{
            let filename = format!("out/subgraph_{}.dot", i);
            let mut f = File::create(filename).unwrap();
            let output = format!("{:?}", Dot::with_config(&temp_subgraph, &[Config::NodeIndexLabel]));
            f.write_all(&output.as_bytes());
        }
    }
    // let mut f = File::create("test.dot").unwrap();
    // //println!("{:?}", Dot::with_config(&img_graph, &[Config::EdgeNoLabel]));
    // //let condensed_graph = condensation(img_graph, false);
    // let output = format!("{:?}", Dot::with_config(&img_graph, &[Config::NodeIndexLabel]));
    // f.write_all(&output.as_bytes());
}


// fn merge_nodes(g: &mut petgraph::Graph<u32, u32, petgraph::Undirected>, node_1: N, node_2: N){ 
//     //Move edges from node_2 onto node_1, sum edge weight on shared edges, and delete node_2 from graph.
//     let node_2_neighbors: Vec<N> = g.neighbors(node_2).collect();
//     for node_2_neighbor in node_2_neighbors{
//         let edge = g.find_edge_undirected(node_1, node_2_neighbor);
//         let edge_2 = g.find_edge_undirected(node_2, node_2_neighbor).unwrap();
//         let edge_2_weight = g.edge_weight(edge_2.0).unwrap();
//         if edge != None{
//             let mut edge_weight = *g.edge_weight_mut(edge.unwrap().0).unwrap();
//             edge_weight += edge_2_weight;
//         }else{
//             if node_2_neighbor != node_1{
//                 g.add_edge(node_1, node_2_neighbor, *edge_2_weight);
//             }
//         }
//     }
//     g.remove_node(node_2);
// }