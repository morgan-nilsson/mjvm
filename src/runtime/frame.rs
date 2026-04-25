
use std::sync::Arc;
use crate::constant_pool::ConstantPool;
use crate::jvm_error::JVMError;

static FRAME_LOCAL_VAR_MAX_SIZE: usize = 256;
static FRAME_OPERAND_STACK_MAX_SIZE: usize = 256;

// JVM slots are 32 bits. Longs and doubles occupy two consecutive slots.
#[derive(Clone, Copy, Debug)]
pub enum Slot {
    Int(i32),
    Float(f32),
    /// Object reference (index into the heap, or 0 for null)
    Ref(u32),
    /// Marks the upper half of a long/double. Never read directly.
    Upper,
    Empty,
}

pub struct Frame {
    pub pc: usize,
    pub code: Vec<u8>,
    pub local_variables: LocalVars,
    pub operand_stack: OperandStack,
    pub constant_pool: Arc<ConstantPool>,
    pub class_name: String,
    pub method_name: String,
    pub method_descriptor: String,
}

pub struct LocalVars {
    vars: Vec<Slot>,
}

pub struct OperandStack {
    slots: Vec<Slot>,
    top: usize,
    max_size: usize,
}

impl Frame {
    pub fn new(
        code: Vec<u8>,
        constant_pool: Arc<ConstantPool>,
        class_name: String,
        method_name: String,
        method_descriptor: String,
    ) -> Self {
        Self {
            pc: 0,
            code,
            local_variables: LocalVars::new(FRAME_LOCAL_VAR_MAX_SIZE),
            operand_stack: OperandStack::new(FRAME_OPERAND_STACK_MAX_SIZE),
            constant_pool,
            class_name,
            method_name,
            method_descriptor,
        }
    }

    pub fn read_byte_from_pc(&mut self) -> Result<u8, JVMError> {
        if self.pc >= self.code.len() {
            return Err(JVMError::InternalError.into());
        }
        let byte = self.code[self.pc];
        self.pc += 1;
        Ok(byte)
    }
}

impl LocalVars {
    pub fn new(max_size: usize) -> Self {
        Self {
            vars: vec![Slot::Empty; max_size],
        }
    }

    pub fn get_int(&self, index: usize) -> Result<i32, JVMError> {
        match self.vars.get(index) {
            Some(Slot::Int(value)) => Ok(*value),
            Some(_) => Err(JVMError::LocalVarTypeMismatch.into()),
            None => Err(JVMError::LocalVarOutOfBounds.into()),
        }
    }

    pub fn set_int(&mut self, index: usize, value: i32) -> Result<(), JVMError> {
        if index >= self.vars.len() {
            return Err(JVMError::LocalVarOutOfBounds.into());
        }
        self.vars[index] = Slot::Int(value);
        Ok(())
    }

    pub fn get_float(&self, index: usize) -> Result<f32, JVMError> {
        match self.vars.get(index) {
            Some(Slot::Float(value)) => Ok(*value),
            Some(_) => Err(JVMError::LocalVarTypeMismatch.into()),
            None => Err(JVMError::LocalVarOutOfBounds.into()),
        }
    }

    pub fn set_float(&mut self, index: usize, value: f32) -> Result<(), JVMError> {
        if index >= self.vars.len() {
            return Err(JVMError::LocalVarOutOfBounds.into());
        }
        self.vars[index] = Slot::Float(value);
        Ok(())
    }

    pub fn get_ref(&self, index: usize) -> Result<u32, JVMError> {
        match self.vars.get(index) {
            Some(Slot::Ref(value)) => Ok(*value),
            Some(_) => Err(JVMError::LocalVarTypeMismatch.into()),
            None => Err(JVMError::LocalVarOutOfBounds.into()),
        }
    }

    pub fn set_ref(&mut self, index: usize, value: u32) -> Result<(), JVMError> {
        if index >= self.vars.len() {
            return Err(JVMError::LocalVarOutOfBounds.into());
        }
        self.vars[index] = Slot::Ref(value);
        Ok(())
    }

    // 64 bit types
    pub fn get_long(&self, index: usize) -> Result<i64, JVMError> {
        match (self.vars.get(index), self.vars.get(index + 1)) {
            (Some(Slot::Int(low)), Some(Slot::Upper)) => Ok(*low as i64),
            (Some(Slot::Upper), Some(Slot::Int(high))) => Ok((*high as i64) << 32),
            (Some(_), Some(_)) => Err(JVMError::LocalVarTypeMismatch.into()),
            _ => Err(JVMError::LocalVarOutOfBounds.into()),
        }
    }

    pub fn set_long(&mut self, index: usize, value: i64) -> Result<(), JVMError> {
        if index + 1 >= self.vars.len() {
            return Err(JVMError::LocalVarOutOfBounds.into());
        }
        self.vars[index] = Slot::Int(value as i32);
        self.vars[index + 1] = Slot::Upper;
        Ok(())
    }

    pub fn get_double(&self, index: usize) -> Result<f64, JVMError> {
        match (self.vars.get(index), self.vars.get(index + 1)) {
            (Some(Slot::Float(low)), Some(Slot::Upper)) => Ok(*low as f64),
            (Some(Slot::Upper), Some(Slot::Float(high))) => Ok((*high as f64) * (1u64 << 32) as f64),
            (Some(_), Some(_)) => Err(JVMError::LocalVarTypeMismatch.into()),
            _ => Err(JVMError::LocalVarOutOfBounds.into()),
        }
    }

    pub fn set_double(&mut self, index: usize, value: f64) -> Result<(), JVMError> {
        if index + 1 >= self.vars.len() {
            return Err(JVMError::LocalVarOutOfBounds.into());
        }
        self.vars[index] = Slot::Float(value as f32);
        self.vars[index + 1] = Slot::Upper;
        Ok(())
    }
}

impl OperandStack {
    pub fn new(max_size: usize) -> Self {
        Self {
            slots: Vec::with_capacity(max_size),
            top: 0,
            max_size,
        }
    }

    pub fn push_int(&mut self, value: i32) -> Result<(), JVMError> {
        if self.top >= self.max_size {
            return Err(JVMError::StackOverflowError.into());
        }
        self.slots.push(Slot::Int(value));
        self.top += 1;
        Ok(())
    }

    pub fn pop_int(&mut self) -> Result<i32, JVMError> {
        if self.top == 0 {
            return Err(JVMError::EmptyStack.into());
        }
        match self.slots.pop() {
            Some(Slot::Int(value)) => {
                self.top -= 1;
                Ok(value)
            }
            Some(_) => Err(JVMError::LocalVarTypeMismatch.into()),
            None => Err(JVMError::EmptyStack.into()),
        }
    }

    pub fn push_float(&mut self, value: f32) -> Result<(), JVMError> {
        if self.top >= self.max_size {
            return Err(JVMError::StackOverflowError.into());
        }
        self.slots.push(Slot::Float(value));
        self.top += 1;
        Ok(())
    }

    pub fn pop_float(&mut self) -> Result<f32, JVMError> {
        if self.top == 0 {
            return Err(JVMError::EmptyStack.into());
        }
        match self.slots.pop() {
            Some(Slot::Float(value)) => {
                self.top -= 1;
                Ok(value)
            }
            Some(_) => Err(JVMError::LocalVarTypeMismatch.into()),
            None => Err(JVMError::EmptyStack.into()),
        }
    }

    pub fn push_ref(&mut self, value: u32) -> Result<(), JVMError> {
        if self.top >= self.max_size {
            return Err(JVMError::StackOverflowError.into());
        }
        self.slots.push(Slot::Ref(value));
        self.top += 1;
        Ok(())
    }

    pub fn pop_ref(&mut self) -> Result<u32, JVMError> {
        if self.top == 0 {
            return Err(JVMError::EmptyStack.into());
        }
        match self.slots.pop() {
            Some(Slot::Ref(value)) => {
                self.top -= 1;
                Ok(value)
            }
            Some(_) => Err(JVMError::LocalVarTypeMismatch.into()),
            None => Err(JVMError::EmptyStack.into()),
        }
    }

    pub fn push_long(&mut self, value: i64) -> Result<(), JVMError> {
        if self.top + 1 >= self.max_size {
            return Err(JVMError::StackOverflowError.into());
        }
        self.slots.push(Slot::Int(value as i32));
        self.slots.push(Slot::Upper);
        self.top += 2;
        Ok(())
    }

    pub fn pop_long(&mut self) -> Result<i64, JVMError> {
        if self.top < 2 {
            return Err(JVMError::EmptyStack.into());
        }
        match (self.slots.pop(), self.slots.pop()) {
            (Some(Slot::Upper), Some(Slot::Int(low))) => {
                self.top -= 2;
                Ok(low as i64)
            }
            (Some(Slot::Int(high)), Some(Slot::Upper)) => {
                self.top -= 2;
                Ok((high as i64) << 32)
            }
            (Some(_), Some(_)) => Err(JVMError::LocalVarTypeMismatch.into()),
            _ => Err(JVMError::EmptyStack.into()),
        }
    }

    pub fn push_double(&mut self, value: f64) -> Result<(), JVMError> {
        self.push_long(value.to_bits() as i64)
    }

    pub fn pop_double(&mut self) -> Result<f64, JVMError> {
        if self.top < 2 {
            return Err(JVMError::EmptyStack.into());
        }
        let bits = self.pop_long()?;
        Ok(f64::from_bits(bits as u64))
    }
}
