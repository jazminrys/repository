extern crate csv;
extern crate petgraph;
use std::io::Write;
use std::error::Error;
use csv::Reader;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
#[derive(Debug, Eq, PartialEq)]
pub struct Airport {
    pub id: u32,
    pub name: String,
    pub city: String,
    pub flights: usize,
}
impl Airport {
    pub fn new(id: u32, name: String, city: String) -> Self {
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
pub fn run<W: Write>(writer: &mut W) -> Result<(), Box<dyn Error>> {
    let file_path = std::env::var("CSV_FILE_PATH").unwrap_or_else(|_| "airport.csv".to_string());
    let mut rdr = Reader::from_path(file_path)?;
    let mut graph = DiGraph::<Airport, ()>::new();
    let mut node_indices: HashMap<u32, NodeIndex> = HashMap::new();
    let mut total_flights_processed = 0;  
    for result in rdr.records() {
        let record = result?;
        println!("Processing record: {:?}", record);
        let origin_airport_id: u32 = record[2].parse()?;
        let origin = record[3].to_string();
        let origin_city_name = record[4].to_string();
        let dest_airport_id: u32 = record[5].parse()?;
        let dest = record[6].to_string();
        let dest_city_name = record[7].to_string();
        println!("Origin Airport ID: {}, Name: {}, City: {}", origin_airport_id, origin, origin_city_name);
        println!("Destination Airport ID: {}, Name: {}, City: {}", dest_airport_id, dest, dest_city_name);
        let origin_node_index = *node_indices.entry(origin_airport_id).or_insert_with(|| {
            let airport = Airport::new(origin_airport_id, origin.clone(), origin_city_name.clone());
            println!("Adding origin airport to graph: {:?}", airport);
            graph.add_node(airport)
        });
        let dest_node_index = *node_indices.entry(dest_airport_id).or_insert_with(|| {
            let airport = Airport::new(dest_airport_id, dest.clone(), dest_city_name.clone());
            println!("Adding destination airport to graph: {:?}", airport);
            graph.add_node(airport)
        });
        println!("Adding edge from {} to {}", origin, dest);
        graph.add_edge(origin_node_index, dest_node_index, ());
        if let Some(origin_airport) = graph.node_weight_mut(origin_node_index) {
            println!("Before updating, flights for {}: {}", origin_airport_id, origin_airport.flights);
            origin_airport.flights += 1;
            total_flights_processed += 1;  
            println!("After updating, flights for {}: {}", origin_airport_id, origin_airport.flights);
        }
    }
    println!("Total flights processed: {}", total_flights_processed);  

    let mut origin_airports_heap: BinaryHeap<&Airport> = BinaryHeap::new();
    for node_index in graph.node_indices() {
        if let Some(airport) = graph.node_weight(node_index) {
            if airport.flights > 0 {
                origin_airports_heap.push(airport);
            }
        }
    }
    write!(writer, "Airports (nodes) ranked by the number of originating flights (degree):\n")?;
    let mut node = 1;
    while let Some(airport) = origin_airports_heap.pop() {
        write!(
            writer,
            "Node: {}, Name: {}, City: {}, Degree: {}\n",
            node, airport.name, airport.city, airport.flights
        )?;
        node += 1;
    }

    Ok(())
}