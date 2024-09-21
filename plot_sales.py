#!/usr/bin/env python3

import json
import pandas as pd
import matplotlib.pyplot as plt

# Load the sales data from JSON file
def load_sales_data(file_name):
    with open(file_name, "r") as f:
        data = json.load(f)
    return data

# Function to plot sales per product by location
def plot_sales_per_product(data):
    # Convert sales history to a pandas DataFrame for easier plotting
    df = pd.DataFrame(data["sales_history"])
    
    # Create a bar chart for each type of popcorn
    df.set_index("location")[["microwave", "smores", "caramel", "cheddar", "popping", "kettle"]].plot(
        kind="bar", stacked=False, figsize=(10, 6))
    
    # Adding labels and title
    plt.title("Popcorn Sales by Product per Location")
    plt.xlabel("Location")
    plt.ylabel("Number of Sales")
    plt.xticks(rotation=45, ha="right")
    plt.tight_layout()
    plt.show()

# Main function
def main():
    # Load sales data from your Rust-generated JSON file
    sales_data = load_sales_data("./target/debug/sales_data.json")
    
    # Plot the sales data
    plot_sales_per_product(sales_data)

if __name__ == "__main__":
    main()