#!/bin/bash

# Check if the correct number of arguments is provided
if [ $# -ne 1 ]; then
    echo "Usage: $0 <output_file>"
    exit 1
fi

output_file=$1

# Define the number of times you want to run the binary
num_runs=100

# Create a CSV file and add a header
echo "buffer_allocation,file_reading,bytes,lines,words" > $output_file

# Loop to run the binary and collect output
for ((i=1; i<=$num_runs; i++))
do
    output=$(./target/release/rust Cargo.toml)  # Assuming your binary is in the same directory

    # Extract the required information
    buffer_allocation=$(echo "$output" | awk '/Buffer allocation took:/ {print $4}')
    file_reading=$(echo "$output" | awk '/Reading the file took:/ {print $5}')
    bytes=$(echo "$output" | awk '/Counting the bytes took:/ {print $5}')
    lines=$(echo "$output" | awk '/Counting the lines took:/ {print $5}')
    words=$(echo "$output" | awk '/Counting the words took:/ {print $5}')

    # Append the information to the CSV file
    echo "$buffer_allocation,$file_reading,$bytes,$lines,$words" >> $output_file
done
