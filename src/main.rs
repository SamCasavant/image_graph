extern crate image;
extern crate petgraph;

use image::{GenericImageView};

use petgraph::graph::{Graph};
use petgraph::dot::{Dot, Config};

use std::collections::HashMap;

// #[derive(Hash, Eq, PartialEq, Debug)]
// struct node_map {
//     coordinates: (u32, u32),
//     node: u32,
// }

// impl node_map {
//     fn new(coordinates: (u32, u32), node: u32) -> node_map {
//         node_map {coordinates: coordinates, node: node }
//     }
// }

fn main() {
    let img = image::open("data/small.jpg").unwrap();
    let mut img_graph: petgraph::Graph<(u32, u32, image::Rgba<u8>), f64, petgraph::Undirected, u32> = Graph::new_undirected();
    let scan_range: u32 = 4;
    let mut node_map = HashMap::new();
    for pixel in img.pixels(){
        let base_node = img_graph.add_node(pixel);
        //println!("{:?}", base_node);
        let threshhold: f64 = 30.0;
        let dimension_factor = 1;
        let color_factor = 1;
        let (base_x_u32, base_y_u32, base_rgba) = pixel;
        let (base_x, base_y) = (base_x_u32 as i64, base_y_u32 as i64);
        let (base_r, base_g, base_b) = (base_rgba[0] as i64, base_rgba[1] as i64, base_rgba[2] as i64);
        node_map.entry((base_x_u32, base_y_u32)).or_insert(base_node);
        let mut neighbors = vec![];//u32, 3, scan_range.pow(2)-1];
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = img.width();
        let mut max_y = img.height();
        if base_x_u32 > scan_range {
            min_x = base_x_u32 - scan_range;
        }
        if base_y_u32 > scan_range {
            min_y = base_y_u32 - scan_range;
        }
        if base_x_u32 + scan_range < img.width(){
            max_x = base_x_u32 + scan_range;
        }
        if base_y_u32 + scan_range < img.height(){
            max_y = base_y_u32 + scan_range;
        }
        for x in min_x..max_x{
            for y in min_y..max_y{
                let neighbor = node_map.get(&(x,y));
                if neighbor != None{
                    neighbors.push(neighbor.unwrap())
                }
            }
        }
        for link_node in neighbors{
            let (link_x_u32, link_y_u32, link_rgba) = img_graph.node_weight(*link_node).unwrap();
            let (link_x, link_y) = (*link_x_u32 as i64, *link_y_u32 as i64);
            let (link_r, link_g, link_b) = (link_rgba[0] as i64, link_rgba[1] as i64, link_rgba[2] as i64);

            if (((base_x-link_x).pow(2) + (base_y-link_y).pow(2)) as f64).sqrt() < 2.0{
                let raw_distance =  (base_x - link_x).pow(2) * dimension_factor +  
                                    (base_y - link_y).pow(2) * dimension_factor +
                                    (base_r - link_r).pow(2) * color_factor +
                                    (base_g - link_g).pow(2) * color_factor + 
                                    (base_b - link_b).pow(2) * color_factor;
                let linear_distance = (raw_distance as f64).sqrt();
                if linear_distance < threshhold{
                    img_graph.add_edge(base_node, *link_node, linear_distance);
                }
            }
        }
    }
    //println!("Graph: {:?}", debug_array);
    use std::fs::File;
    use std::io::Write;
    let mut f = File::create("out.dot").unwrap();
    //println!("{:?}", Dot::with_config(&img_graph, &[Config::EdgeNoLabel]));
    let output = format!("{:?}", Dot::with_config(&img_graph, &[Config::NodeIndexLabel]));
    f.write_all(&output.as_bytes());
}
