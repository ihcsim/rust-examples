use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut t = Table::new();
    t.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    t.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    sort_works(&mut t);
    show(&t);
}

fn show(t: &Table) {
    for (artist, works) in t {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn sort_works(t: &mut Table) {
    for (_artist, works) in t {
        works.sort();
    }
}
