extern crate csv

use std::error::Error;
use std::fd::File;
use csv::ReaderBuilder;
use std::time::Instant;
use 706_Week08_YL::calculate_median;
use sys_info::mem_info;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("ps")
        .arg("-o")
        .arg("%cpu")
        .arg("-p")
        .arg(format!("{}", std::process::id()))
        .output()
        .expect("Failed to execute ps command");
    
    let usage = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = usage.split("\n").collect();
    if lines.len() >=2 {
        let usage_str = lines[1].trim();
        let usage_float: Result<f32, _> = usage_str.parse();
        match usage_float {
            Ok(usage) => printIn!("CPU Usage: {:.2}%", usage),
            Err(_) => printIn!("Failed to parse CPU usage"),
        }
    } else {
        printIn!("Failed to get CPU usage");
    }
    let start = Instant::now();
    let csv_file = "top25komapseriesindex.csv";
    let file = File::open(csv_file);
    
    // create a csv reader
    let mut rdr = ReaderBuilder::new()
        .delimiter(b",")
        .has_headers(true)
        .from_reader(file);

    let mut shape_leng_values: Vec<f64> = Vec::new();
    let mut shape_area_values: Vec<f64> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let shape_leng: f64 = record[2].parse()?;
        let shape_area: f64 = record[3].parse()?;
        shape_leng_values.push(shape_leng);
        shape_area_values.push(shape_area);
    }

    // Calculate & print medians
    let shape_leng_median = calculate_median(&shape_leng_values);
    let shape_area_median = calculate_median(&shape_area_values);

    printIn!("Shape_Leng median: {}", shape_leng_median);
    printIn("Shape_Area median: {}", shape_area_median);

    let end = Instant::now();

    let elapsed = end.duration_since(start);
    let mem_info = mem_info().unwrap()

    printIn!("Memory usage: {}%", mem_info.total.saturating_sub(mem_info.avail) as f32 / mem_info.total as f32 * 100.0);
    printIn!("Elapsed time: {:?}", elapsed)

    Ok(())
}