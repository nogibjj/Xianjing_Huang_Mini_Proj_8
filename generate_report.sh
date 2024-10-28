#!/bin/bash

# Set the output file
OUTPUT_FILE="performance_report.md"

# Add a header to the report
echo "# Performance Report" > $OUTPUT_FILE
echo "" >> $OUTPUT_FILE

# Run Python script and append output to the report
echo "## Python Script Output" >> $OUTPUT_FILE
python3 main.py >> $OUTPUT_FILE 2>&1
echo "" >> $OUTPUT_FILE

# Navigate to the Rust project directory
cd sqlite

# Run the Rust project and append output to the report
echo "## Rust CLI Output" >> ../$OUTPUT_FILE
cargo run --quiet >> ../$OUTPUT_FILE 2>&1
echo "" >> ../$OUTPUT_FILE

# Navigate back to the original directory
cd ..

echo "Performance report generated in $OUTPUT_FILE"
