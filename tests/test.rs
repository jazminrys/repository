use std::io::Write;
use tempfile::NamedTempFile;
use project::run;

#[test]
fn test_run_function() {
    let mut file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        file,
        "PASSENGERS,UNIQUE_CARRIER_NAME,ORIGIN_AIRPORT_ID,ORIGIN,ORIGIN_CITY_NAME,DEST_AIRPORT_ID,DEST,DEST_CITY_NAME\n\
        114,Southwest Airlines Co.,14831,SJC,\"San Jose, CA\",12982,LIH,\"Lihue, HI\"\n\
        1990,Frontier Airlines Inc.,14771,SFO,\"San Francisco, CA\",11298,DFW,\"Dallas/Fort Worth, TX\"\n\
        3414,United Air Lines Inc.,13930,ORD,\"Chicago, IL\",14908,SNA,\"Santa Ana, CA\"\n\
        1532,Southwest Airlines Co.,13796,OAK,\"Oakland, CA\",12191,HOU,\"Houston, TX\"\n\
        1761,United Air Lines Inc.,14771,SFO,\"San Francisco, CA\",11292,DEN,\"Denver, CO\"\n"
    )
    .expect("Failed to write to temporary file");
    file.as_file_mut()
        .sync_all()
        .expect("Failed to flush changes"); 
    let file_path = file.into_temp_path();
    std::env::set_var(
        "CSV_FILE_PATH",
        file_path
            .to_str()
            .expect("Failed to convert file path to string"),
    );

    let mut output = Vec::new();
    run(&mut output).expect("Failed to run function");

    let output_str = String::from_utf8(output).expect("Failed to convert output to string");
    println!("Output: {}", output_str);

    let output_lines: Vec<&str> = output_str.lines().map(|line| line.trim()).collect();
    let expected_lines: Vec<&str> = "\
         Airports (nodes) ranked by the number of originating flights (degree):\n\
         Node: 1, Name: SJC, City: San Jose, CA, Degree: 1, Average Passengers per Flight: 114.00, Destinations: 1\n\
         Node: 2, Name: ORD, City: Chicago, IL, Degree: 1, Average Passengers per Flight: 3414.00, Destinations: 1\n\
         Node: 3, Name: OAK, City: Oakland, CA, Degree: 1, Average Passengers per Flight: 1532.00, Destinations: 1\n\
         Node: 4, Name: SFO, City: San Francisco, CA, Degree: 2, Average Passengers per Flight: 1875500, Destinations: 2\n\
         Top destination cities for SFO (San Francisco, CA):\n\
         1. Dallas/Fort Worth, TX\n\
         2. Denver, CO\n\
         Top destination cities for SJC (San Jose, CA):\n\
         1. Lihue, HI\n\
         Top destination cities for OAK (Oakland, CA):\n\
         1. Houston, TX
         Top five routes\n\
         1. SFO to DEN: 1 flights\n\
         2. OAK to HOU: 1 flights\n\
         3. SJC to LIH: 1 flights\n\
         4. ORD to SNA: 1 flights\n\
         5. SFO to DFW: 1 flights\n\
         Total flights in the CSV file: 5\n\
         Average passengers per flight: 1762.20"
    .lines()
    .map(|line| line.trim())
    .collect();

    assert_eq!(output_lines, expected_lines);
}