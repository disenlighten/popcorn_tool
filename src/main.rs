
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

// Struct to hold sales data
#[derive(Serialize, Deserialize, Debug)]
struct SalesData {
    cumulative_sales: HashMap<String, u32>, 
    cumulative_shifts: u32, 
    sales_history: Vec<SalesEntry>, 
}

// Struct to represent a day's sales entry
#[derive(Serialize, Deserialize, Debug)]
struct SalesEntry {
    location: String,
    microwave: u32,
    smores: u32, 
    caramel: u32,
    cheddar: u32,
    popping: u32,
    kettle: u32, 
}

impl SalesData {
    fn new() -> SalesData {
        SalesData {
            cumulative_sales: HashMap::from([
                ("microwave".to_string(), 0),
                ("smores".to_string(), 0),
                ("caramel".to_string(), 0),
                ("cheddar".to_string(), 0),
                ("popping".to_string(), 0),
                ("kettle".to_string(), 0),
            ]),
            cumulative_shifts: 0,
            sales_history: Vec::new(),
        }
    }
// Update sales data with new sales for a given day
fn update_sales(&mut self, entry: SalesEntry, num_shifts: u32) {
    self.cumulative_shifts += num_shifts;

    *self.cumulative_sales.get_mut("microwave").unwrap() += entry.microwave;
    *self.cumulative_sales.get_mut("smores").unwrap() += entry.smores;
    *self.cumulative_sales.get_mut("caramel").unwrap() += entry.caramel;
    *self.cumulative_sales.get_mut("cheddar").unwrap() += entry.cheddar;
    *self.cumulative_sales.get_mut("popping").unwrap() += entry.popping;
    *self.cumulative_sales.get_mut("kettle").unwrap() += entry.kettle;

    self.sales_history.push(entry);
}

fn rolling_averages(&self) {
    println!("\nRolling Hourly Averages:");
    for (popcorn_type, &sales) in &self.cumulative_sales {
        let avg = sales as f64 / (self.cumulative_shifts * 2) as f64;
        println!("{}: {:.2} per hour", popcorn_type, avg);
        }
    }
}

// Save sales data to a file
fn save_data(data: &SalesData) -> io::Result<()> {
    let serialized_data = serde_json::to_string(data)?;
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open("sales_data.json")?;
    file.write_all(serialized_data.as_bytes())?;
    println!("Data saved successfully!");
    Ok(())
}

// load sales data from a file
fn load_data() -> SalesData {
    if Path::new("sales_data.json").exists() {
        let contents = fs::read_to_string("sales_data.json").expect("Unable to read file");
        serde_json::from_str(&contents).expect("Error parsing JSON")
    } else {
        println!("No existing data found, starting fresh.");
        SalesData::new()
    }
}

// Function to get user input for the sales data
fn get_sales_input() -> (SalesEntry, u32) {
    let mut location = String::new();
    println!("Enter location name: ");
    io::stdin().read_line(&mut location).expect("Failed to read location");
    let location = location.trim().to_string();

    let microwave = get_sales("Unbelievable Butter: ");
    let smores = get_sales("Smores: ");
    let caramel = get_sales("Salted Caramel: ");
    let cheddar = get_sales("White Cheddar: ");
    let popping = get_sales("Popping Corn: ");
    let kettle = get_sales("Kettle Corn: ");

    let num_shifts = get_sales("Number of 2 hour shifts: ");

    let entry = SalesEntry {
        location, 
        microwave,
        smores,
        caramel,
        cheddar,
        popping,
        kettle,
    };

    (entry, num_shifts)
}

// Helper funciton to read an integer input for sales
fn get_sales(prompt: &str) -> u32 {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}

fn main() {
    let mut sales_data = load_data();

    loop {
        let (entry, num_shifts) = get_sales_input();
        sales_data.update_sales(entry, num_shifts);
        sales_data.rolling_averages();

        save_data(&sales_data).expect("Failed to save data");

        let mut cont = String::new();
        println!("Do you want to enter more data (y/n: ");
        io::stdin().read_line(&mut cont).expect(" Failed to read input");
        if cont.trim().to_lowercase() != "y" {
            break;
        }
    }
}