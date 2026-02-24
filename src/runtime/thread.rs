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

pub struct ThreadStack {
    stack: Vec<u8>,
}

impl ThreadStack {
    pub fn new() -> Self {
        ThreadStack {
            stack: Vec::new(),
        }
    }

    // short = i8
    pub fn push_byte(&mut self, value: i8) {
        self.stack.push(value as u8);
    }

    pub fn pop_byte(&mut self) -> Option<i8> {
        self.stack.pop().map(|b| b as i8)
    }

    // short = i16
    pub fn push_short(&mut self, value: i16) {
        self.stack.push((value >> 8) as u8);
        self.stack.push((value & 0xFF) as u8);
    }

    pub fn pop_short(&mut self) -> Option<i16> {
        let low = self.stack.pop()? as u16;
        let high = self.stack.pop()? as u16;
        Some(((high << 8) | low) as i16)
    }

    // int = i32
    pub fn push_int(&mut self, value: i32) {
        self.stack.push(((value >> 24) & 0xFF) as u8);
        self.stack.push(((value >> 16) & 0xFF) as u8);
        self.stack.push(((value >> 8) & 0xFF) as u8);
        self.stack.push((value & 0xFF) as u8);
    }

    pub fn pop_int(&mut self) -> Option<i32> {
        let b0 = self.stack.pop()? as u32;
        let b1 = self.stack.pop()? as u32;
        let b2 = self.stack.pop()? as u32;
        let b3 = self.stack.pop()? as u32;
        Some(((b3 << 24) | (b2 << 16) | (b1 << 8) | b0) as i32)
    }

    // long = i64
    pub fn push_long(&mut self, value: i64) {
        for i in (0..8).rev() {
            self.stack.push(((value >> (i * 8)) & 0xFF) as u8);
        }
    }

    pub fn pop_long(&mut self) -> Option<i64> {
        let mut result = 0i64;
        for i in 0..8 {
            let byte = self.stack.pop()? as i64;
            result |= byte << (i * 8);
        }
        Some(result)
    }

    // float = ieee 754 (f32)
    pub fn push_float(&mut self, value: f32) {
        let bytes = value.to_bits();
        self.push_int(bytes as i32);
    }

    pub fn pop_float(&mut self) -> Option<f32> {
        let int_bits = self.pop_int()? as u32;
        Some(f32::from_bits(int_bits))
    }

    // double = ieee 754 (f64)
    pub fn push_double(&mut self, value: f64) {
        let bytes = value.to_bits();
        self.push_long(bytes as i64);
    }

    pub fn pop_double(&mut self) -> Option<f64> {
        let long_bits = self.pop_long()? as u64;
        Some(f64::from_bits(long_bits))
    }

    // char = u16
    pub fn push_char(&mut self, value: char) {
        let code = value as u16;
        self.push_short(code as i16);
    }

    pub fn pop_char(&mut self) -> Option<char> {
        let code = self.pop_short()? as u16;
        Some(std::char::from_u32(code as u32)?)
    }

    // reference
    pub fn push_ref(&mut self, reference_index: u32) {
        self.push_int(reference_index as i32);
    }

    pub fn pop_ref(&mut self) -> Option<Reference> {
        let pointer = self.pop_int()? as u32;
        Some(Reference::new(pointer))
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
        s.push_int(42);
        assert_eq!(s.pop_int().unwrap(), 42);
    }

    #[test]
    fn test_push_pop_int_negative() {
        let mut s = new_stack();
        s.push_int(-1);
        assert_eq!(s.pop_int().unwrap(), -1);
    }

    #[test]
    fn test_push_pop_int_min_max() {
        let mut s = new_stack();
        s.push_int(i32::MIN);
        assert_eq!(s.pop_int().unwrap(), i32::MIN);
        s.push_int(i32::MAX);
        assert_eq!(s.pop_int().unwrap(), i32::MAX);
    }

    #[test]
    fn test_pop_int_empty_returns_none() {
        let mut s = new_stack();
        assert!(s.pop_int().is_none());
    }

    #[test]
    fn test_int_stack_lifo_order() {
        let mut s = new_stack();
        s.push_int(1);
        s.push_int(2);
        assert_eq!(s.pop_int().unwrap(), 2);
        assert_eq!(s.pop_int().unwrap(), 1);
    }

    // ── long (i64) ────────────────────────────────────────────────────────────

    #[test]
    fn test_push_pop_long_roundtrip() {
        let mut s = new_stack();
        s.push_long(123456789012345);
        assert_eq!(s.pop_long().unwrap(), 123456789012345i64);
    }

    #[test]
    fn test_push_pop_long_negative() {
        let mut s = new_stack();
        s.push_long(-1i64);
        assert_eq!(s.pop_long().unwrap(), -1i64);
    }

    #[test]
    fn test_push_pop_long_min_max() {
        let mut s = new_stack();
        s.push_long(i64::MIN);
        assert_eq!(s.pop_long().unwrap(), i64::MIN);
        s.push_long(i64::MAX);
        assert_eq!(s.pop_long().unwrap(), i64::MAX);
    }

    #[test]
    fn test_pop_long_empty_returns_none() {
        let mut s = new_stack();
        assert!(s.pop_long().is_none());
    }

    // ── float (f32) ───────────────────────────────────────────────────────────

    #[test]
    fn test_push_pop_float_roundtrip() {
        let mut s = new_stack();
        s.push_float(3.14f32);
        assert_eq!(s.pop_float().unwrap(), 3.14f32);
    }

    #[test]
    fn test_push_pop_float_zero() {
        let mut s = new_stack();
        s.push_float(0.0f32);
        assert_eq!(s.pop_float().unwrap(), 0.0f32);
    }

    #[test]
    fn test_push_pop_float_negative() {
        let mut s = new_stack();
        s.push_float(-1.5f32);
        assert_eq!(s.pop_float().unwrap(), -1.5f32);
    }

    #[test]
    fn test_pop_float_empty_returns_none() {
        let mut s = new_stack();
        assert!(s.pop_float().is_none());
    }

    // ── double (f64) ──────────────────────────────────────────────────────────

    #[test]
    fn test_push_pop_double_roundtrip() {
        let mut s = new_stack();
        s.push_double(2.718281828f64);
        assert_eq!(s.pop_double().unwrap(), 2.718281828f64);
    }

    #[test]
    fn test_push_pop_double_zero() {
        let mut s = new_stack();
        s.push_double(0.0f64);
        assert_eq!(s.pop_double().unwrap(), 0.0f64);
    }

    #[test]
    fn test_push_pop_double_min_max() {
        let mut s = new_stack();
        s.push_double(f64::MIN);
        assert_eq!(s.pop_double().unwrap(), f64::MIN);
        s.push_double(f64::MAX);
        assert_eq!(s.pop_double().unwrap(), f64::MAX);
    }

    #[test]
    fn test_pop_double_empty_returns_none() {
        let mut s = new_stack();
        assert!(s.pop_double().is_none());
    }

    // ── short (i16) ───────────────────────────────────────────────────────────

    #[test]
    fn test_push_pop_short_roundtrip() {
        let mut s = new_stack();
        s.push_short(1000i16);
        assert_eq!(s.pop_short().unwrap(), 1000i16);
    }

    #[test]
    fn test_push_pop_short_min_max() {
        let mut s = new_stack();
        s.push_short(i16::MIN);
        assert_eq!(s.pop_short().unwrap(), i16::MIN);
        s.push_short(i16::MAX);
        assert_eq!(s.pop_short().unwrap(), i16::MAX);
    }

    #[test]
    fn test_pop_short_empty_returns_none() {
        let mut s = new_stack();
        assert!(s.pop_short().is_none());
    }

    // ── byte (i8) ─────────────────────────────────────────────────────────────

    #[test]
    fn test_push_pop_byte_roundtrip() {
        let mut s = new_stack();
        s.push_byte(42i8);
        assert_eq!(s.pop_byte().unwrap(), 42i8);
    }

    #[test]
    fn test_push_pop_byte_negative() {
        let mut s = new_stack();
        s.push_byte(-1i8);
        assert_eq!(s.pop_byte().unwrap(), -1i8);
    }

    #[test]
    fn test_pop_byte_empty_returns_none() {
        let mut s = new_stack();
        assert!(s.pop_byte().is_none());
    }

    // ── char (u16 via i16) ────────────────────────────────────────────────────

    #[test]
    fn test_push_pop_char_ascii() {
        let mut s = new_stack();
        s.push_char('A');
        assert_eq!(s.pop_char().unwrap(), 'A');
    }

    #[test]
    fn test_push_pop_char_unicode() {
        let mut s = new_stack();
        s.push_char('€');
        assert_eq!(s.pop_char().unwrap(), '€');
    }

    #[test]
    fn test_pop_char_empty_returns_none() {
        let mut s = new_stack();
        assert!(s.pop_char().is_none());
    }

    // ── reference (u32) ──────────────────────────────────────────────────────

    #[test]
    fn test_push_pop_ref_roundtrip() {
        let mut s = new_stack();
        s.push_ref(99);
        let r = s.pop_ref().unwrap();
        assert_eq!(r.get_ref_index(), 99);
        assert!(!r.is_null());
    }

    #[test]
    fn test_push_pop_null_ref() {
        let mut s = new_stack();
        s.push_ref(0);
        let r = s.pop_ref().unwrap();
        assert!(r.is_null());
    }

    #[test]
    fn test_pop_ref_empty_returns_none() {
        let mut s = new_stack();
        assert!(s.pop_ref().is_none());
    }

    // ── mixed type isolation ──────────────────────────────────────────────────

    #[test]
    fn test_int_and_long_do_not_alias() {
        let mut s = new_stack();
        s.push_int(1);
        s.push_long(2);
        assert_eq!(s.pop_long().unwrap(), 2i64);
        assert_eq!(s.pop_int().unwrap(), 1i32);
    }

    #[test]
    fn test_float_and_double_do_not_alias() {
        let mut s = new_stack();
        s.push_float(1.0f32);
        s.push_double(2.0f64);
        assert_eq!(s.pop_double().unwrap(), 2.0f64);
        assert_eq!(s.pop_float().unwrap(), 1.0f32);
    }
}