"""Query the database"""

import sqlite3

LOG_FILE = "query_log.md"


def log(query):
    with open(LOG_FILE, "a") as file:
        file.write(f"```sql\n{query}\n```\n\n")


def create_CRUD(country,
            confederation,
            population_share,
            tv_audience_share,
            gdp_weighted_share):
    conn = sqlite3.connect("FifaDB.db")
    cursor = conn.cursor()

    cursor.execute(
        """
        INSERT INTO FifaDB(
            country,
            confederation,
            population_share,
            tv_audience_share,
            gdp_weighted_share
            ) 
            VALUES (?, ?, ?, ?, ?)
        """,
        (country,
        confederation,
        population_share,
        tv_audience_share,
        gdp_weighted_share),
    )
    conn.commit()
    conn.close()

    log(
        f"""INSERT INTO FifaDB VALUES (
            {country}, 
            {confederation}, 
            {population_share}, 
            {tv_audience_share}, 
            {gdp_weighted_share});"""
    )

def read_CRUD():
    conn = sqlite3.connect("FifaDB.db")
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM FifaDB")
    results = cursor.fetchall()
    conn.close()
    log("SELECT * FROM FifaDB;")
    return results

def update_CRUD(country,
            confederation,
            population_share,
            tv_audience_share,
            gdp_weighted_share,
            updateID):
    conn = sqlite3.connect("FifaDB.db")
    c = conn.cursor()
    c.execute(
        """
        UPDATE FifaDB
        SET country=?, 
        confederation=?, 
        population_share=?,
        tv_audience_share=?, 
        gdp_weighted_share=? 
        WHERE id=?
        """,
        (
            country,
            confederation,
            population_share,
            tv_audience_share,
            gdp_weighted_share,
            updateID,
        ),
    )
    conn.commit()
    conn.close()

    log(
        f"""UPDATE FifaDB SET 
        country={country}, 
        confederation={confederation},
        population_share={population_share}, 
        tv_audience_share={tv_audience_share}, 
        gdp_weighted_share={gdp_weighted_share} 
        WHERE id={updateID};"""
    )

def delete_CRUD(deleteID):
    """delete query"""
    conn = sqlite3.connect("FifaDB.db")
    c = conn.cursor()
    c.execute("DELETE FROM FifaDB WHERE id=?", (deleteID,))
    conn.commit()
    conn.close()

    log(f"DELETE FROM FifaDB WHERE id={deleteID};")





