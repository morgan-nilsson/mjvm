use std::sync::Arc;
use std::sync::Mutex;
use crate::runtime::reference_table::Reference;

pub struct Thread {
    pub pc: usize,
    /// Bytecode of the current method, consumed by read_*_from_pc.
    pub code: Vec<u8>,
    /// Local variable array for the current frame (4 bytes per slot).
    pub local_vars: LocalVars,
    pub thread_stack: ThreadStack,
    pub jvm_heap: Arc<Mutex<JVMHeap>>,
    pub method_area: Arc<Mutex<MethodArea>>,
    pub native_method_stack: NativeMethodStack,
    pub wide_mode: bool,
}

impl Thread {
    /// Read one byte from `code[pc]` and advance pc.
    pub fn read_byte_from_pc(&mut self) -> Option<u8> {
        let byte = self.code.get(self.pc).copied();
        if byte.is_some() {
            self.pc += 1;
        }
        byte
    }

    /// Read a big-endian u16 from `code[pc..pc+2]` and advance pc by 2.
    pub fn read_short_from_pc(&mut self) -> Option<u16> {
        if self.pc + 2 > self.code.len() {
            return None;
        }
        let high = self.code[self.pc] as u16;
        let low  = self.code[self.pc + 1] as u16;
        self.pc += 2;
        Some((high << 8) | low)
    }

    /// Read a big-endian u32 from `code[pc..pc+4]` and advance pc by 4.
    pub fn read_int_from_pc(&mut self) -> Option<u32> {
        if self.pc + 4 > self.code.len() {
            return None;
        }
        let b = &self.code[self.pc..self.pc + 4];
        self.pc += 4;
        Some(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
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



/// JVM local variable array for a single frame.
///
/// Each slot is 4 bytes.  `long` / `double` values occupy two consecutive
/// slots (the lower-numbered slot holds the value, just as in the JVM spec).
/// Slot indices follow: `set::<i64>(n, v)` writes `v` starting at byte
/// offset `n * 4`, using 8 bytes (slots n and n+1).
pub struct LocalVars {
    data: Vec<u8>,
}

impl LocalVars {
    /// Allocate storage for `num_slots` 32-bit slots (zero-initialised).
    pub fn new(num_slots: usize) -> Self {
        LocalVars { data: vec![0u8; num_slots * 4] }
    }

    /// Load a typed value from slot `slot`.  Returns `None` when out of bounds.
    pub fn get<T: Stackable>(&self, slot: usize) -> Option<T> {
        let offset = slot * 4;
        let size   = T::SIZE;
        if offset + size > self.data.len() {
            return None;
        }
        Some(T::from_bytes(&self.data[offset..offset + size]))
    }

    /// Store a typed value into slot `slot`, growing the backing store if needed.
    pub fn set<T: Stackable>(&mut self, slot: usize, value: T) {
        let bytes  = value.to_bytes();
        let offset = slot * 4;
        if offset + bytes.len() > self.data.len() {
            self.data.resize(offset + bytes.len(), 0);
        }
        self.data[offset..offset + bytes.len()].copy_from_slice(&bytes);
    }

    pub fn byte_length(&self) -> usize {
        self.data.len()
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