use core::panic;
use std::sync::{Arc, Mutex};
use crate::runtime::reference_table::Reference;
use log::warn;

pub struct Thread {
    pub pc: usize,
    pub thread_stack: ThreadStack,
    pub jvm_heap: Arc<Mutex<JVMHeap>>,
    pub method_area: Arc<Mutex<MethodArea>>,
    pub native_method_stack: NativeMethodStack,
    pub wide_mode: bool,
}

impl Thread {
    pub fn read_byte_from_pc(&mut self) -> Option<u8> {
        todo!()
    }

    pub fn read_short_from_pc(&mut self) -> Option<u16> {
        todo!()
    }

    pub fn read_int_from_pc(&mut self) -> Option<u32> {
        todo!()
    }

    pub fn read_opcode(&mut self) -> Option<u8> {
        if self.wide_mode {
            self.wide_mode = false;
        }


        // if opcode that is read is wide then set self.wide_mode to true and read the next byte as the opcode
        self.read_byte_from_pc()
    }

    pub fn read_operands(&mut self, num_operands: usize) -> Option<Vec<u8>> {
        let mut operands = Vec::with_capacity(num_operands);
        for _ in 0..num_operands {
            operands.push(self.read_byte_from_pc()?);
        }
        Some(operands)
    }
}

use std::convert::TryInto;

pub trait Stackable: Sized {
    const SIZE: usize = std::mem::size_of::<Self>();
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Self;
}

macro_rules! impl_stackable_for {
    ($t:ty) => {
        impl Stackable for $t {
            fn to_bytes(&self) -> Vec<u8> {
                self.to_ne_bytes().to_vec()
            }

            fn from_bytes(bytes: &[u8]) -> Self {
                let array: [u8; std::mem::size_of::<$t>()] = bytes.try_into().expect("Wrong length");
                <$t>::from_ne_bytes(array)
            }
        }
    };
}

impl_stackable_for!(i8);
impl_stackable_for!(i16);
impl_stackable_for!(i32);
impl_stackable_for!(i64);
impl_stackable_for!(f32);
impl_stackable_for!(f64);

impl Stackable for char {
    const SIZE: usize = 2;

    fn to_bytes(&self) -> Vec<u8> {
        (*self as u16).to_ne_bytes().to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let array: [u8; 2] = bytes.try_into().expect("Wrong length");
        char::from_u32(u16::from_ne_bytes(array) as u32).expect("Invalid char")
    }
}

impl Stackable for bool {
    fn to_bytes(&self) -> Vec<u8> {
        vec![*self as u8]
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        bytes[0] != 0
    }
}

impl Stackable for Reference {
    fn to_bytes(&self) -> Vec<u8> {
        self.ref_index.to_ne_bytes().to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let array: [u8; 4] = bytes.try_into().expect("Wrong length");
        Reference::new(u32::from_ne_bytes(array))
    }
}

pub struct ThreadStack {
    data: Vec<u8>,
}

impl ThreadStack {
    pub fn new() -> Self {
        ThreadStack {
            data: Vec::new(),
        }
    }

    pub fn push<T: Stackable>(&mut self, value: T) {
        let bytes = value.to_bytes();
        self.data.extend_from_slice(&bytes);
    }

    pub fn pop<T: Stackable>(&mut self) -> Option<T> {
        let size = T::SIZE;
        if self.data.len() < size {
            return None;
        }
        let start = self.data.len() - size;
        let bytes = self.data[start..].to_vec();
        self.data.truncate(start);
        Some(T::from_bytes(&bytes))
    }
}



pub struct JVMHeap {

}

pub struct MethodArea {

}

pub struct NativeMethodStack {

}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_stack() -> ThreadStack {
        ThreadStack::new()
    }

    // ── int (i32) ─────────────────────────────────────────────────────────────

    #[test]
    fn test_push_pop_int_roundtrip() {
        let mut s = new_stack();
        s.push::<i32>(42);
        assert_eq!(s.pop::<i32>().unwrap(), 42);
    }

    #[test]
    fn test_push_pop_int_negative() {
        let mut s = new_stack();
        s.push::<i32>(-1);
        assert_eq!(s.pop::<i32>().unwrap(), -1);
    }

    #[test]
    fn test_push_pop_int_min_max() {
        let mut s = new_stack();
        s.push::<i32>(i32::MIN);
        assert_eq!(s.pop::<i32>().unwrap(), i32::MIN);
        s.push::<i32>(i32::MAX);
        assert_eq!(s.pop::<i32>().unwrap(), i32::MAX);
    }

    #[test]
    fn test_pop_int_empty_returns_none() {
        let mut s = new_stack();
        assert!(s.pop::<i32>().is_none());
    }

    #[test]
    fn test_int_stack_lifo_order() {
        let mut s = new_stack();
        s.push::<i32>(1);
        s.push::<i32>(2);
        assert_eq!(s.pop::<i32>().unwrap(), 2);
        assert_eq!(s.pop::<i32>().unwrap(), 1);
    }

    // ── long (i64) ────────────────────────────────────────────────────────────

    #[test]
    fn test_push_pop_long_roundtrip() {
        let mut s = new_stack();
        s.push::<i64>(123456789012345);
        assert_eq!(s.pop::<i64>().unwrap(), 123456789012345i64);
    }

    #[test]
    fn test_push_pop_long_negative() {
        let mut s = new_stack();
        s.push::<i64>(-1i64);
        assert_eq!(s.pop::<i64>().unwrap(), -1i64);
    }

    #[test]
    fn test_push_pop_long_min_max() {
        let mut s = new_stack();
        s.push::<i64>(i64::MIN);
        assert_eq!(s.pop::<i64>().unwrap(), i64::MIN);
        s.push::<i64>(i64::MAX);
        assert_eq!(s.pop::<i64>().unwrap(), i64::MAX);
    }

    #[test]
    fn test_pop_long_empty_returns_none() {
        let mut s = new_stack();
        assert!(s.pop::<i64>().is_none());
    }

    // ── float (f32) ───────────────────────────────────────────────────────────

    #[test]
    fn test_push_pop_float_roundtrip() {
        let mut s = new_stack();
        s.push::<f32>(3.14f32);
        assert_eq!(s.pop::<f32>().unwrap(), 3.14f32);
    }

    #[test]
    fn test_push_pop_float_zero() {
        let mut s = new_stack();
        s.push::<f32>(0.0f32);
        assert_eq!(s.pop::<f32>().unwrap(), 0.0f32);
    }

    #[test]
    fn test_push_pop_float_negative() {
        let mut s = new_stack();
        s.push::<f32>(-1.5f32);
        assert_eq!(s.pop::<f32>().unwrap(), -1.5f32);
    }

    #[test]
    fn test_pop_float_empty_returns_none() {
        let mut s = new_stack();
        assert!(s.pop::<f32>().is_none());
    }

    // ── double (f64) ──────────────────────────────────────────────────────────

    #[test]
    fn test_push_pop_double_roundtrip() {
        let mut s = new_stack();
        s.push::<f64>(2.718281828f64);
        assert_eq!(s.pop::<f64>().unwrap(), 2.718281828f64);
    }

    #[test]
    fn test_push_pop_double_zero() {
        let mut s = new_stack();
        s.push::<f64>(0.0f64);
        assert_eq!(s.pop::<f64>().unwrap(), 0.0f64);
    }

    #[test]
    fn test_push_pop_double_min_max() {
        let mut s = new_stack();
        s.push::<f64>(f64::MIN);
        assert_eq!(s.pop::<f64>().unwrap(), f64::MIN);
        s.push::<f64>(f64::MAX);
        assert_eq!(s.pop::<f64>().unwrap(), f64::MAX);
    }

    #[test]
    fn test_pop_double_empty_returns_none() {
        let mut s = new_stack();
        assert!(s.pop::<f64>().is_none());
    }

    // ── short (i16) ───────────────────────────────────────────────────────────

    #[test]
    fn test_push_pop_short_roundtrip() {
        let mut s = new_stack();
        s.push::<i16>(1000i16);
        assert_eq!(s.pop::<i16>().unwrap(), 1000i16);
    }

    #[test]
    fn test_push_pop_short_min_max() {
        let mut s = new_stack();
        s.push::<i16>(i16::MIN);
        assert_eq!(s.pop::<i16>().unwrap(), i16::MIN);
        s.push::<i16>(i16::MAX);
        assert_eq!(s.pop::<i16>().unwrap(), i16::MAX);
    }

    #[test]
    fn test_pop_short_empty_returns_none() {
        let mut s = new_stack();
        assert!(s.pop::<i16>().is_none());
    }

    // ── byte (i8) ─────────────────────────────────────────────────────────────

    #[test]
    fn test_push_pop_byte_roundtrip() {
        let mut s = new_stack();
        s.push::<i8>(42i8);
        assert_eq!(s.pop::<i8>().unwrap(), 42i8);
    }

    #[test]
    fn test_push_pop_byte_negative() {
        let mut s = new_stack();
        s.push::<i8>(-1i8);
        assert_eq!(s.pop::<i8>().unwrap(), -1i8);
    }

    #[test]
    fn test_pop_byte_empty_returns_none() {
        let mut s = new_stack();
        assert!(s.pop::<i8>().is_none());
    }

    // ── char (u16 via i16) ────────────────────────────────────────────────────

    #[test]
    fn test_push_pop_char_ascii() {
        let mut s = new_stack();
        s.push::<char>('A');
        assert_eq!(s.pop::<char>().unwrap(), 'A');
    }

    #[test]
    fn test_push_pop_char_unicode() {
        let mut s = new_stack();
        s.push::<char>('€');
        assert_eq!(s.pop::<char>().unwrap(), '€');
    }

    #[test]
    fn test_pop_char_empty_returns_none() {
        let mut s = new_stack();
        assert!(s.pop::<char>().is_none());
    }

    // ── reference (u32) ──────────────────────────────────────────────────────

    #[test]
    fn test_push_pop_ref_roundtrip() {
        let mut s = new_stack();
        s.push(Reference::new(99));
        let r = s.pop::<Reference>().unwrap();
        assert_eq!(r.get_ref_index(), 99);
        assert!(!r.is_null());
    }

    #[test]
    fn test_push_pop_null_ref() {
        let mut s = new_stack();
        s.push(Reference::new(0));
        let r = s.pop::<Reference>().unwrap();
        assert!(r.is_null());
    }

    #[test]
    fn test_pop_ref_empty_returns_none() {
        let mut s = new_stack();
        assert!(s.pop::<Reference>().is_none());
    }

    // ── mixed type isolation ──────────────────────────────────────────────────

    #[test]
    fn test_int_and_long_do_not_alias() {
        let mut s = new_stack();
        s.push::<i32>(1);
        s.push::<i64>(2);
        assert_eq!(s.pop::<i64>().unwrap(), 2i64);
        assert_eq!(s.pop::<i32>().unwrap(), 1i32);
    }

    #[test]
    fn test_float_and_double_do_not_alias() {
        let mut s = new_stack();
        s.push::<f32>(1.0f32);
        s.push::<f64>(2.0f64);
        assert_eq!(s.pop::<f64>().unwrap(), 2.0f64);
        assert_eq!(s.pop::<f32>().unwrap(), 1.0f32);
    }
}