mod record;
mod registry;

use crate::record::Record;
use crate::registry::RecordRegistry;

fn main() {
    let r1 = Record {
        id: 101,
        name: "holly".to_string(),
    };
    let r2 = Record {
        id: 102,
        name: "molly".to_string(),
    };
    let r3 = Record {
        id: 103,
        name: "solly".to_string(),
    };
    let r4 = Record {
        id: 104,
        name: "kolly".to_string(),
    };

    let mut registry_book = RecordRegistry::new();
    registry_book.add(&r1);
    registry_book.add(&r2);
    registry_book.add(&r3);
    registry_book.add(&r4);

    registry_book.print_register();

    let result_to_find_record = match registry_book.get_by_id(103) {
        Some(record) => format!(
            "Record id is: {}, record name is: {}",
            record.id, record.name,
        ),
        None => format!("Record with these ID is not exist"),
    };
    println!("{}", result_to_find_record);

    let longest_str_in_records = match registry_book.longest_name() {
        Some(record_name) => format!("Longest record name is: {}", record_name),
        None => format!("Record with these ID is not exist"),
    };
    println!("{}", longest_str_in_records);
}
