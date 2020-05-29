extern crate image;
extern crate petgraph;

use image::{GenericImageView};

use petgraph::graph::{Graph};
use petgraph::dot::{Dot, Config};

use std::collections::HashMap;


type G = petgraph::Graph<(u32, u32, image::Rgba<u8>), u32, petgraph::Undirected, u32>;
type N = petgraph::prelude::NodeIndex;

fn main() {
    let img = image::open("data/letter_p.jpg").unwrap();
    let mut img_graph: G = Graph::new_undirected();
    let scan_range: u32 = 5;
    let threshhold: f64 = 30.0;
    let dimension_factor = 10;
    let color_factor = 1;
    let mut node_map = HashMap::new();
    for pixel in img.pixels(){
        //Generate graph from pixel data
        let base_node = img_graph.add_node(pixel);
        let (base_x, base_y, base_rgba) = pixel;
        let (base_r, base_g, base_b) = (base_rgba[0], base_rgba[1], base_rgba[2]);
        node_map.entry((base_x, base_y)).or_insert(base_node); 
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
                let (link_x, link_y, link_rgba) = img_graph.node_weight(*link_node).unwrap();
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
    let mut removed_nodes: Vec<N> = vec![];
    for base in img_graph.node_indices(){
        if removed_nodes.contains(&base) == false{
            let base_neighbors: Vec<N> = img_graph.neighbors(base).collect();
            for link in &base_neighbors{
                if removed_nodes.contains(link) == false{
                    let link_neighbors: Vec<N> = img_graph.neighbors(*link).collect();
                    let shared_neighbors: Vec<&N> = base_neighbors.iter().filter(|n| link_neighbors.contains(n)).collect();
                    println!("base_neighbors: {:?}\nlink_neighbors: {:?}\nshared neighbors: {:?}", base_neighbors.len(), link_neighbors.len(), shared_neighbors.len());
                    //if shared_neighbors.len() > 0{
                        for link_neighbor in &link_neighbors{
                            if shared_neighbors.contains(&link_neighbor) == false && link_neighbor != &base{
                                if img_graph.find_edge_undirected(base, *link_neighbor) == None{
                                    let edge = img_graph.find_edge_undirected(*link, *link_neighbor).unwrap();
                                    img_graph.add_edge(base, *link_neighbor, *img_graph.edge_weight(edge.0).unwrap());
                                }
                            }
                        }
                        img_graph.remove_node(*link);
                        removed_nodes.push(*link);
                    //}
                }
            }
        }
    }


    //println!("Graph: {:?}", debug_array);
    use std::fs::File;
    use std::io::Write;
    let mut f = File::create("test.dot").unwrap();
    //println!("{:?}", Dot::with_config(&img_graph, &[Config::EdgeNoLabel]));
    //let condensed_graph = condensation(img_graph, false);
    let output = format!("{:?}", Dot::with_config(&img_graph, &[Config::NodeIndexLabel]));
    f.write_all(&output.as_bytes());
}


fn merge_nodes(mut g: petgraph::Graph<(u32, u32, image::Rgba<u8>), u32, petgraph::Undirected>, node_1: N, node_2: N){ 
    //Move edges from node_2 onto node_1, sum edge weight on shared edges, and delete node_2 from graph.
    let node_2_neighbors: Vec<N> = g.neighbors(node_2).collect();
    for node_2_neighbor in node_2_neighbors{
        let edge = g.find_edge_undirected(node_1, node_2_neighbor);
        let edge_2 = g.find_edge_undirected(node_2, node_2_neighbor).unwrap();
        let edge_2_weight = *g.edge_weight(edge_2.0).unwrap();
        if edge != None{
            let edge_weight = g.edge_weight_mut(edge.unwrap().0).unwrap();
            *edge_weight += edge_2_weight;
        }else{
            if node_2_neighbor != node_1{
                g.add_edge(node_1, node_2_neighbor, edge_2_weight);
            }
        }
    }
    g.remove_node(node_2);
}