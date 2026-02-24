use std::{sync::Mutex};

static REFERENCE_TABLE: Mutex<ReferenceTable> = Mutex::new(ReferenceTable { table: Vec::new() });

pub static NULL_REF: Reference = Reference { ref_index: 0 };


pub struct ReferenceTable {
    table: Vec<u32>,
}

impl ReferenceTable {
    pub fn add_reference(&mut self, pointer: u32) -> Reference {
        self.table.push(pointer);
        Reference {
            ref_index: (self.table.len() - 1) as u32,
        }
    }
}

pub struct Reference {
    ref_index: u32,
}

impl Reference {
    pub fn new(ref_index: u32) -> Self {
        Reference { ref_index }
    }

    pub fn is_null(&self) -> bool {
        self.ref_index == 0
    }

    pub fn get_ref_index(&self) -> u32 {
        self.ref_index
    }
}