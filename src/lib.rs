use std::io::Write;
use std::error::Error;
use csv::Reader;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
pub struct Airport { ///features ill be focusing on
    pub id: u32,
    pub name: String,
    pub city: String,
    pub flights: usize,
    pub passengers: usize,
    pub destinations: HashSet<u32>,
}

impl Airport {
    pub fn new(id: u32, name: String, city: String) -> Self {
        Airport {
            id,
            name,
            city,
            flights: 0,
            passengers: 0,
            destinations: HashSet::new(),
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

    let mut graph = DiGraph::<Airport, ()>::new(); ///graph with airports as the nodes
    let mut node_indices: HashMap<u32, NodeIndex> = HashMap::new();
    let mut total_flights_processed = 0;
    let mut route_counts: HashMap<(u32, u32), usize> = HashMap::new();
    let mut total_flights = 0;
    let mut total_passengers = 0;

    for result in rdr.records() {
        let record = result?;

        let origin_airport_id: u32 = record[2].parse()?;
        let origin = record[3].to_string();
        let origin_city_name = record[4].to_string();
        let dest_airport_id: u32 = record[5].parse()?;
        let dest = record[6].to_string();
        let dest_city_name = record[7].to_string();
        let route = (origin_airport_id, dest_airport_id);
        total_flights += 1;  
        let passengers: usize = record[0].chars()
            .filter(|c| c.is_digit(10)) 
            .collect::<String>() 
            .parse()
            .unwrap_or(0);
        total_passengers += passengers;
        *route_counts.entry(route).or_insert(0) += 1;

        let origin_node_index = *node_indices.entry(origin_airport_id).or_insert_with(|| {
            let airport = Airport::new(origin_airport_id, origin.clone(), origin_city_name.clone());
            graph.add_node(airport)
        });
        let dest_node_index = *node_indices.entry(dest_airport_id).or_insert_with(|| {
            let airport = Airport::new(dest_airport_id, dest.clone(), dest_city_name.clone());
            graph.add_node(airport)
        });

        graph.add_edge(origin_node_index, dest_node_index, ()); ///route to destination is edge


        if let Some(origin_airport) = graph.node_weight_mut(origin_node_index) {
            println!("Before updating, flights for {}: {}", origin_airport_id, origin_airport.flights);
            origin_airport.flights += 1;
            origin_airport.destinations.insert(dest_airport_id); 
            total_flights_processed += 1;  ///to ensure there are no errors
            println!("After updating, flights for {}: {}", origin_airport_id, origin_airport.flights);
        }
        if let Some(dest_airport) = graph.node_weight_mut(dest_node_index) {
            dest_airport.passengers += passengers;
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
        let avg_passengers_per_flight = if airport.flights > 0 {
            airport.passengers as f64 / airport.flights as f64
        } else {
            0.0
        };
        write!(
            writer,
            "Node: {}, Name: {}, City: {}, Degree: {}, Passenger: {}, Average Passengers per Flight: {:.2}, Destinations: {}\n",
            node, airport.name, airport.city, airport.flights, airport.passengers, avg_passengers_per_flight, airport.destinations.len()
        )?;
        node += 1;
    }
    for &(airport_name, airport_id) in &[("SFO", 14771), ("SJC", 14831), ("OAK", 13796)] {
        let airport_node_index = node_indices.get(&airport_id).unwrap_or_else(|| {
            panic!("Airport ID {} not found in the graph", airport_id)
        });
        if let Some(airport) = graph.node_weight(*airport_node_index) {
            let mut destination_city_counts: HashMap<&str, usize> = HashMap::new();

            for &dest_id in &airport.destinations {
                if let Some(dest_node_index) = node_indices.get(&dest_id) {
                    if let Some(dest_airport) = graph.node_weight(*dest_node_index) {
                        *destination_city_counts.entry(&dest_airport.city).or_insert(0) += 1;
                    }
                }
            }
            let mut top_destination_cities = destination_city_counts.into_iter().collect::<Vec<_>>();
            top_destination_cities.sort_by(|&(_, count1), &(_, count2)| count2.cmp(&count1));

            writeln!(writer, "Top destination cities for {} ({}):", airport_name, airport.city)?;
            for (i, (city, _)) in top_destination_cities.iter().take(3).enumerate() {
                writeln!(writer, "{}. {}", i + 1, city)?;
            }
        }
    }
    let mut top_routes = route_counts.into_iter().collect::<Vec<_>>();
    top_routes.sort_by(|(_, count1), (_, count2)| count2.cmp(&count1));
    writeln!(writer, "Top five routes")?;
    for (i, ((origin_id, dest_id), count)) in top_routes.iter().take(5).enumerate() {
        let origin_name = graph.node_weight(node_indices[origin_id]).unwrap().name.as_str();
        let dest_name = graph.node_weight(node_indices[dest_id]).unwrap().name.as_str();
        writeln!(writer, "{}. {} to {}: {} flights", i + 1, origin_name, dest_name, count)?;
    }
    let average_passengers_per_flight = if total_flights > 0 {
        total_passengers as f64 / total_flights as f64
    } else {
        0.0
    };
    writeln!(writer, "Total flights in the CSV file: {}", total_flights)?;
    writeln!(writer, "Average passengers per flight: {:.2}", average_passengers_per_flight)?;
    Ok(())
}