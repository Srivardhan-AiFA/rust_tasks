use crate::record::Record;

pub struct RecordRegistry<'a> {
    records: Vec<&'a Record>,
}

impl<'a> RecordRegistry<'a> {
    pub fn new() -> Self {
        let record_store: Vec<&'a Record> = Vec::new();

        Self {
            records: record_store,
        }
    }

    pub fn add(&mut self, record: &'a Record) {
        self.records.push(record);
    }

    pub fn print_register(&self) {
        for (idx, record) in self.records.iter().enumerate() {
            println!(
                "Record No: {}, And the record id is: {}, record name is: {}",
                idx + 1,
                record.id,
                record.name
            )
        }
    }

    pub fn get_by_id(&self, id: u32) -> Option<&'a Record> {
        match self.records.iter().find(|x| x.id == id) {
            Some(s) => Some(s),
            _ => None,
        }
    }

    pub fn longest_name(&self) -> Option<&str> {
        let mut longest_name = &self.records.get(0)?.name;
        for (_, record_name) in self.records.iter().enumerate() {
            if record_name.name.len() > longest_name.len() {
                longest_name = &record_name.name
            }
        }
        Some(longest_name)
    }
}
