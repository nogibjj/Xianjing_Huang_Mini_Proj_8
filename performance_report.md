# Performance Report

## Python Script Output
Extracting data...
Extracting data successfully!
Transforming data...
Transforming data successfully!
Read Database...
(1, 'United States', 'CONCACAF', 4.5, 4.3, 11.3)
(2, 'Japan', 'AFC', 1.9, 4.9, 9.1)
(3, 'China', 'AFC', 19.5, 14.8, 7.3)
(4, 'Germany', 'UEFA', 1.2, 2.9, 6.3)
(5, 'Brazil', 'CONMEBOL', 2.8, 7.1, 5.4)
(6, 'United Kingdom', 'UEFA', 0.9, 2.1, 4.2)
(7, 'Italy', 'UEFA', 0.9, 2.1, 4.0)
(8, 'France', 'UEFA', 0.9, 2.0, 4.0)
(9, 'Russia', 'UEFA', 2.1, 3.1, 3.5)
(10, 'Spain', 'UEFA', 0.7, 1.8, 3.1)
...
and 181 more rows not shown
Insert a record: 'TestCountry1', 'TestConfederation2', 0.1, 0.2, 0.3 
Update a record: 'TestCountry1', 'TestConfederation2', 1.1, 1.2, 1.3, 192
Delete a record: id: 192
Memory Usage: 30.64 MB
CPU Time: 0.08 seconds
Real Time: 0.12 seconds

## Rust CLI Output
Running command: Extract
Extract a url to a file path
File has been downloaded to data/fifa_countries_audience.csv
Command execution time: 0.17 seconds
Command memory usage: 16.82 MB
Running command: Create { table_name: "test_table2" }
Creating Table test_table2
Table 'test_table2' created successfully.
Command execution time: 0.00 seconds
Command memory usage: 16.96 MB
Running command: Load { table_name: "test_table2", file_path: "data/fifa_countries_audience.csv" }
Loading data into table 'test_table2' from 'data/fifa_countries_audience.csv'
Data loaded successfully from 'data/fifa_countries_audience.csv' into table 'test_table2'.
Command execution time: 0.05 seconds
Command memory usage: 17.07 MB
Running command: Query { table_name: "test_table2" }
Read Table: test_table2
ID: 1, Country: United States, Confederation: CONCACAF, Population Share: 4.5, TV Audience Share: 4.3, GDP Weighted Share: 11.3
ID: 2, Country: Japan, Confederation: AFC, Population Share: 1.9, TV Audience Share: 4.9, GDP Weighted Share: 9.1
ID: 3, Country: China, Confederation: AFC, Population Share: 19.5, TV Audience Share: 14.8, GDP Weighted Share: 7.3
ID: 4, Country: Germany, Confederation: UEFA, Population Share: 1.2, TV Audience Share: 2.9, GDP Weighted Share: 6.3
ID: 5, Country: Brazil, Confederation: CONMEBOL, Population Share: 2.8, TV Audience Share: 7.1, GDP Weighted Share: 5.4
ID: 6, Country: United Kingdom, Confederation: UEFA, Population Share: 0.9, TV Audience Share: 2.1, GDP Weighted Share: 4.2
ID: 7, Country: Italy, Confederation: UEFA, Population Share: 0.9, TV Audience Share: 2.1, GDP Weighted Share: 4
ID: 8, Country: France, Confederation: UEFA, Population Share: 0.9, TV Audience Share: 2, GDP Weighted Share: 4
ID: 9, Country: Russia, Confederation: UEFA, Population Share: 2.1, TV Audience Share: 3.1, GDP Weighted Share: 3.5
ID: 10, Country: Spain, Confederation: UEFA, Population Share: 0.7, TV Audience Share: 1.8, GDP Weighted Share: 3.1
...
and 181 more rows not shown
Command execution time: 0.00 seconds
Command memory usage: 17.09 MB
Running command: Insert { table_name: "test_table2", country: "TestCountry", confederation: "TestConfederation", population_share: 0.1, tv_audience_share: 0.2, gdp_weighted_share: 0.3 }
Insert record in table 'test_table2' with country TestCountry, confederation TestConfederation, population_share 0.1, tv_audience_share 0.2, gdp_weighted_share 0.3
Inserted record into the 'test_table2' table successfully!
Command execution time: 0.00 seconds
Command memory usage: 17.23 MB
Running command: Update { table_name: "test_table2", id: 192, new_country: Some("TestCountry"), new_confederation: Some("TestConfederation"), new_population_share: Some(1.1), new_tv_audience_share: Some(2.2), new_gdp_weighted_share: Some(3.3) }
Updating record in table 'test_table2' with ID 192
Record with ID '192' updated successfully in table 'test_table2'.
Command execution time: 0.00 seconds
Command memory usage: 17.30 MB
Running command: Delete { table_name: "test_table2", id: 192 }
Delete record in table 'test_table2' with ID 192
Deleted person with ID '192' from the 'test_table2' table successfully!
Command execution time: 0.00 seconds
Command memory usage: 17.33 MB
Running command: Drop { table_name: "test_table2" }
Deleting: test_table2
Table 'test_table2' dropped successfully.
Command execution time: 0.00 seconds
Command memory usage: 17.33 MB

Total Execution Time for all commands: 0.28 seconds
Peak Memory Usage across all commands: 17.33 MB

