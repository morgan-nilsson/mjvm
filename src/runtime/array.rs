use crate::runtime::reference_table::Reference;
use crate::runtime::reference_table::ReferenceValue;

pub struct ArrayValue<T: ArrayElement> {
    data: Vec<T>,
}

impl<T: ArrayElement + Clone + Default> ArrayValue<T> {
    pub fn new(length: u16) -> Self {
        Self { data: vec![T::default(); length as usize] }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn set(&mut self, index: usize, value: T) -> Result<(), String> {
        if index < self.data.len() {
            Ok(self.data[index] = value)
        } else {
            Err(format!("Index out of bounds: {}", index))
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

pub enum AnyArrayValue {
    Bool(ArrayValue<bool>),
    I8(ArrayValue<i8>),
    Char(ArrayValue<char>),
    I16(ArrayValue<i16>),
    I32(ArrayValue<i32>),
    I64(ArrayValue<i64>),
    F32(ArrayValue<f32>),
    F64(ArrayValue<f64>),
    REF(ArrayValue<Reference>),
}

impl AnyArrayValue {
    pub fn len(&self) -> usize {
        match self {
            AnyArrayValue::Bool(a) => a.len(),
            AnyArrayValue::I8(a)   => a.len(),
            AnyArrayValue::Char(a)  => a.len(),
            AnyArrayValue::I16(a)  => a.len(),
            AnyArrayValue::I32(a)  => a.len(),
            AnyArrayValue::I64(a)  => a.len(),
            AnyArrayValue::F32(a)  => a.len(),
            AnyArrayValue::F64(a)  => a.len(),
            AnyArrayValue::REF(a)  => a.len(),
        }
    }

    pub fn get(&self, index: usize) -> Option<Box<dyn std::any::Any>> {
        match self {
            AnyArrayValue::Bool(a) => a.get(index).map(|v| Box::new(*v) as Box<dyn std::any::Any>),
            AnyArrayValue::I8(a)   => a.get(index).map(|v| Box::new(*v) as Box<dyn std::any::Any>),
            AnyArrayValue::Char(a)  => a.get(index).map(|v| Box::new(*v) as Box<dyn std::any::Any>),
            AnyArrayValue::I16(a)  => a.get(index).map(|v| Box::new(*v) as Box<dyn std::any::Any>),
            AnyArrayValue::I32(a)  => a.get(index).map(|v| Box::new(*v) as Box<dyn std::any::Any>),
            AnyArrayValue::I64(a)  => a.get(index).map(|v| Box::new(*v) as Box<dyn std::any::Any>),
            AnyArrayValue::F32(a)  => a.get(index).map(|v| Box::new(*v) as Box<dyn std::any::Any>),
            AnyArrayValue::F64(a)  => a.get(index).map(|v| Box::new(*v) as Box<dyn std::any::Any>),
            AnyArrayValue::REF(a)  => a.get(index).map(|v| Box::new(*v) as Box<dyn std::any::Any>),
        }
    }

    pub fn set(&mut self, index: usize, value: Box<dyn std::any::Any>) -> Result<(), String> {
        match self {
            AnyArrayValue::Bool(a) => {
                if let Ok(v) = value.downcast::<bool>() {
                    a.set(index, *v)
                } else {
                    Err("Type mismatch for Bool array".into())
                }
            }
            AnyArrayValue::I8(a) => {
                if let Ok(v) = value.downcast::<i8>() {
                    a.set(index, *v)
                } else {
                    Err("Type mismatch for I8 array".into())
                }
            }
            AnyArrayValue::Char(a) => {
                if let Ok(v) = value.downcast::<char>() {
                    a.set(index, *v)
                } else {
                    Err("Type mismatch for Char array".into())
                }
            }
            AnyArrayValue::I16(a) => {
                if let Ok(v) = value.downcast::<i16>() {
                    a.set(index, *v)
                } else {
                    Err("Type mismatch for I16 array".into())
                }
            }
            AnyArrayValue::I32(a) => {
                if let Ok(v) = value.downcast::<i32>() {
                    a.set(index, *v)
                } else {
                    Err("Type mismatch for I32 array".into())
                }
            }
            AnyArrayValue::I64(a) => {
                if let Ok(v) = value.downcast::<i64>() {
                    a.set(index, *v)
                } else {
                    Err("Type mismatch for I64 array".into())
                }
            }
            AnyArrayValue::F32(a) => {
                if let Ok(v) = value.downcast::<f32>() {
                    a.set(index, *v)
                } else {
                    Err("Type mismatch for F32 array".into())
                }
            }
            AnyArrayValue::F64(a) => {
                if let Ok(v) = value.downcast::<f64>() {
                    a.set(index, *v)
                } else {
                    Err("Type mismatch for F64 array".into())
                }
            }
            AnyArrayValue::REF(a) => {
                if let Ok(v) = value.downcast::<Reference>() {
                    a.set(index, *v)
                } else {
                    Err("Type mismatch for Reference array".into())
                }
            }
        }
    }
}

pub fn create_array_from_char(c: char, length: u16) -> AnyArrayValue {
    match c {
        'Z' => AnyArrayValue::Bool(ArrayValue::<bool>::new(length)),
        'B' => AnyArrayValue::I8(ArrayValue::<i8>::new(length)),
        'C' => AnyArrayValue::Char(ArrayValue::<char>::new(length)),
        'S' => AnyArrayValue::I16(ArrayValue::<i16>::new(length)),
        'I' => AnyArrayValue::I32(ArrayValue::<i32>::new(length)),
        'J' => AnyArrayValue::I64(ArrayValue::<i64>::new(length)),
        'F' => AnyArrayValue::F32(ArrayValue::<f32>::new(length)),
        'D' => AnyArrayValue::F64(ArrayValue::<f64>::new(length)),
        _ => panic!("Unsupported array type: {}", c),
    }
}

pub trait ArrayElement: Sized {}

impl ArrayElement for bool {}
impl ArrayElement for char {}
impl ArrayElement for i8 {}
impl ArrayElement for i16 {}
impl ArrayElement for i32 {}
impl ArrayElement for i64 {}
impl ArrayElement for f32 {}
impl ArrayElement for f64 {}
impl ArrayElement for Reference {}