
use std::collections::HashMap;
use crate::jvm_error::JVMError;

/// The type of a heap allocated thing
pub enum ObjectValue {
    Instance(ClassInstance),
    Array(ArrayInstance),
}

pub enum ArrayType {
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Boolean,
    Ref(String),
}

pub enum ArrayData {
    Byte(Vec<u8>),
    Chars(Vec<u16>),
    Short(Vec<i16>),
    Int(Vec<i32>),
    Long(Vec<i64>),
    Float(Vec<f32>),
    Double(Vec<f64>),
    Boolean(Vec<bool>),
    Ref(Vec<u32>), // refIDs
}

#[derive(Debug, Clone)]
pub enum FieldValue {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Ref(u32), // refID
}

pub struct JVMHeap {
    /// Live objects indexed by refID
    pub objects: HashMap<u32, ObjectValue>,
    /// Next refID to assign
    pub next_ref_id: u32,
    /// The string table
    pub string_table: HashMap<String, u32>,
    /// max heap size in bytes
    pub max_heap_size: usize,
    /// current heap size in bytes
    pub current_heap_size: usize,
}

pub struct ClassInstance {
    /// The name of this class
    pub class_name: String,
    /// The fields of this instance, indexed by field name
    pub fields: HashMap<String, FieldValue>,
    /// Marked to delete
    pub gc_mark: bool,
}

pub struct ArrayInstance {
    /// The type of the array elements
    pub array_type: ArrayType,
    /// The array data
    pub elements: ArrayData,
    /// Marked to delete
    pub gc_mark: bool,
}

impl JVMHeap {
    pub fn new(max_heap_size: usize) -> Self {
        Self {
            objects: HashMap::new(),
            next_ref_id: 1, // start at 1 to reserve 0 for null
            string_table: HashMap::new(),
            max_heap_size,
            current_heap_size: 0,
        }
    }

    /// Allocates a new object on the heap and returns its refID
    pub fn allocate_object(
        &mut self,
        class_name: &str,
        default_fields: HashMap<String, FieldValue>,
    ) -> Result<u32, JVMError> {
        self.gc_sweep()?;

        let ref_id = self.next_ref_id;
        self.next_ref_id += 1;

        let instance = ClassInstance {
            class_name: class_name.to_string(),
            fields: default_fields,
            gc_mark: false,
        };

        self.objects.insert(ref_id, ObjectValue::Instance(instance));
        return Ok(ref_id);
    }

    pub fn allocate_array(
        &mut self,
        array_type: ArrayType,
        length: usize,
    ) -> Result<u32, JVMError> {
        if length < 0 {
            return Err(JVMError::NegativeArraySizeException.into());
        }

        self.gc_sweep()?;

        let ref_id = self.next_ref_id;
        self.next_ref_id += 1;

        let elements = match array_type {
            ArrayType::Byte => ArrayData::Byte(vec![0; length]),
            ArrayType::Char => ArrayData::Chars(vec![0; length]),
            ArrayType::Short => ArrayData::Short(vec![0; length]),
            ArrayType::Int => ArrayData::Int(vec![0; length]),
            ArrayType::Long => ArrayData::Long(vec![0; length]),
            ArrayType::Float => ArrayData::Float(vec![0.0; length]),
            ArrayType::Double => ArrayData::Double(vec![0.0; length]),
            ArrayType::Boolean => ArrayData::Boolean(vec![false; length]),
            ArrayType::Ref(_) => ArrayData::Ref(vec![0; length]), // initialize ref array with nulls
        };

        let instance = ArrayInstance {
            array_type,
            elements,
            gc_mark: false,
        };

        self.objects.insert(ref_id, ObjectValue::Array(instance));
        return Ok(ref_id);
    }

    pub fn get_object(&self, ref_id: u32) -> Result<&ObjectValue, JVMError> {
        if ref_id == 0 {
            return Err(JVMError::NullPointerException.into());
        }
        self.objects.get(&ref_id).ok_or(JVMError::InvalidReference.into())
    }

    pub fn get_object_mut(&mut self, ref_id: u32) -> Result<&mut ObjectValue, JVMError> {
        if ref_id == 0 {
            return Err(JVMError::NullPointerException.into());
        }
        self.objects.get_mut(&ref_id).ok_or(JVMError::InvalidReference.into())
    }

    pub fn get_field(
        &self,
        ref_id: u32,
        field_name: &str,
    ) -> Result<FieldValue, JVMError> {
        match self.get_object(ref_id)? {
            ObjectValue::Instance(instance) => {
                instance.fields.get(field_name)
                    .cloned()
                    .ok_or(JVMError::InvalidReference.into())
            }
            _ => Err(JVMError::NotAnInstance.into()),
        }
    }

    pub fn set_field(
        &mut self,
        ref_id: u32,
        field_name: &str,
        value: FieldValue,
    ) -> Result<(), JVMError> {
        match self.get_object_mut(ref_id)? {
            ObjectValue::Instance(instance) => {
                instance.fields.insert(field_name.to_string(), value);
                Ok(())
            }
            _ => Err(JVMError::NotAnInstance.into()),
        }
    }

    pub fn intern_string(&mut self, value: &str) -> Result<u32, JVMError> {
        if let Some(&ref_id) = self.string_table.get(value) {
            return Ok(ref_id);
        }

        // allocate the string
        let chars: Vec<u16> = value.encode_utf16().collect();
        let char_array_id = self.allocate_array(
            ArrayType::Char, 
            chars.len()
        )?;

        if let ObjectValue::Array(arr) = self.get_object_mut(char_array_id)? {
            arr.elements = ArrayData::Chars(chars);
        }

        // allocate a java/lang/String object
        let mut fields = HashMap::new();
        fields.insert("java/lang/String.value".to_string(), FieldValue::Ref(char_array_id));
        fields.insert("java/lang/String.hash".to_string(), FieldValue::Int(0)); // lazy compute hash

        let string_id = self.allocate_object(
            "java/lang/String",
            fields,
        )?;
        self.string_table.insert(value.to_string(), string_id);

        Ok(string_id)
    }

    pub fn gc_sweep(&mut self) -> Result<(), JVMError> {
        if self.current_heap_size <= self.max_heap_size {
            return Ok(());
        }

        // Mark and sweep GC

        // Temporary error throw
        return Err(JVMError::OutOfMemoryError.into());
    }
}
