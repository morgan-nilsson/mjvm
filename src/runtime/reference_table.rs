use std::sync::Mutex;
use crate::runtime::array::AnyArrayValue;
use crate::runtime::object::ObjectValue;
use crate::runtime::interface::InterfaceValue;

static REFERENCE_TABLE: Mutex<ReferenceTable> = Mutex::new(ReferenceTable { table: Vec::new() });

pub static NULL_REF: Reference = Reference { ref_index: 0 };


pub struct ReferenceTable {
    table: Vec<Box<ReferenceValue>>,
}

impl ReferenceTable {
    // Adds references to a table
    pub fn add_reference(&mut self, value: ReferenceValue) -> Reference {
        self.table.push(Box::new(value));
        Reference {
            ref_index: (self.table.len() - 1) as u32,
        }
    }

    pub fn get_reference(&self, reference: &Reference) -> Option<&Box<ReferenceValue>> {
        self.table.get(reference.ref_index as usize)
    }
}

pub struct Reference {
    pub ref_index: u32,
}

impl Reference {
    pub fn new(value: u32) -> Self {
        Self { ref_index: value }
    }

    pub fn is_null(&self) -> bool {
        self.ref_index == NULL_REF.ref_index
    }

    pub fn is_not_null(&self) -> bool {
        !self.is_null()
    }

    pub fn get_ref_index(&self) -> u32 {
        self.ref_index
    }
}

pub enum ReferenceValue {
    Object(ObjectValue),
    Array(AnyArrayValue),
    Interface(InterfaceValue),
}