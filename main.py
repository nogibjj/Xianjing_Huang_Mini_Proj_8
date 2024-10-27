"""
ETL-Query script
"""

from mylib.extract import extract
from mylib.transform_load import load
from mylib.query import create_CRUD, read_CRUD, update_CRUD, delete_CRUD
import time
import psutil

def track_resource_usage():
    process = psutil.Process()
    mem_info = process.memory_info()
    print(f"Memory Usage: {mem_info.rss / (1024 * 1024):.2f} MB")
    print(f"CPU Time: {process.cpu_times().user:.2f} seconds")

# Extract
print("Extracting data...")
start_time = time.time()
extract()
print("Extracting data successfully!")

# Transform and load
print("Transforming data...")
load()
print("Transforming data successfully!")

# CRUD
print("Read Database...")
results = read_CRUD()
for row in results:
    print(row)
print("Insert a record...")
create_CRUD("TestCountry1", "TestConfederation2", 0.1, 0.2, 0.3)
# print("Read Database after create...")
# results = read_CRUD()
# for row in results:
#     print(row)
print("Update a record...")
update_CRUD("TestCountry1", "TestConfederation2", 1.1, 1.2, 1.3, 192)
# print("Read Database after update...")
# results = read_CRUD()
# for row in results:
#     print(row)
print("Delete a record...")
delete_CRUD(192)
# print("Read Database after delete...")
# results = read_CRUD()
# for row in results:
#     print(row)
track_resource_usage()
print(f"Real Time: {time.time() - start_time:.2f} seconds")