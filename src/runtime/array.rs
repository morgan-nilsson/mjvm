use crate::runtime::reference_table::Reference;

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