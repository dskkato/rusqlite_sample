extern crate rusqlite;
extern crate time;

use rusqlite::{params, Connection, Result};
use std::collections::HashMap;
use time::Timespec;

#[derive(Debug)]
struct Cat {
    name: String,
    date_of_birth: Timespec,
    color: String,
}

fn main() -> Result<()> {
    let conn = Connection::open("cats.db")?;
    let date_of_birth = time::get_time();

    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in &cat_colors {
        conn.execute(
            "INSERT INTO cat_colors (name) values (?1)",
            &[&color.to_string()],
        )?;
        let last_id = conn.last_insert_rowid();
        for cat in catnames {
            conn.execute(
                "INSERT INTO cats (name, date_of_birth, color_id) values (?1, ?2, ?3)",
                &[&cat.to_string(), &date_of_birth, &last_id],
            )?;
        }
    }
    let mut stmt = conn.prepare(
        "SELECT c.name, date_of_birth, cc.name from cats c 
                                 INNER JOIN cat_colors cc ON cc.id = c.color_id;",
    )?;
    let cats = stmt.query_map(&[], |row| Cat {
        name: row.get(0),
        date_of_birth: row.get(1),
        color: row.get(2),
    })?;

    for cat in cats {
        println!("Found cat {:?}", cat);
    }

    Ok(())
}
