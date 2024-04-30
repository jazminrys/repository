extern crate csv;
extern crate petgraph;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::error::Error;
use petgraph::graph::{DiGraph, NodeIndex};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Airport {
    id: u32,
    name: String,
    city: String,
    flights: usize, 
}
impl Airport {
    fn new(id: u32, name: String, city: String) -> Self {
        Airport {
            id,
            name,
            city,
            flights: 0,
        }
    }
}
impl Ord for Airport {
    fn cmp(&self, other: &Self) -> Ordering {
        other.flights.cmp(&self.flights)
    }
}
impl PartialOrd for Airport {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "airport.csv";
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut graph = DiGraph::<Airport, ()>::new();
    let mut node_indices: HashMap<u32, NodeIndex> = HashMap::new();
    for result in rdr.records() {
        let record = result?;
        let origin_airport_id: u32 = record[2].parse()?;
        let origin = record[3].to_string();
        let origin_city_name = record[4].to_string();
        let dest_airport_id: u32 = record[5].parse()?;
        let dest = record[6].to_string();
        let dest_city_name = record[7].to_string();
        let origin_node_index = *node_indices
            .entry(origin_airport_id)
            .or_insert_with(|| {
                let airport = Airport::new(origin_airport_id, origin.clone(), origin_city_name.clone());
                graph.add_node(airport)
            });
        if let Some(origin_airport) = graph.node_weight_mut(origin_node_index) {
            origin_airport.flights += 1;
        }
        let dest_node_index = *node_indices
            .entry(dest_airport_id)
            .or_insert_with(|| {
                let airport = Airport::new(dest_airport_id, dest.clone(), dest_city_name.clone());
                graph.add_node(airport)
            });
        graph.add_edge(origin_node_index, dest_node_index, ());
    }
    let mut origin_airports_heap: BinaryHeap<&Airport> = BinaryHeap::new();
    for node_index in graph.node_indices() {
        if let Some(airport) = graph.node_weight(node_index) {
            if airport.flights > 0 {
                origin_airports_heap.push(airport);
            }
        }
    }
    println!("Airports ranked by the number of flights (origin):");
    while let Some(airport) = origin_airports_heap.pop() {
        println!(
            "Airport ID: {}, Name: {}, City: {}, Flights: {}",
            airport.id, airport.name, airport.city, airport.flights
        );
    }
    Ok(())
}