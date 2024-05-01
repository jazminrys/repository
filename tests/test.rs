use std::io::Write;
use tempfile::NamedTempFile;
use project::run; 

#[test]
fn test_run_function() {
    let mut file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(
        file,
        "PASSENGERS,UNIQUE_CARRIER_NAME,ORIGIN_AIRPORT_ID,ORIGIN,ORIGIN_CITY_NAME,DEST_AIRPORT_ID,DEST,DEST_CITY_NAME\n\
        3414,United Air Lines Inc.,13930,ORD,\"Chicago, IL\",14908,SNA,\"Santa Ana, CA\"\n\
        3415,Envoy Air,13930,ORD,\"Chicago, IL\",10721,BOS,\"Boston, MA\"\n\
        3415,JetBlue Airways,12266,IAH,\"Houston, TX\",10721,BOS,\"Boston, MA\"\n\
        3416,American Airlines Inc.,14107,PHX,\"Phoenix, AZ\",13204,MCO,\"Orlando, FL\"\n\
        3416,Spirit Air Lines,12889,LAS,\"Las Vegas, NV\",14869,SLC,\"Salt Lake City, UT\""
    ).expect("Failed to write to temporary file");
    file.as_file_mut().sync_all().expect("Failed to flush changes to disk"); // Flushes changes to disk
    let file_path = file.into_temp_path();  
    std::env::set_var("CSV_FILE_PATH", file_path.to_str().expect("Failed to convert file path to string"));

    let mut output = Vec::new();
    run(&mut output).expect("Failed to run function");

    let output_str = String::from_utf8(output).expect("Failed to convert output to string");
    println!("Output: {}", output_str); 
    
    let mut output_lines: Vec<&str> = output_str.lines().map(|line| line.trim()).collect();
    let mut expected_lines: Vec<&str> = "\
        Airports ranked by the number of flights (origin):\n\
        Airport ID: 13930, Name: ORD, City: Chicago, IL, Flights: 2\n\
        Airport ID: 12266, Name: IAH, City: Houston, TX, Flights: 1\n\
        Airport ID: 14107, Name: PHX, City: Phoenix, AZ, Flights: 1\n\
        Airport ID: 12889, Name: LAS, City: Las Vegas, NV, Flights: 1\n"
        .lines()
        .map(|line| line.trim())
        .collect();
    
    output_lines.sort();
    expected_lines.sort();

    assert_eq!(output_lines, expected_lines);
}