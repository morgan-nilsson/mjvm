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

    pub fn pop_byte(&mut self) -> Option<u8> {
        self.stack.pop()
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