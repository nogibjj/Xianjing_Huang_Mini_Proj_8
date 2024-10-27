"""
Transforms and Loads data into the local SQLite3 database

"""
import sqlite3
import csv

#load the csv file and insert into a new sqlite3 database
def load(dataset="data/fifa_countries_audience.csv"):
    """"Transforms and Loads data into the local SQLite3 database"""
    #prints the full working directory and path
    #print(os.getcwd())
    payload = csv.reader(open(dataset, newline=''), delimiter=',')
    # skips the header of csv
    next(payload)
    conn = sqlite3.connect('FifaDB.db')
    c = conn.cursor()
    c.execute("DROP TABLE IF EXISTS FifaDB")
    c.execute(
        """
        CREATE TABLE FifaDB (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            country TEXT,
            confederation TEXT,
            population_share REAL,
            tv_audience_share REAL,
            gdp_weighted_share REAL
        )
    """
    )

    #insert
    c.executemany(
        """
        INSERT INTO FifaDB (
            country,
            confederation,
            population_share,
            tv_audience_share,
            gdp_weighted_share
            ) 
            VALUES (?, ?, ?, ?, ?)""",
        payload,
    )
    # c.executemany("INSERT INTO PlayTennisDB VALUES (?,?, ?, ?, ?, ?)", payload)
    conn.commit()
    conn.close()
    return "FifaDB.db"

