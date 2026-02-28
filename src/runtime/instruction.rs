use crate::runtime::array::create_array_from_char;
use crate::runtime::thread::Thread;
use crate::runtime::reference_table::Reference;
use crate::runtime::array::AnyArrayValue;
use crate::runtime::reference_table::NULL_REF;
use crate::runtime::reference_table::REFERENCE_TABLE;
use crate::runtime::reference_table::ReferenceValue;

use anyhow::Ok;
use anyhow::Result;
use thiserror::Error;

pub fn preform_instruction(thread: &mut Thread) {
    let opcode = thread.read_byte_from_pc().expect("Failed to read opcode from bytecode");

    if let Err(e) = call_by_opcode(thread, opcode) {
        eprintln!("Error executing instruction with opcode {:#x}: {}", opcode, e);
    }
}

fn call_by_opcode(thread: &mut Thread, opcode: u8) -> Result<()> {
    match opcode {
        0x00 => { nop(thread) },
        0x01 => { aconst_null(thread) },
        0x02 => { iconst_m1(thread) },
        0x03 => { iconst_0(thread) },
        0x04 => { iconst_1(thread) },
        0x05 => { iconst_2(thread) },
        0x06 => { iconst_3(thread) },
        0x07 => { iconst_4(thread) },
        0x08 => { iconst_5(thread) },
        0x09 => { lconst_0(thread) },
        0x0a => { lconst_1(thread) },
        0x0b => { fconst_0(thread) },
        0x0c => { fconst_1(thread) },
        0x0d => { fconst_2(thread) },
        0x0e => { dconst_0(thread) },
        0x0f => { dconst_1(thread) },
        0x10 => { bipush(thread) },
        0x11 => { sipush(thread) },
        0x12 => { ldc(thread) },
        0x13 => { ldc_w(thread) },
        0x14 => { ldc2_w(thread) },

        0x15 => { iload(thread) },
        0x16 => { lload(thread) },
        0x17 => { fload(thread) },
        0x18 => { dload(thread) },
        0x19 => { aload(thread) },
        0x1a => { iload_0(thread) },
        0x1b => { iload_1(thread) },
        0x1c => { iload_2(thread) },
        0x1d => { iload_3(thread) },
        0x1e => { lload_0(thread) },
        0x1f => { lload_1(thread) },
        0x20 => { lload_2(thread) },
        0x21 => { lload_3(thread) },
        0x22 => { fload_0(thread) },
        0x23 => { fload_1(thread) },
        0x24 => { fload_2(thread) },
        0x25 => { fload_3(thread) },
        0x26 => { dload_0(thread) },
        0x27 => { dload_1(thread) },
        0x28 => { dload_2(thread) },
        0x29 => { dload_3(thread) },
        0x2a => { aload_0(thread) },
        0x2b => { aload_1(thread) },
        0x2c => { aload_2(thread) },
        0x2d => { aload_3(thread) },
        0x2e => { iaload(thread) },
        0x2f => { laload(thread) },
        0x30 => { faload(thread) },
        0x31 => { daload(thread) },
        0x32 => { aaload(thread) },
        0x33 => { baload(thread) },
        0x34 => { caload(thread) },
        0x35 => { saload(thread) },

        0x36 => { istore(thread) },
        0x37 => { lstore(thread) },
        0x38 => { fstore(thread) },
        0x39 => { dstore(thread) },
        0x3a => { astore(thread) },
        0x3b => { istore_0(thread) },
        0x3c => { istore_1(thread) },
        0x3d => { istore_2(thread) },
        0x3e => { istore_3(thread) },
        0x3f => { lstore_0(thread) },
        0x40 => { lstore_1(thread) },
        0x41 => { lstore_2(thread) },
        0x42 => { lstore_3(thread) },
        0x43 => { fstore_0(thread) },
        0x44 => { fstore_1(thread) },
        0x45 => { fstore_2(thread) },
        0x46 => { fstore_3(thread) },
        0x47 => { dstore_0(thread) },
        0x48 => { dstore_1(thread) },
        0x49 => { dstore_2(thread) },
        0x4a => { dstore_3(thread) },
        0x4b => { astore_0(thread) },
        0x4c => { astore_1(thread) },
        0x4d => { astore_2(thread) },
        0x4e => { astore_3(thread) },
        0x4f => { iastore(thread) },
        0x50 => { lastore(thread) },
        0x51 => { fastore(thread) },
        0x52 => { dastore(thread) },
        0x53 => { aastore(thread) },
        0x54 => { bastore(thread) },
        0x55 => { castore(thread) },
        0x56 => { sastore(thread) },

        0x57 => { pop(thread) },
        0x58 => { pop2(thread) },
        0x59 => { dup(thread) },
        0x5a => { dup_x1(thread) },
        0x5b => { dup_x2(thread) },
        0x5c => { dup2(thread) },
        0x5d => { dup2_x1(thread) },
        0x5e => { dup2_x2(thread) },
        0x5f => { swap(thread) },

        0x60 => { iadd(thread) },
        0x61 => { ladd(thread) },
        0x62 => { fadd(thread) },
        0x63 => { dadd(thread) },
        0x64 => { isub(thread) },
        0x65 => { lsub(thread) },
        0x66 => { fsub(thread) },
        0x67 => { dsub(thread) },
        0x68 => { imul(thread) },
        0x69 => { lmul(thread) },
        0x6a => { fmul(thread) },
        0x6b => { dmul(thread) },
        0x6c => { idiv(thread) },
        0x6d => { ldiv(thread) },
        0x6e => { fdiv(thread) },
        0x6f => { ddiv(thread) },
        0x70 => { irem(thread) },
        0x71 => { lrem(thread) },
        0x72 => { frem(thread) },
        0x73 => { drem(thread) },
        0x74 => { ineg(thread) },
        0x75 => { lneg(thread) },
        0x76 => { fneg(thread) },
        0x77 => { dneg(thread) },
        0x78 => { ishl(thread) },
        0x79 => { lshl(thread) },
        0x7a => { ishr(thread) },
        0x7b => { lshr(thread) },
        0x7c => { iushr(thread) },
        0x7d => { lushr(thread) },
        0x7e => { iand(thread) },
        0x7f => { land(thread) },
        0x80 => { ior(thread) },
        0x81 => { lor(thread) },
        0x82 => { ixor(thread) },
        0x83 => { lxor(thread) },
        0x84 => { iinc(thread) },

        0x85 => { i2l(thread) },
        0x86 => { i2f(thread) },
        0x87 => { i2d(thread) },
        0x88 => { l2i(thread) },
        0x89 => { l2f(thread) },
        0x8a => { l2d(thread) },
        0x8b => { f2i(thread) },
        0x8c => { f2l(thread) },
        0x8d => { f2d(thread) },
        0x8e => { d2i(thread) },
        0x8f => { d2l(thread) },
        0x90 => { d2f(thread) },
        0x91 => { i2b(thread) },
        0x92 => { i2c(thread) },
        0x93 => { i2s(thread) },

        // comparisons
        0x94 => { lcmp(thread) },
        0x95 => { fcmpl(thread) },
        0x96 => { fcmpg(thread) },
        0x97 => { dcmpl(thread) },
        0x98 => { dcmpg(thread) },
        0x99 => { ifeq(thread) },
        0x9a => { ifne(thread) },
        0x9b => { iflt(thread) },
        0x9c => { ifge(thread) },
        0x9d => { ifgt(thread) },
        0x9e => { ifle(thread) },
        0x9f => { if_icmpeq(thread) },
        0xa0 => { if_icmpne(thread) },
        0xa1 => { if_icmplt(thread) },
        0xa2 => { if_icmpge(thread) },
        0xa3 => { if_icmpgt(thread) },
        0xa4 => { if_icmple(thread) },
        0xa5 => { if_acmpeq(thread) },
        0xa6 => { if_acmpne(thread) },

        // control
        0xa7 => { goto(thread) },
        0xa8 => { jsr(thread) },
        0xa9 => { ret(thread) },
        0xaa => { tableswitch(thread) },
        0xab => { lookupswitch(thread) },
        0xac => { ireturn(thread) },
        0xad => { lreturn(thread) },
        0xae => { freturn(thread) },
        0xaf => { dreturn(thread) },
        0xb0 => { areturn(thread) },
        0xb1 => { return_(thread) },

        // references
        0xb2 => { getstatic(thread) },
        0xb3 => { putstatic(thread) },
        0xb4 => { getfield(thread) },
        0xb5 => { putfield(thread) },
        0xb6 => { invokevirtual(thread) },
        0xb7 => { invokespecial(thread) },
        0xb8 => { invokestatic(thread) },
        0xb9 => { invokeinterface(thread) },
        0xba => { invokedynamic(thread) },
        0xbb => { new(thread) },
        0xbc => { newarray(thread) },
        0xbd => { anewarray(thread) },
        0xbe => { arraylength(thread) },
        0xbf => { athrow(thread) },
        0xc0 => { checkcast(thread) },
        0xc1 => { instanceof(thread) },
        0xc2 => { monitorenter(thread) },
        0xc3 => { monitorexit(thread) },

        // extended
        0xc4 => { wide(thread) },
        0xc5 => { multianewarray(thread) },
        0xc6 => { ifnull(thread) },
        0xc7 => { ifnonnull(thread) },
        0xc8 => { goto_w(thread) },
        0xc9 => { jsr_w(thread) },

        // reserved
        0xca..=0xff => { Err(InstructionError::UnknownError.into()) },


    }
}

#[derive(Error, Debug)]
pub enum InstructionError {
    // VirtualMachineErrors
    #[error("An internal error occurred during instruction execution")]
    InternalError,
    #[error("The JVM ran out of memory during instruction execution")]
    OutOfMemoryError,
    #[error("The thread stack overflowed during instruction execution")]
    StackOverflowError,
    #[error("An unknown error occurred during instruction execution")]
    UnknownError,

    #[error("A null pointer was dereferenced during instruction execution")]
    NullPointerException,

    #[error("An array index was out of bounds during instruction execution")]
    ArrayIndexOutOfBoundsException,

    #[error("An illegal monitor state was encountered during instruction execution")]
    IllegalMonitorStateException,

    #[error("A class cast was attempted during instruction execution")]
    ClassCastException,

    #[error("An arithmetic error occurred during instruction execution")]
    ArithmeticException,

    #[error("A negative array size was specified during instruction execution")]
    NegativeArraySizeException,

}

fn aaload(thread: &mut Thread) -> Result<()> {
    let index = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the array referenced
    let lock = REFERENCE_TABLE.lock().map_err(|_| InstructionError::InternalError)?;
    let array = lock.get_reference(&array_ref).ok_or(InstructionError::InternalError)?;

    // ensure this is an array
    if !array.is_array() {
        return Err(InstructionError::InternalError.into());
    }

    let array = array.as_array().ok_or(InstructionError::InternalError)?;

    // ensure the array is of type [reference]
    if !matches!(array, AnyArrayValue::REF(_)) {
        return Err(InstructionError::InternalError.into());
    }

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException
    if index < 0 || index >= array.len() as i32 {
        return Err(InstructionError::ArrayIndexOutOfBoundsException.into());
    }

    // get the value
    let value = array.get(index as usize).ok_or(InstructionError::InternalError)?;

    // ensure the value is a reference else throw InstructionsError::InternalError
    if value.downcast_ref::<Reference>().is_none() {
        return Err(InstructionError::InternalError.into());
    }

    let ref_value = value.downcast_ref::<Reference>().ok_or(InstructionError::InternalError)?;

    // push the value onto the stack
    thread.thread_stack.push::<Reference>(*ref_value);

    Ok(())
}

fn aastore(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;
    let index = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the array referenced
    let mut lock = REFERENCE_TABLE.lock().map_err(|_| InstructionError::InternalError)?;
    let array = lock.get_mut_reference(&array_ref).ok_or(InstructionError::InternalError)?;

    // ensure this is an array
    if !array.is_array() {
        return Err(InstructionError::InternalError.into());
    }

    let array: &mut AnyArrayValue = array.as_mut_array().ok_or(InstructionError::InternalError)?;

    // ensure the array is of type [reference]
    if !matches!(array, AnyArrayValue::REF(_)) {
        return Err(InstructionError::InternalError.into());
    }

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException
    if index < 0 || index >= array.len() as i32 {
        return Err(InstructionError::ArrayIndexOutOfBoundsException.into());
    }

    // set the value at the index to the value popped from the stack
    array.set(index as usize, Box::new(value)).map_err(|_| InstructionError::InternalError)?;

    Ok(())
}

fn aconst_null(thread: &mut Thread) -> Result<()> {

    thread.thread_stack.push(Reference::new(NULL_REF.get_ref_index()));

    Ok(())
}

fn aload(thread: &mut Thread) -> Result<()> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError
    if index >= thread.local_vars.byte_length() as u16 {
        return Err(InstructionError::InternalError.into());
    }

    // ensure the local variable at the index is a reference else throw InstructionsError::InternalError
    let local_var = thread.local_vars.get::<Reference>(index as usize).ok_or(InstructionError::InternalError)?;

    // push the reference onto the stack
    thread.thread_stack.push(local_var);

    Ok(())
}

fn aload_0(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 0 is a reference else throw InstructionsError::InternalError
    let local_var = thread.local_vars.get::<Reference>(0).ok_or(InstructionError::InternalError)?;

    // push the reference from the local variable at index 0 onto the stack
    thread.thread_stack.push(local_var);

    Ok(())
}

fn aload_1(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 1 is a reference else throw InstructionsError::InternalError
    let local_var = thread.local_vars.get::<Reference>(1).ok_or(InstructionError::InternalError)?;

    // push the reference from the local variable at index 1 onto the stack
    thread.thread_stack.push(local_var);

    Ok(())
}

fn aload_2(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 2 is a reference else throw InstructionsError::InternalError
    let local_var = thread.local_vars.get::<Reference>(2).ok_or(InstructionError::InternalError)?;

    // push the reference from the local variable at index 2 onto the stack
    thread.thread_stack.push(local_var);

    Ok(())
}

fn aload_3(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 3 is a reference else throw InstructionsError::InternalError
    let local_var = thread.local_vars.get::<Reference>(3).ok_or(InstructionError::InternalError)?;

    // push the reference from the local variable at index 3 onto the stack
    thread.thread_stack.push(local_var);

    Ok(())
}

fn anewarray(thread: &mut Thread) -> Result<()> {
    let index = thread.read_byte_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a class reference else throw InstructionsError::InternalError

    // get the class referenced

    // pop the count from the stack

    // ensure count is non-negative else throw NegativeArraySizeException

    // create a new array of the class referenced with the count and push a reference to it onto the stack

    todo!("Implement anewarray instruction");
}

fn areturn(thread: &mut Thread) -> Result<()> {

    todo!("Implement areturn instruction");

}

fn arraylength(thread: &mut Thread) -> Result<()> {

    let array_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the array referenced
    let lock = REFERENCE_TABLE.lock().map_err(|_| InstructionError::InternalError)?;
    let array = lock.get_reference(&array_ref).ok_or(InstructionError::InternalError)?;

    // ensure this is an array
    if !array.is_array() {
        return Err(InstructionError::InternalError.into());
    }

    let array = array.as_array().ok_or(InstructionError::InternalError)?;

    // push the length of the array onto the stack
    thread.thread_stack.push(array.len() as i32);

    Ok(())
}

fn astore(thread: &mut Thread) -> Result<()> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError
    if index >= thread.local_vars.byte_length() as u16 {
        return Err(InstructionError::InternalError.into());
    }

    // ensure the local variable at the index is a reference else throw InstructionsError::InternalError
    thread.local_vars.get::<Reference>(index as usize).ok_or(InstructionError::InternalError)?;

    // pop the reference from the stack and store it in the local variable at the index
    let value = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(index as usize, value);

    Ok(())
}

fn astore_0(thread: &mut Thread) -> Result<()> {
    // pop the reference from the stack and store it in the local variable at index 0
    let value = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(0, value);

    Ok(())
}

fn astore_1(thread: &mut Thread) -> Result<()> {
    // pop the reference from the stack and store it in the local variable at index 1
    let value = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(1, value);

    Ok(())
}

fn astore_2(thread: &mut Thread) -> Result<()> {
    // pop the reference from the stack and store it in the local variable at index 2
    let value = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(2, value);

    Ok(())
}

fn astore_3(thread: &mut Thread) -> Result<()> {
    // pop the reference from the stack and store it in the local variable at index 3
    let value = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(3, value);

    Ok(())
}

fn athrow(thread: &mut Thread) -> Result<()> {
    let exception_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if exception_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the exception object referenced

    // throw the exception

    todo!("Implement athrow instruction");
}

fn baload(thread: &mut Thread) -> Result<()> {
    let index = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the array referenced
    let lock = REFERENCE_TABLE.lock().map_err(|_| InstructionError::InternalError)?;
    let array = lock.get_reference(&array_ref).ok_or(InstructionError::InternalError)?;

    // ensure this is an array
    if !array.is_array() {
        return Err(InstructionError::InternalError.into());
    }

    let array = array.as_array().ok_or(InstructionError::InternalError)?;

    // ensure the array is of type boolean
    if !matches!(array, AnyArrayValue::Bool(_)){
        return Err(InstructionError::InternalError.into());
    }

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException
    if index < 0 || index >= array.len() as i32 {
        return Err(InstructionError::ArrayIndexOutOfBoundsException.into());
    }

    // push the value at the index onto the stack
    let value: bool = *array.get(index as usize).ok_or(InstructionError::InternalError)?.downcast_ref::<bool>().ok_or(InstructionError::InternalError)?;

    thread.thread_stack.push(value);

    Ok(())
}

fn bastore(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let index = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the array referenced
    let mut lock = REFERENCE_TABLE.lock().map_err(|_| InstructionError::InternalError)?;
    let array = lock.get_mut_reference(&array_ref).ok_or(InstructionError::InternalError)?;

    // ensure this is an array
    if !array.is_array() {
        return Err(InstructionError::InternalError.into());
    }

    let array: &mut AnyArrayValue = array.as_mut_array().ok_or(InstructionError::InternalError)?;

    // ensure the array is of type boolean
    if !matches!(array, AnyArrayValue::Bool(_)){
        return Err(InstructionError::InternalError.into());
    }

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException
    if index < 0 || index >= array.len() as i32 {
        return Err(InstructionError::ArrayIndexOutOfBoundsException.into());
    }

    // set the value at the index to the value popped from the stack
    array.set(index as usize, Box::new(value != 0)).map_err(|_| InstructionError::InternalError)?;

    Ok(())

}

fn bipush(thread: &mut Thread) -> Result<()> {
    let value = thread.read_byte_from_pc().ok_or(InstructionError::InternalError)? as i8;

    thread.thread_stack.push::<i32>(value as i32);

    Ok(())
}

fn caload(thread: &mut Thread) -> Result<()> {
    let index = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the array referenced
    let lock = REFERENCE_TABLE.lock().map_err(|_| InstructionError::InternalError)?;
    let array = lock.get_reference(&array_ref).ok_or(InstructionError::InternalError)?;

    // ensure this is an array
    if !array.is_array() {
        return Err(InstructionError::InternalError.into());
    }

    let array = array.as_array().ok_or(InstructionError::InternalError)?;

    // ensure the array is of type char
    if !matches!(array, AnyArrayValue::Char(_)){
        return Err(InstructionError::InternalError.into());
    }

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException
    if index < 0 || index >= array.len() as i32 {
        return Err(InstructionError::ArrayIndexOutOfBoundsException.into());
    }

    // push the value at the index onto the stack
    let value: char = *array.get(index as usize).ok_or(InstructionError::InternalError)?.downcast_ref::<char>().ok_or(InstructionError::InternalError)?;
    thread.thread_stack.push(value);

    Ok(())
}

fn castore(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<char>().ok_or(InstructionError::InternalError)?;
    let index = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the array referenced
    let mut lock = REFERENCE_TABLE.lock().map_err(|_| InstructionError::InternalError)?;
    let array = lock.get_mut_reference(&array_ref).ok_or(InstructionError::InternalError)?;

    // ensure this is an array
    if !array.is_array() {
        return Err(InstructionError::InternalError.into());
    }

    let array: &mut AnyArrayValue = array.as_mut_array().ok_or(InstructionError::InternalError)?;

    // ensure the array is of type char
    if !matches!(array, AnyArrayValue::Char(_)){
        return Err(InstructionError::InternalError.into());
    }

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException
    if index < 0 || index >= array.len() as i32 {
        return Err(InstructionError::ArrayIndexOutOfBoundsException.into());
    }

    // set the value at the index to the value popped from the stack
    array.set(index as usize, Box::new(value)).map_err(|_| InstructionError::InternalError)?;

    Ok(())
}

fn checkcast(thread: &mut Thread) -> Result<()> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a class reference else throw InstructionsError::InternalError

    // get the class referenced

    // ensure the object reference on top of the stack is an instance of the class referenced else throw InstructionsError::InternalError

    todo!("Implement checkcast instruction");
}

fn d2f(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;

    // convert the double value to a float value
    // push the float value onto the stack
    thread.thread_stack.push(value as f32);

    Ok(())
}

fn d2i(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;

    // convert the double value to an int value
    // push the int value onto the stack
    thread.thread_stack.push(value as i32);

    Ok(())
}

fn d2l(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;

    // convert the double value to a long value
    // push the long value onto the stack
    thread.thread_stack.push(value as i64);

    Ok(())
}

fn dadd(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;

    // add the two double values
    // push the result onto the stack
    thread.thread_stack.push(value1 + value2);

    Ok(())
}

fn daload(thread: &mut Thread) -> Result<()> {
    let index = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the array referenced
    let lock = REFERENCE_TABLE.lock().map_err(|_| InstructionError::InternalError)?;
    let array = lock.get_reference(&array_ref).ok_or(InstructionError::InternalError)?;

    // ensure this is an array
    if !array.is_array() {
        return Err(InstructionError::InternalError.into());
    }

    let array = array.as_array().ok_or(InstructionError::InternalError)?;

    // ensure the array is of type double
    if !matches!(array, AnyArrayValue::F64(_)){
        return Err(InstructionError::InternalError.into());
    }

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException
    if index < 0 || index >= array.len() as i32 {
        return Err(InstructionError::ArrayIndexOutOfBoundsException.into());
    }

    // push the value at the index onto the stack
    let value: f64 = *array.get(index as usize).ok_or(InstructionError::InternalError)?.downcast_ref::<f64>().ok_or(InstructionError::InternalError)?;
    thread.thread_stack.push(value);

    Ok(())

}

fn dastore(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;
    let index = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the array referenced
    let mut lock = REFERENCE_TABLE.lock().map_err(|_| InstructionError::InternalError)?;
    let array = lock.get_mut_reference(&array_ref).ok_or(InstructionError::InternalError)?;

    // ensure this is an array
    if !array.is_array() {
        return Err(InstructionError::InternalError.into());
    }

    let array: &mut AnyArrayValue = array.as_mut_array().ok_or(InstructionError::InternalError)?;

    // ensure the array is of type double
    if !matches!(array, AnyArrayValue::F64(_)){
        return Err(InstructionError::InternalError.into());
    }

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException
    if index < 0 || index >= array.len() as i32 {
        return Err(InstructionError::ArrayIndexOutOfBoundsException.into());
    }

    // set the value at the index to the value popped from the stack
    array.set(index as usize, Box::new(value)).map_err(|_| InstructionError::InternalError)?;

    Ok(())
}

fn dcmpg(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;

    // compare the two double values
    // push the result onto the stack
    if value1.is_nan() || value2.is_nan() || value1 > value2 {
        thread.thread_stack.push(1);
    } else if value1 == value2 {
        thread.thread_stack.push(0);
    } else {
        thread.thread_stack.push(-1);
    }

    Ok(())
}

fn dcmpl(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;

    // compare the two double values
    // push the result onto the stack
    if value1.is_nan() || value2.is_nan() {
        thread.thread_stack.push(-1);
    } else if value1 > value2 {
        thread.thread_stack.push(1);
    } else if value1 == value2 {
        thread.thread_stack.push(0);
    } else {
        thread.thread_stack.push(-1);
    }

    Ok(())
}

fn dconst_0(thread: &mut Thread) -> Result<()> {
    thread.thread_stack.push::<f64>(0.0);

    Ok(())
}

fn dconst_1(thread: &mut Thread) -> Result<()> {
    thread.thread_stack.push::<f64>(1.0);

    Ok(())
}

fn ddiv(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;

    // divide the two double values
    // push the result onto the stack
    if value2 == 0.0 {
        return Err(InstructionError::ArithmeticException.into());
    }

    thread.thread_stack.push(value1 / value2);

    Ok(())
}

fn dload(thread: &mut Thread) -> Result<()> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError
    if index >= thread.local_vars.byte_length() as u16 {
        return Err(InstructionError::InternalError.into());
    }

    // ensure the local variable at the index is a double else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<f64>(index as usize).ok_or(InstructionError::InternalError)?;

    // push the double value from the local variable at the index onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn dload_0(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 0 is a double else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<f64>(0).ok_or(InstructionError::InternalError)?;

    // push the double value from the local variable at index 0 onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn dload_1(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 1 is a double else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<f64>(1).ok_or(InstructionError::InternalError)?;

    // push the double value from the local variable at index 1 onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn dload_2(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 2 is a double else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<f64>(2).ok_or(InstructionError::InternalError)?;

    // push the double value from the local variable at index 2 onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn dload_3(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 3 is a double else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<f64>(3).ok_or(InstructionError::InternalError)?;

    // push the double value from the local variable at index 3 onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn dmul(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;

    // multiply the two double values
    // push the result onto the stack
    thread.thread_stack.push(value1 * value2);

    Ok(())
}

fn dneg(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;

    // negate the double value
    // push the result onto the stack
    thread.thread_stack.push(-value);

    Ok(())
}

fn drem(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;

    // compute the remainder of the division of the two double values
    // push the result onto the stack
    if value2 == 0.0 {
        return Err(InstructionError::ArithmeticException.into());
    }

    thread.thread_stack.push(value1 % value2);

    Ok(())
}

fn dreturn(thread: &mut Thread) -> Result<()> {

    todo!("Implement dreturn instruction");

}

fn dstore(thread: &mut Thread) -> Result<()> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError
    if index >= thread.local_vars.byte_length() as u16 {
        return Err(InstructionError::InternalError.into());
    }

    // ensure the local variable at the index is a double else throw InstructionsError::InternalError
    thread.local_vars.get::<f64>(index as usize).ok_or(InstructionError::InternalError)?;

    // pop the double value from the stack and store it in the local variable at the index
    let value = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(index as usize, value);

    Ok(())
}

fn dstore_0(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 0 is a double else throw InstructionsError::InternalError
    thread.local_vars.get::<f64>(0).ok_or(InstructionError::InternalError)?;

    // pop the double value from the stack and store it in the local variable at index 0
    let value = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(0, value);

    Ok(())
}

fn dstore_1(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 1 is a double else throw InstructionsError::InternalError
    thread.local_vars.get::<f64>(1).ok_or(InstructionError::InternalError)?;

    // pop the double value from the stack and store it in the local variable at index 1
    let value = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(1, value);

    Ok(())
}

fn dstore_2(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 2 is a double else throw InstructionsError::InternalError
    thread.local_vars.get::<f64>(2).ok_or(InstructionError::InternalError)?;

    // pop the double value from the stack and store it in the local variable at index 2
    let value = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(2, value);

    Ok(())
}

fn dstore_3(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 3 is a double else throw InstructionsError::InternalError
    thread.local_vars.get::<f64>(3).ok_or(InstructionError::InternalError)?;

    // pop the double value from the stack and store it in the local variable at index 3
    let value = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(3, value);

    Ok(())
}

fn dsub(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<f64>().ok_or(InstructionError::InternalError)?;

    // subtract the two double values
    // push the result onto the stack
    thread.thread_stack.push(value1 - value2);

    Ok(())
}

fn dup(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    // push the value back onto the stack twice
    thread.thread_stack.push(value);
    thread.thread_stack.push(value);

    Ok(())
}

fn dup_x1(thread: &mut Thread) -> Result<()> {
    let value1 = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;
    let value2 = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    // push value1 back onto the stack, then push value2, then push value1 again
    thread.thread_stack.push(value1);
    thread.thread_stack.push(value2);
    thread.thread_stack.push(value1);

    Ok(())
}

fn dup_x2(thread: &mut Thread) -> Result<()> {
    let value1 = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;
    let value2 = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;
    let value3 = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    // push value1 back onto the stack, then push value3, then push value2, then push value1 again
    thread.thread_stack.push(value1);
    thread.thread_stack.push(value3);
    thread.thread_stack.push(value2);
    thread.thread_stack.push(value1);

    Ok(())
}

fn dup2(thread: &mut Thread) -> Result<()> {
    let value1 = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;
    let value2 = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    // push value2 back onto the stack, then push value1, then push value2 again, then push value1 again
    thread.thread_stack.push(value2);
    thread.thread_stack.push(value1);
    thread.thread_stack.push(value2);
    thread.thread_stack.push(value1);

    Ok(())
}

fn dup2_x1(thread: &mut Thread) -> Result<()> {
    let value1 = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;
    let value2 = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;
    let value3 = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    // push value2 back onto the stack, then push value1, then push value3, then push value2 again, then push value1 again
    thread.thread_stack.push(value2);
    thread.thread_stack.push(value1);
    thread.thread_stack.push(value3);
    thread.thread_stack.push(value2);
    thread.thread_stack.push(value1);

    Ok(())
}

fn dup2_x2(thread: &mut Thread) -> Result<()> {
    let value1 = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;
    let value2 = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;
    let value3 = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;
    let value4 = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    // push value2 back onto the stack, then push value1, then push value4, then push value3, then push value2 again, then push value1 again
    thread.thread_stack.push(value2);
    thread.thread_stack.push(value1);
    thread.thread_stack.push(value4);
    thread.thread_stack.push(value3);
    thread.thread_stack.push(value2);
    thread.thread_stack.push(value1);

    Ok(())
}

fn f2d(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;

    // convert the float value to a double value
    // push the double value onto the stack
    thread.thread_stack.push(value as f64);

    Ok(())
}

fn f2i(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;

    // convert the float value to an int value
    // push the int value onto the stack
    thread.thread_stack.push(value as i32);

    Ok(())
}

fn f2l(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;

    // convert the float value to a long value
    // push the long value onto the stack
    thread.thread_stack.push(value as i64);

    Ok(())
}

fn fadd(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;

    // add the two float values
    // push the result onto the stack
    thread.thread_stack.push(value1 + value2);

    Ok(())
}

fn faload(thread: &mut Thread) -> Result<()> {
    let index = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the array referenced
    let lock = REFERENCE_TABLE.lock().map_err(|_| InstructionError::InternalError)?;
    let array = lock.get_reference(&array_ref).ok_or(InstructionError::InternalError)?;

    // ensure this is an array
    if !array.is_array() {
        return Err(InstructionError::InternalError.into());
    }

    let array = array.as_array().ok_or(InstructionError::InternalError)?;

    // ensure the array is of type float
    if !matches!(array, AnyArrayValue::F32(_)){
        return Err(InstructionError::InternalError.into());
    }

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException
    if index < 0 || index >= array.len() as i32 {
        return Err(InstructionError::ArrayIndexOutOfBoundsException.into());
    }

    // push the value at the index onto the stack
    let value: f32 = *array.get(index as usize).ok_or(InstructionError::InternalError)?.downcast_ref::<f32>().ok_or(InstructionError::InternalError)?;
    thread.thread_stack.push(value);

    Ok(())
}

fn fastore(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;
    let index = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the array referenced
    let mut lock = REFERENCE_TABLE.lock().map_err(|_| InstructionError::InternalError)?;
    let array = lock.get_mut_reference(&array_ref).ok_or(InstructionError::InternalError)?;

    // ensure this is an array
    if !array.is_array() {
        return Err(InstructionError::InternalError.into());
    }

    let array: &mut AnyArrayValue = array.as_mut_array().ok_or(InstructionError::InternalError)?;

    // ensure the array is of type float
    if !matches!(array, AnyArrayValue::F32(_)){
        return Err(InstructionError::InternalError.into());
    }

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException
    if index < 0 || index >= array.len() as i32 {
        return Err(InstructionError::ArrayIndexOutOfBoundsException.into());
    }

    // set the value at the index to the value popped from the stack
    array.set(index as usize, Box::new(value)).map_err(|_| InstructionError::InternalError)?;

    Ok(())
}

fn fcmpg(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;

    // compare the two float values
    // push the result onto the stack
    if value1.is_nan() || value2.is_nan() || value1 > value2 {
        thread.thread_stack.push(1);
        thread.thread_stack.push(1);
    } else if value1 == value2 {
        thread.thread_stack.push(0);
    } else {
        thread.thread_stack.push(-1);
    }

    Ok(())
}

fn fcmpl(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;

    // compare the two float values
    // push the result onto the stack
    if value1.is_nan() || value2.is_nan() {
        thread.thread_stack.push(-1);
    } else if value1 > value2 {
        thread.thread_stack.push(1);
    } else if value1 == value2 {
        thread.thread_stack.push(0);
    } else {
        thread.thread_stack.push(-1);
    }

    Ok(())
}

fn fconst_0(thread: &mut Thread) -> Result<()> {
    thread.thread_stack.push::<f32>(0.0);

    Ok(())
}

fn fconst_1(thread: &mut Thread) -> Result<()> {
    thread.thread_stack.push::<f32>(1.0);

    Ok(())
}

fn fconst_2(thread: &mut Thread) -> Result<()> {
    thread.thread_stack.push::<f32>(2.0);

    Ok(())
}

fn fdiv(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;

    // divide the two float values
    // push the result onto the stack
    thread.thread_stack.push(value1 / value2);

    Ok(())
}

fn fload(thread: &mut Thread) -> Result<()> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError
    if index >= thread.local_vars.byte_length() as u16 {
        return Err(InstructionError::InternalError.into());
    }

    // ensure the local variable at the index is a float else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<f32>(index as usize).ok_or(InstructionError::InternalError)?;

    // push the float value from the local variable at the index onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn fload_0(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 0 is a float else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<f32>(0).ok_or(InstructionError::InternalError)?;

    // push the float value from the local variable at index 0 onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn fload_1(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 1 is a float else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<f32>(1).ok_or(InstructionError::InternalError)?;

    // push the float value from the local variable at index 1 onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn fload_2(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 2 is a float else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<f32>(2).ok_or(InstructionError::InternalError)?;

    // push the float value from the local variable at index 2 onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn fload_3(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 3 is a float else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<f32>(3).ok_or(InstructionError::InternalError)?;

    // push the float value from the local variable at index 3 onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn fmul(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;

    // multiply the two float values
    // push the result onto the stack
    thread.thread_stack.push(value1 * value2);

    Ok(())
}

fn fneg(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;

    // negate the float value
    // push the result onto the stack
    thread.thread_stack.push(-value);

    Ok(())
}

fn frem(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;

    // compute the remainder of the division of the two float values
    // push the result onto the stack
    if value2 == 0.0 {
        return Err(InstructionError::ArithmeticException.into());
    }

    thread.thread_stack.push(value1 % value2);

    Ok(())
}

fn freturn(thread: &mut Thread) -> Result<()> {

    todo!("Implement freturn instruction");

}

fn fstore(thread: &mut Thread) -> Result<()> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError
    if index >= thread.local_vars.byte_length() as u16 {
        return Err(InstructionError::InternalError.into());
    }

    // ensure the local variable at the index is a float else throw InstructionsError::InternalError
    thread.local_vars.get::<f32>(index as usize).ok_or(InstructionError::InternalError)?;

    // pop the float value from the stack and store it in the local variable at the index
    let value = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(index as usize, value);

    Ok(())
}

fn fstore_0(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 0 is a float else throw InstructionsError::InternalError
    thread.local_vars.get::<f32>(0).ok_or(InstructionError::InternalError)?;

    // pop the float value from the stack and store it in the local variable at index 0
    let value = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(0, value);

    Ok(())
}

fn fstore_1(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 1 is a float else throw InstructionsError::InternalError
    thread.local_vars.get::<f32>(1).ok_or(InstructionError::InternalError)?;

    // pop the float value from the stack and store it in the local variable at index 1
    let value = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(1, value);

    Ok(())
}

fn fstore_2(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 2 is a float else throw InstructionsError::InternalError
    thread.local_vars.get::<f32>(2).ok_or(InstructionError::InternalError)?;

    // pop the float value from the stack and store it in the local variable at index 2
    let value = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(2, value);

    Ok(())
}

fn fstore_3(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 3 is a float else throw InstructionsError::InternalError
    thread.local_vars.get::<f32>(3).ok_or(InstructionError::InternalError)?;

    // pop the float value from the stack and store it in the local variable at index 3
    let value = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(3, value);

    Ok(())
}

fn fsub(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<f32>().ok_or(InstructionError::InternalError)?;

    // subtract the two float values
    // push the result onto the stack
    thread.thread_stack.push(value1 - value2);

    Ok(())
}

fn getfield(thread: &mut Thread) -> Result<()> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a field reference else throw InstructionsError::InternalError

    // get the field referenced

    // ensure the object reference on top of the stack is not null else throw InstructionsError::NullPointerException

    // get the value of the field from the object reference on top of the stack

    // push the value onto the stack

    todo!("Implement getfield instruction");
}

fn getstatic(thread: &mut Thread) -> Result<()> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a field reference else throw InstructionsError::InternalError

    // get the field referenced

    // get the value of the static field

    // push the value onto the stack

    todo!("Implement getstatic instruction");
}

fn goto(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;

    // jump to the instruction at the offset from the current instruction
    thread.pc = offset as usize;

    Ok(())
}

fn goto_w(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_int_from_pc().ok_or(InstructionError::InternalError)? as i32;

    // jump to the instruction at the offset from the current instruction
    thread.pc = offset as usize;

    Ok(())
}

fn i2b(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // convert the int value to a byte value
    // push the byte value onto the stack
    thread.thread_stack.push((value as i8) as i32);

    Ok(())
}

fn i2c(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // convert the int value to a char value
    // push the char value onto the stack
    thread.thread_stack.push((value as u16) as i32);

    Ok(())
}

fn i2d(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // convert the int value to a double value
    // push the double value onto the stack
    thread.thread_stack.push(value as f64);

    Ok(())
}

fn i2f(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // convert the int value to a float value
    // push the float value onto the stack
    thread.thread_stack.push(value as f32);

    Ok(())
}

fn i2l(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // convert the int value to a long value
    // push the long value onto the stack
    thread.thread_stack.push(value as i64);

    Ok(())
}

fn i2s(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // convert the int value to a short value
    // push the short value onto the stack
    thread.thread_stack.push((value as i16) as i32);

    Ok(())
}

fn iadd(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // add the two int values
    // push the result onto the stack
    thread.thread_stack.push(value1.wrapping_add(value2));

    Ok(())
}

fn iaload(thread: &mut Thread) -> Result<()> {
    let index = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the array referenced
    let lock = REFERENCE_TABLE.lock().map_err(|_| InstructionError::InternalError)?;
    let array = lock.get_reference(&array_ref).ok_or(InstructionError::InternalError)?;

    // ensure this is an array
    if !array.is_array() {
        return Err(InstructionError::InternalError.into());
    }

    let array = array.as_array().ok_or(InstructionError::InternalError)?;

    // ensure the array is of type int
    if !matches!(array, AnyArrayValue::I32(_)){
        return Err(InstructionError::InternalError.into());
    }

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException
    if index < 0 || index >= array.len() as i32 {
        return Err(InstructionError::ArrayIndexOutOfBoundsException.into());
    }

    // push the value at the index onto the stack
    let value: i32 = *array.get(index as usize).ok_or(InstructionError::InternalError)?.downcast_ref::<i32>().ok_or(InstructionError::InternalError)?;
    thread.thread_stack.push(value);

    Ok(())
}

fn iand(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // perform a bitwise AND operation on the two int values
    // push the result onto the stack
    thread.thread_stack.push(value1 & value2);

    Ok(())
}

fn iastore(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let index = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the array referenced
    let mut lock = REFERENCE_TABLE.lock().map_err(|_| InstructionError::InternalError)?;
    let array = lock.get_mut_reference(&array_ref).ok_or(InstructionError::InternalError)?;

    // ensure this is an array
    if !array.is_array() {
        return Err(InstructionError::InternalError.into());
    }

    let array: &mut AnyArrayValue = array.as_mut_array().ok_or(InstructionError::InternalError)?;

    // ensure the array is of type int
    if !matches!(array, AnyArrayValue::I32(_)){
        return Err(InstructionError::InternalError.into());
    }

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException
    if index < 0 || index >= array.len() as i32 {
        return Err(InstructionError::ArrayIndexOutOfBoundsException.into());
    }

    // set the value at the index to the value popped from the stack
    array.set(index as usize, Box::new(value)).map_err(|_| InstructionError::InternalError)?;

    Ok(())
}

fn iconst_m1(thread: &mut Thread) -> Result<()> {
    thread.thread_stack.push::<i32>(-1);

    Ok(())
}

fn iconst_0(thread: &mut Thread) -> Result<()> {
    thread.thread_stack.push::<i32>(0);

    Ok(())
}

fn iconst_1(thread: &mut Thread) -> Result<()> {
    thread.thread_stack.push::<i32>(1);

    Ok(())
}

fn iconst_2(thread: &mut Thread) -> Result<()> {
    thread.thread_stack.push::<i32>(2);

    Ok(())
}

fn iconst_3(thread: &mut Thread) -> Result<()> {
    thread.thread_stack.push::<i32>(3);

    Ok(())
}

fn iconst_4(thread: &mut Thread) -> Result<()> {
    thread.thread_stack.push::<i32>(4);

    Ok(())
}

fn iconst_5(thread: &mut Thread) -> Result<()> {
    thread.thread_stack.push::<i32>(5);

    Ok(())
}

fn idiv(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    if value2 == 0 {
        return Err(InstructionError::ArithmeticException.into());
    }

    // divide the two int values
    // push the result onto the stack
    thread.thread_stack.push(value1 / value2);

    Ok(())
}

fn if_acmpeq(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value2 = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    // if the two object references are equal, jump to the instruction at the offset from the current instruction
    if value1 == value2 {
        thread.pc = thread.pc + (offset as usize) - 3; // subtract 3 for the opcode and the two bytes of the offset
    }

    Ok(())
}

fn if_acmpne(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value2 = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    // if the two object references are not equal, jump to the instruction at the offset from the current instruction
    if value1 != value2 {
        thread.pc = thread.pc + (offset as usize) - 3; // subtract 3 for the opcode and the two bytes of the offset
    }

    Ok(())
}

fn if_icmpeq(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // if the two int values are equal, jump to the instruction at the offset from the current instruction
    if value1 == value2 {
        thread.pc = thread.pc + (offset as usize) - 3; // subtract 3 for the opcode and the two bytes of the offset
    }

    Ok(())
}

fn if_icmpne(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // if the two int values are not equal, jump to the instruction at the offset from the current instruction
    if value1 != value2 {
        thread.pc = thread.pc + (offset as usize) - 3; // subtract 3 for the opcode and the two bytes of the offset
    }

    Ok(())
}

fn if_icmplt(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // if value1 is less than value2, jump to the instruction at the offset from the current instruction
    if value1 < value2 {
        thread.pc = thread.pc + (offset as usize) - 3; // subtract 3 for the opcode and the two bytes of the offset
    }

    Ok(())
}

fn if_icmpge(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // if value1 is greater than or equal to value2, jump to the instruction at the offset from the current instruction
    if value1 >= value2 {
        thread.pc = thread.pc + (offset as usize) - 3; // subtract 3 for the opcode and the two bytes of the offset
    }

    Ok(())
}

fn if_icmpgt(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // if value1 is greater than value2, jump to the instruction at the offset from the current instruction
    if value1 > value2 {
        thread.pc = thread.pc + (offset as usize) - 3; // subtract 3 for the opcode and the two bytes of the offset
    }

    Ok(())
}

fn if_icmple(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // if value1 is less than or equal to value2, jump to the instruction at the offset from the current instruction
    if value1 <= value2 {
        thread.pc = thread.pc + (offset as usize) - 3; // subtract 3 for the opcode and the two bytes of the offset
    }

    Ok(())
}

fn ifeq(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // if the int value is equal to 0, jump to the instruction at the offset from the current instruction
    if value == 0 {
        thread.pc = thread.pc + (offset as usize) - 3; // subtract 3 for the opcode and the two bytes of the offset
    }

    Ok(())
}

fn ifne(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // if the int value is not equal to 0, jump to the instruction at the offset from the current instruction
    if value != 0 {
        thread.pc = thread.pc + (offset as usize) - 3; // subtract 3 for the opcode and the two bytes of the offset
    }

    Ok(())
}

fn iflt(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // if the int value is less than 0, jump to the instruction at the offset from the current instruction
    if value < 0 {
        thread.pc = thread.pc + (offset as usize) - 3; // subtract 3 for the opcode and the two bytes of the offset
    }

    Ok(())
}

fn ifge(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // if the int value is greater than or equal to 0, jump to the instruction at the offset from the current instruction
    if value >= 0 {
        thread.pc = thread.pc + (offset as usize) - 3; // subtract 3 for the opcode and the two bytes of the offset
    }

    Ok(())
}

fn ifgt(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // if the int value is greater than 0, jump to the instruction at the offset from the current instruction
    if value > 0 {
        thread.pc = thread.pc + (offset as usize) - 3; // subtract 3 for the opcode and the two bytes of the offset
    }

    Ok(())
}

fn ifle(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // if the int value is less than or equal to 0, jump to the instruction at the offset from the current instruction
    if value <= 0 {
        thread.pc = thread.pc + (offset as usize) - 3; // subtract 3 for the opcode and the two bytes of the offset
    }

    Ok(())
}

fn ifnonnull(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    // if the object reference is not null, jump to the instruction at the offset from the current instruction
    if value.is_not_null() {
        thread.pc = thread.pc + (offset as usize) - 3; // subtract 3 for the opcode and the two bytes of the offset
    }

    Ok(())
}

fn ifnull(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    // if the object reference is null, jump to the instruction at the offset from the current instruction
    if value.is_null() {
        thread.pc = thread.pc + (offset as usize) - 3; // subtract 3 for the opcode and the two bytes of the offset
    }

    Ok(())
}

fn iinc(thread: &mut Thread) -> Result<()> {
    let index = thread.read_byte_from_pc().ok_or(InstructionError::InternalError)? as u16;
    let constant = thread.read_byte_from_pc().ok_or(InstructionError::InternalError)? as i8;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError
    if index >= thread.local_vars.byte_length() as u16 {
        return Err(InstructionError::InternalError.into());
    }

    // ensure the local variable at the index is an int else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<i32>(index as usize).ok_or(InstructionError::InternalError)?;

    // increment the int value in the local variable at the index by the constant
    let new_value = value.wrapping_add(constant as i32);
    thread.local_vars.set(index as usize, new_value);

    Ok(())
}

fn iload(thread: &mut Thread) -> Result<()> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError
    if index >= thread.local_vars.byte_length() as u16 {
        return Err(InstructionError::InternalError.into());
    }

    // ensure the local variable at the index is an int else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<i32>(index as usize).ok_or(InstructionError::InternalError)?;

    // push the int value from the local variable at the index onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn iload_0(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 0 is an int else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<i32>(0).ok_or(InstructionError::InternalError)?;

    // push the int value from the local variable at index 0 onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn iload_1(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 1 is an int else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<i32>(1).ok_or(InstructionError::InternalError)?;

    // push the int value from the local variable at index 1 onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn iload_2(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 2 is an int else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<i32>(2).ok_or(InstructionError::InternalError)?;

    // push the int value from the local variable at index 2 onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn iload_3(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 3 is an int else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<i32>(3).ok_or(InstructionError::InternalError)?;

    // push the int value from the local variable at index 3 onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn imul(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // multiply the two int values
    // push the result onto the stack
    thread.thread_stack.push(value1.wrapping_mul(value2));

    Ok(())
}

fn ineg(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // negate the int value
    // push the result onto the stack
    thread.thread_stack.push(value.wrapping_neg());

    Ok(())
}

fn instanceof(thread: &mut Thread) -> Result<()> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a class reference else throw InstructionsError::InternalError

    // get the class referenced

    // ensure the object reference on top of the stack is not null else throw InstructionsError::NullPointerException

    // check if the object reference on top of the stack is an instance of the class referenced

    // push 1 onto the stack if it is an instance, otherwise push 0

    todo!("Implement instanceof instruction");
}

fn invokedynamic(thread: &mut Thread) -> Result<()> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a method reference else throw InstructionsError::InternalError

    // get the method referenced

    // invoke the method dynamically

    todo!("Implement invokedynamic instruction");
}

fn invokeinterface(thread: &mut Thread) -> Result<()> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;
    let count = thread.read_byte_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a method reference else throw InstructionsError::InternalError

    // get the method referenced

    // ensure the object reference on top of the stack is not null else throw InstructionsError::NullPointerException

    // invoke the method on the object reference on top of the stack

    todo!("Implement invokeinterface instruction");
}

fn invokespecial(thread: &mut Thread) -> Result<()> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a method reference else throw InstructionsError::InternalError

    // get the method referenced

    // ensure the object reference on top of the stack is not null else throw InstructionsError::NullPointerException

    // invoke the method on the object reference on top of the stack

    todo!("Implement invokespecial instruction");
}

fn invokestatic(thread: &mut Thread) -> Result<()> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a method reference else throw InstructionsError::InternalError

    // get the method referenced

    // invoke the static method

    todo!("Implement invokestatic instruction");
}

fn invokevirtual(thread: &mut Thread) -> Result<()> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a method reference else throw InstructionsError::InternalError

    // get the method referenced

    // ensure the object reference on top of the stack is not null else throw InstructionsError::NullPointerException

    // invoke the method on the object reference on top of the stack

    todo!("Implement invokevirtual instruction");
}

fn ior(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // perform a bitwise OR operation on the two int values
    // push the result onto the stack
    thread.thread_stack.push(value1 | value2);

    Ok(())
}

fn irem(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    if value2 == 0 {
        return Err(InstructionError::ArithmeticException.into());
    }

    // compute the remainder of the division of the two int values
    // push the result onto the stack
    thread.thread_stack.push(value1 % value2);

    Ok(())
}

fn ireturn(thread: &mut Thread) -> Result<()> {

    todo!("Implement ireturn instruction");

}

fn ishl(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // shift the first int value to the left by the number of bits specified by the second int value
    // push the result onto the stack
    thread.thread_stack.push(value1.wrapping_shl(value2 as u32));

    Ok(())
}

fn ishr(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // shift the first int value to the right by the number of bits specified by the second int value, using sign extension
    // push the result onto the stack
    thread.thread_stack.push(value1.wrapping_shr(value2 as u32));

    Ok(())
}

fn istore(thread: &mut Thread) -> Result<()> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError
    if index >= thread.local_vars.byte_length() as u16 {
        return Err(InstructionError::InternalError.into());
    }

    // ensure the local variable at the index is an int else throw InstructionsError::InternalError
    thread.local_vars.get::<i32>(index as usize).ok_or(InstructionError::InternalError)?;

    // pop the int value from the stack and store it in the local variable at the index
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(index as usize, value);

    Ok(())
}

fn istore_0(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 0 is an int else throw InstructionsError::InternalError
    thread.local_vars.get::<i32>(0).ok_or(InstructionError::InternalError)?;

    // pop the int value from the stack and store it in the local variable at index 0
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(0, value);

    Ok(())
}

fn istore_1(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 1 is an int else throw InstructionsError::InternalError
    thread.local_vars.get::<i32>(1).ok_or(InstructionError::InternalError)?;

    // pop the int value from the stack and store it in the local variable at index 1
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(1, value);

    Ok(())
}

fn istore_2(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 2 is an int else throw InstructionsError::InternalError
    thread.local_vars.get::<i32>(2).ok_or(InstructionError::InternalError)?;

    // pop the int value from the stack and store it in the local variable at index 2
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(2, value);

    Ok(())
}

fn istore_3(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 3 is an int else throw InstructionsError::InternalError
    thread.local_vars.get::<i32>(3).ok_or(InstructionError::InternalError)?;

    // pop the int value from the stack and store it in the local variable at index 3
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(3, value);

    Ok(())
}

fn isub(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // subtract the two int values
    // push the result onto the stack
    thread.thread_stack.push(value1.wrapping_sub(value2));

    Ok(())
}

fn iushr(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // shift the first int value to the right by the number of bits specified by the second int value, using zero extension
    // push the result onto the stack
    thread.thread_stack.push(((value1 as u32) >> value2) as i32);

    Ok(())
}

fn ixor(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    // perform a bitwise XOR operation on the two int values
    // push the result onto the stack
    thread.thread_stack.push(value1 ^ value2);

    Ok(())
}

fn jsr(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;

    // push the address of the next instruction onto the stack
    thread.thread_stack.push::<i32>(thread.pc as i32);

    // jump to the instruction at the offset from the current instruction
    thread.pc = offset as usize;

    Ok(())
}

fn jsr_w(thread: &mut Thread) -> Result<()> {
    let offset = thread.read_int_from_pc().ok_or(InstructionError::InternalError)? as i32;

    // push the address of the next instruction onto the stack
    thread.thread_stack.push::<i32>(thread.pc as i32);

    // jump to the instruction at the offset from the current instruction
    thread.pc = offset as usize;

    Ok(())
}

fn l2d(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;

    // convert the long value to a double value
    // push the double value onto the stack
    thread.thread_stack.push(value as f64);

    Ok(())
}

fn l2f(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;

    // convert the long value to a float value
    // push the float value onto the stack
    thread.thread_stack.push(value as f32);

    Ok(())
}

fn l2i(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;

    // convert the long value to an int value
    // push the int value onto the stack
    thread.thread_stack.push(value as i32);

    Ok(())
}

fn ladd(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;

    // add the two long values
    // push the result onto the stack
    thread.thread_stack.push(value1.wrapping_add(value2));

    Ok(())
}

fn laload(thread: &mut Thread) -> Result<()> {
    let index = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the array referenced
    let lock = REFERENCE_TABLE.lock().map_err(|_| InstructionError::InternalError)?;
    let array = lock.get_reference(&array_ref).ok_or(InstructionError::InternalError)?;

    // ensure this is an array
    if !array.is_array() {
        return Err(InstructionError::InternalError.into());
    }

    let array = array.as_array().ok_or(InstructionError::InternalError)?;

    // ensure the array is of type long
    if !matches!(array, AnyArrayValue::I64(_)){
        return Err(InstructionError::InternalError.into());
    }

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException
    if index < 0 || index >= array.len() as i32 {
        return Err(InstructionError::ArrayIndexOutOfBoundsException.into());
    }

    // push the value at the index onto the stack
    let value: i64 = *array.get(index as usize).ok_or(InstructionError::InternalError)?.downcast_ref::<i64>().ok_or(InstructionError::InternalError)?;
    thread.thread_stack.push(value);

    Ok(())
}

fn land(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;

    // perform a bitwise AND operation on the two long values
    // push the result onto the stack
    thread.thread_stack.push(value1 & value2);

    Ok(())
}

fn lastore(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;
    let index = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the array referenced
    let mut lock = REFERENCE_TABLE.lock().map_err(|_| InstructionError::InternalError)?;
    let array = lock.get_mut_reference(&array_ref).ok_or(InstructionError::InternalError)?;

    // ensure this is an array
    if !array.is_array() {
        return Err(InstructionError::InternalError.into());
    }

    let array: &mut AnyArrayValue = array.as_mut_array().ok_or(InstructionError::InternalError)?;

    // ensure the array is of type long
    if !matches!(array, AnyArrayValue::I64(_)){
        return Err(InstructionError::InternalError.into());
    }

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException
    if index < 0 || index >= array.len() as i32 {
        return Err(InstructionError::ArrayIndexOutOfBoundsException.into());
    }

    // set the value at the index to the value popped from the stack
    array.set(index as usize, Box::new(value)).map_err(|_| InstructionError::InternalError)?;

    Ok(())
}

fn lcmp(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;

    // compare the two long values
    // push 0 onto the stack if the values are equal, 1 if value1 is greater than value2, and -1 if value1 is less than value2
    if value1 == value2 {
        thread.thread_stack.push(0);
    } else if value1 > value2 {
        thread.thread_stack.push(1);
    } else {
        thread.thread_stack.push(-1);
    }

    Ok(())
}

fn lconst_0(thread: &mut Thread) -> Result<()> {
    thread.thread_stack.push::<i64>(0);

    Ok(())
}

fn lconst_1(thread: &mut Thread) -> Result<()> {
    thread.thread_stack.push::<i64>(1);

    Ok(())
}

fn ldc(thread: &mut Thread) -> Result<()> {
    let index = thread.read_byte_from_pc().ok_or(InstructionError::InternalError)? as u16;

    // ensure the constant pool entry at the index is a class reference, field reference, method reference, string reference, or integer constant else throw InstructionsError::InternalError

    // get the constant referenced

    // push the constant onto the stack

    todo!("Implement ldc instruction");
}

fn ldc_w(thread: &mut Thread) -> Result<()> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a class reference, field reference, method reference, string reference, or integer constant else throw InstructionsError::InternalError

    // get the constant referenced

    // push the constant onto the stack

    todo!("Implement ldc_w instruction");
}

fn ldc2_w(thread: &mut Thread) -> Result<()> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a long constant or double constant else throw InstructionsError::InternalError

    // get the constant referenced

    // push the constant onto the stack

    todo!("Implement ldc2_w instruction");
}

fn ldiv(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;

    if value2 == 0 {
        return Err(InstructionError::ArithmeticException.into());
    }

    // divide the two long values
    // push the result onto the stack
    thread.thread_stack.push(value1 / value2);

    Ok(())
}

fn lload(thread: &mut Thread) -> Result<()> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError
    if index >= thread.local_vars.byte_length() as u16 {
        return Err(InstructionError::InternalError.into());
    }

    // ensure the local variable at the index is a long else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<i64>(index as usize).ok_or(InstructionError::InternalError)?;

    // push the long value from the local variable at the index onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn lload_0(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 0 is a long else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<i64>(0).ok_or(InstructionError::InternalError)?;

    // push the long value from the local variable at index 0 onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn lload_1(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 1 is a long else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<i64>(1).ok_or(InstructionError::InternalError)?;

    // push the long value from the local variable at index 1 onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn lload_2(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 2 is a long else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<i64>(2).ok_or(InstructionError::InternalError)?;

    // push the long value from the local variable at index 2 onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn lload_3(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 3 is a long else throw InstructionsError::InternalError
    let value = thread.local_vars.get::<i64>(3).ok_or(InstructionError::InternalError)?;

    // push the long value from the local variable at index 3 onto the stack
    thread.thread_stack.push(value);

    Ok(())
}

fn lmul(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;

    // multiply the two long values
    // push the result onto the stack
    thread.thread_stack.push(value1 * value2);

    Ok(())
}

fn lneg(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;

    // negate the long value
    // push the result onto the stack
    thread.thread_stack.push(-value);

    Ok(())
}

fn lookupswitch(thread: &mut Thread) -> Result<()> {
    // skip padding bytes

    let default_offset = thread.read_int_from_pc().ok_or(InstructionError::InternalError)?;
    let npairs = thread.read_int_from_pc().ok_or(InstructionError::InternalError)?;

    // read the match-offset pairs

    // get the int value on top of the stack

    // search the match-offset pairs for a match with the int value

    // if a match is found, jump to the instruction at the offset from the current instruction specified by the match, otherwise jump to the instruction at the default offset from the current instruction

    todo!("Implement lookupswitch instruction");
}

fn lor(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;

    // perform a bitwise OR operation on the two long values
    // push the result onto the stack
    thread.thread_stack.push(value1 | value2);

    Ok(())
}

fn lrem(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;

    if value2 == 0 {
        return Err(InstructionError::ArithmeticException.into());
    }

    // compute the remainder of the division of the two long values
    // push the result onto the stack
    thread.thread_stack.push(value1 % value2);

    Ok(())
}

fn lreturn(thread: &mut Thread) -> Result<()> {

    todo!("Implement lreturn instruction");

}

fn lshl(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;

    // shift the long value to the left by the number of bits specified by the int value
    // push the result onto the stack
    thread.thread_stack.push(value1.wrapping_shl(value2 as u32));

    Ok(())
}

fn lshr(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;

    // shift the long value to the right by the number of bits specified by the int value, using sign extension
    // push the result onto the stack
    thread.thread_stack.push(value1.wrapping_shr(value2 as u32));

    Ok(())
}

fn lstore(thread: &mut Thread) -> Result<()> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError
    if index >= thread.local_vars.byte_length() as u16 {
        return Err(InstructionError::InternalError.into());
    }

    // ensure the local variable at the index is a long else throw InstructionsError::InternalError
    thread.local_vars.get::<i64>(index as usize).ok_or(InstructionError::InternalError)?;

    // pop the long value from the stack and store it in the local variable at the index
    let value = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(index as usize, value);

    Ok(())
}

fn lstore_0(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 0 is a long else throw InstructionsError::InternalError
    thread.local_vars.get::<i64>(0).ok_or(InstructionError::InternalError)?;

    // pop the long value from the stack and store it in the local variable at index 0
    let value = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(0, value);

    Ok(())
}

fn lstore_1(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 1 is a long else throw InstructionsError::InternalError
    thread.local_vars.get::<i64>(1).ok_or(InstructionError::InternalError)?;

    // pop the long value from the stack and store it in the local variable at index 1
    let value = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(1, value);

    Ok(())
}

fn lstore_2(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 2 is a long else throw InstructionsError::InternalError
    thread.local_vars.get::<i64>(2).ok_or(InstructionError::InternalError)?;

    // pop the long value from the stack and store it in the local variable at index 2
    let value = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(2, value);

    Ok(())
}

fn lstore_3(thread: &mut Thread) -> Result<()> {
    // ensure the local variable at index 3 is a long else throw InstructionsError::InternalError
    thread.local_vars.get::<i64>(3).ok_or(InstructionError::InternalError)?;

    // pop the long value from the stack and store it in the local variable at index 3
    let value = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;
    thread.local_vars.set(3, value);

    Ok(())
}

fn lsub(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;

    // subtract the two long values
    // push the result onto the stack
    thread.thread_stack.push(value1 - value2);

    Ok(())
}

fn lushr(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;

    // shift the long value to the right by the number of bits specified by the int value, using zero extension
    // push the result onto the stack
    thread.thread_stack.push(((value1 as u64) >> value2) as i64);

    Ok(())
}

fn lxor(thread: &mut Thread) -> Result<()> {
    let value2 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop::<i64>().ok_or(InstructionError::InternalError)?;

    // perform a bitwise XOR operation on the two long values
    // push the result onto the stack
    thread.thread_stack.push(value1 ^ value2);

    Ok(())
}

fn monitorenter(thread: &mut Thread) -> Result<()> {
    let object_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if object_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the object monitor for the object reference

    // enter the monitor, blocking if necessary until it is available

    todo!("Implement monitorenter instruction");
}

fn monitorexit(thread: &mut Thread) -> Result<()> {
    let object_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if object_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the object monitor for the object reference

    // exit the monitor

    todo!("Implement monitorexit instruction");
}

fn multianewarray(thread: &mut Thread) -> Result<()> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;
    let dimensions = thread.read_byte_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a class reference else throw InstructionsError::InternalError

    // get the class referenced

    // pop the dimension sizes from the stack

    // create a new multi-dimensional array with the specified dimensions and push a reference to it onto the stack

    todo!("Implement multianewarray instruction");
}

fn new(thread: &mut Thread) -> Result<()> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a class reference else throw InstructionsError::InternalError

    // get the class referenced

    // create a new instance of the class and push a reference to it onto the stack

    todo!("Implement new instruction");
}

fn newarray(thread: &mut Thread) -> Result<()> {
    let atype = thread.read_byte_from_pc().ok_or(InstructionError::InternalError)?;
    let count = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;

    if count < 0 {
        return Err(InstructionError::NegativeArraySizeException.into());
    }

    // create a new array of the specified type and length and push a reference to it onto the stack
    let array_value = create_array_from_char(atype as char, count as u16);
    let mut lock = REFERENCE_TABLE.lock().map_err(|_| InstructionError::InternalError)?;
    let array_ref = lock.add_reference(ReferenceValue::Array(array_value));
    thread.thread_stack.push(array_ref);

    Ok(())
}

fn nop(_: &mut Thread) -> Result<()> {
    Ok(())
}

fn pop(thread: &mut Thread) -> Result<()> {
    thread.thread_stack.pop::<i16>().ok_or(InstructionError::InternalError)?;

    Ok(())
}

fn pop2(thread: &mut Thread) -> Result<()> {
    thread.thread_stack.pop::<i16>().ok_or(InstructionError::InternalError)?;
    thread.thread_stack.pop::<i16>().ok_or(InstructionError::InternalError)?;

    Ok(())
}

fn putfield(thread: &mut Thread) -> Result<()> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a field reference else throw InstructionsError::InternalError

    // get the field referenced

    // ensure the object reference on top of the stack is not null else throw InstructionsError::NullPointerException

    // pop the value from the stack and set the field of the object reference on top of the stack to the value

    todo!("Implement putfield instruction");
}

fn putstatic(thread: &mut Thread) -> Result<()> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a field reference else throw InstructionsError::InternalError

    // get the field referenced

    // pop the value from the stack and set the static field to the value

    todo!("Implement putstatic instruction");
}

fn ret(thread: &mut Thread) -> Result<()> {
    let index = thread.read_byte_from_pc().ok_or(InstructionError::InternalError)? as u16;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError

    // ensure the local variable at the index is an int else throw InstructionsError::InternalError

    // return from the subroutine at the address specified by the local variable at the index

    todo!("Implement ret instruction");
}

fn return_(thread: &mut Thread) -> Result<()> {

    todo!("Implement return instruction");

}

fn saload(thread: &mut Thread) -> Result<()> {
    let index = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the array referenced
    let lock = REFERENCE_TABLE.lock().map_err(|_| InstructionError::InternalError)?;
    let array = lock.get_reference(&array_ref).ok_or(InstructionError::InternalError)?;

    // ensure this is an array
    if !array.is_array() {
        return Err(InstructionError::InternalError.into());
    }

    let array = array.as_array().ok_or(InstructionError::InternalError)?;

    // ensure the array is of type short
    if !matches!(array, AnyArrayValue::I16(_)){
        return Err(InstructionError::InternalError.into());
    }

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException
    if index < 0 || index >= array.len() as i32 {
        return Err(InstructionError::ArrayIndexOutOfBoundsException.into());
    }

    // push the value at the index onto the stack
    let value: i16 = *array.get(index as usize).ok_or(InstructionError::InternalError)?.downcast_ref::<i16>().ok_or(InstructionError::InternalError)?;
    thread.thread_stack.push(value);

    Ok(())
}

fn sastore(thread: &mut Thread) -> Result<()> {
    let value = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let index = thread.thread_stack.pop::<i32>().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop::<Reference>().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException.into());
    }

    // get the array referenced
    let mut lock = REFERENCE_TABLE.lock().map_err(|_| InstructionError::InternalError)?;
    let array = lock.get_mut_reference(&array_ref).ok_or(InstructionError::InternalError)?;

    // ensure this is an array
    if !array.is_array() {
        return Err(InstructionError::InternalError.into());
    }

    let array: &mut AnyArrayValue = array.as_mut_array().ok_or(InstructionError::InternalError)?;

    // ensure the array is of type short
    if !matches!(array, AnyArrayValue::I16(_)){
        return Err(InstructionError::InternalError.into());
    }

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException
    if index < 0 || index >= array.len() as i32 {
        return Err(InstructionError::ArrayIndexOutOfBoundsException.into());
    }

    // set the value at the index to the value popped from the stack
    array.set(index as usize, Box::new(value as i16)).map_err(|_| InstructionError::InternalError)?;

    Ok(())
}

fn sipush(thread: &mut Thread) -> Result<()> {
    let value = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;

    // push the short value onto the stack as an int
    thread.thread_stack.push(value as i32);

    Ok(())
}

fn swap(thread: &mut Thread) -> Result<()> {
    let value1 = thread.thread_stack.pop::<i16>().ok_or(InstructionError::InternalError)?;
    let value2 = thread.thread_stack.pop::<i16>().ok_or(InstructionError::InternalError)?;

    // push the values back onto the stack in reverse order
    thread.thread_stack.push::<i16>(value1);
    thread.thread_stack.push::<i16>(value2);

    Ok(())
}

fn tableswitch(thread: &mut Thread) -> Result<()> {
    // skip padding bytes

    let default_offset = thread.read_int_from_pc().ok_or(InstructionError::InternalError)?;
    let low = thread.read_int_from_pc().ok_or(InstructionError::InternalError)?;
    let high = thread.read_int_from_pc().ok_or(InstructionError::InternalError)?;

    // read the jump offsets

    // get the int value on top of the stack

    // if the int value is between low and high, jump to the instruction at the offset from the current instruction specified by the int value, otherwise jump to the instruction at the default offset from the current instruction

    todo!("Implement tableswitch instruction");
}

fn wide(thread: &mut Thread) -> Result<()> {
    thread.wide_mode = true;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::thread::{JVMHeap, LocalVars, MethodArea, NativeMethodStack, Thread, ThreadStack};
    use std::sync::{Arc, Mutex};

    /// Thread with no bytecode and 16 local-variable slots (zero-filled).
    fn make_thread() -> Thread {
        Thread {
            pc: 0,
            code: Vec::new(),
            local_vars: LocalVars::new(16),
            thread_stack: ThreadStack::new(),
            jvm_heap: Arc::new(Mutex::new(JVMHeap {})),
            method_area: Arc::new(Mutex::new(MethodArea {})),
            native_method_stack: NativeMethodStack {},
            wide_mode: false,
        }
    }

    /// Thread whose operand bytes begin at code[0] with pc=0.
    /// Use for instructions that consume bytes from the stream (bipush, sipush,
    /// iload, etc.).
    fn make_thread_with_code(code: Vec<u8>) -> Thread {
        Thread {
            pc: 0,
            code,
            local_vars: LocalVars::new(16),
            thread_stack: ThreadStack::new(),
            jvm_heap: Arc::new(Mutex::new(JVMHeap {})),
            method_area: Arc::new(Mutex::new(MethodArea {})),
            native_method_stack: NativeMethodStack {},
            wide_mode: false,
        }
    }

    /// Thread with explicit `pc` — used for branch-instruction tests.
    ///
    /// Convention: set `pc = 1` so that the opcode sits at address 0 (already
    /// consumed by the dispatch loop).  Operand bytes follow at code[1..].
    ///
    ///   branch target = opcode_address + branchoffset
    ///                 = (pc_at_entry – 1) + branchoffset
    ///
    /// With pc=1: target = 0 + branchoffset = branchoffset.
    fn make_thread_at_pc(pc: usize, code: Vec<u8>) -> Thread {
        Thread {
            pc,
            code,
            local_vars: LocalVars::new(16),
            thread_stack: ThreadStack::new(),
            jvm_heap: Arc::new(Mutex::new(JVMHeap {})),
            method_area: Arc::new(Mutex::new(MethodArea {})),
            native_method_stack: NativeMethodStack {},
            wide_mode: false,
        }
    }

    // ── 0x00  nop ─────────────────────────────────────────────────────────────

    #[test]
    fn test_nop_returns_ok_and_leaves_stack_empty() {
        let mut t = make_thread();
        assert!(nop(&mut t).is_ok());
        assert!(t.thread_stack.pop::<i32>().is_none());
    }

    // ── 0x01  aconst_null ─────────────────────────────────────────────────────

    #[test]
    fn test_aconst_null_pushes_null_reference() {
        let mut t = make_thread();
        assert!(aconst_null(&mut t).is_ok());
        let r = t.thread_stack.pop::<Reference>().unwrap();
        assert!(r.is_null());
    }

    // ── 0x02–0x08  iconst ─────────────────────────────────────────────────────

    #[test]
    fn test_iconst_m1_pushes_negative_one() {
        let mut t = make_thread();
        assert!(iconst_m1(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), -1);
    }

    #[test]
    fn test_iconst_0_pushes_zero() {
        let mut t = make_thread();
        assert!(iconst_0(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 0);
    }

    #[test]
    fn test_iconst_1_pushes_one() {
        let mut t = make_thread();
        assert!(iconst_1(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 1);
    }

    #[test]
    fn test_iconst_2_pushes_two() {
        let mut t = make_thread();
        assert!(iconst_2(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 2);
    }

    #[test]
    fn test_iconst_3_pushes_three() {
        let mut t = make_thread();
        assert!(iconst_3(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 3);
    }

    #[test]
    fn test_iconst_4_pushes_four() {
        let mut t = make_thread();
        assert!(iconst_4(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 4);
    }

    #[test]
    fn test_iconst_5_pushes_five() {
        let mut t = make_thread();
        assert!(iconst_5(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 5);
    }

    // ── 0x09–0x0a  lconst ─────────────────────────────────────────────────────

    #[test]
    fn test_lconst_0_pushes_zero_long() {
        let mut t = make_thread();
        assert!(lconst_0(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 0i64);
    }

    #[test]
    fn test_lconst_1_pushes_one_long() {
        let mut t = make_thread();
        assert!(lconst_1(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 1i64);
    }

    // ── 0x0b–0x0d  fconst ─────────────────────────────────────────────────────

    #[test]
    fn test_fconst_0_pushes_zero_float() {
        let mut t = make_thread();
        assert!(fconst_0(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f32>().unwrap(), 0.0f32);
    }

    #[test]
    fn test_fconst_1_pushes_one_float() {
        let mut t = make_thread();
        assert!(fconst_1(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f32>().unwrap(), 1.0f32);
    }

    #[test]
    fn test_fconst_2_pushes_two_float() {
        let mut t = make_thread();
        assert!(fconst_2(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f32>().unwrap(), 2.0f32);
    }

    // ── 0x0e–0x0f  dconst ─────────────────────────────────────────────────────

    #[test]
    fn test_dconst_0_pushes_zero_double() {
        let mut t = make_thread();
        assert!(dconst_0(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f64>().unwrap(), 0.0f64);
    }

    #[test]
    fn test_dconst_1_pushes_one_double() {
        let mut t = make_thread();
        assert!(dconst_1(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f64>().unwrap(), 1.0f64);
    }

    // ── 0x10  bipush ──────────────────────────────────────────────────────────
    // Reads one signed byte from the bytecode stream and pushes it sign-extended
    // to int on the operand stack.

    #[test]
    fn test_bipush_positive_value() {
        let mut t = make_thread_with_code(vec![42]);
        assert!(bipush(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 42);
    }

    #[test]
    fn test_bipush_sign_extends_negative_byte() {
        // 0xFF = -1 as signed byte → -1 as i32
        let mut t = make_thread_with_code(vec![0xFF]);
        assert!(bipush(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), -1);
    }

    #[test]
    fn test_bipush_min_signed_byte() {
        // 0x80 = -128 as signed byte
        let mut t = make_thread_with_code(vec![0x80]);
        assert!(bipush(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), -128);
    }

    #[test]
    fn test_bipush_max_signed_byte() {
        // 0x7F = 127 as signed byte
        let mut t = make_thread_with_code(vec![0x7F]);
        assert!(bipush(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 127);
    }

    #[test]
    fn test_bipush_no_operand_returns_internal_error() {
        let mut t = make_thread_with_code(vec![]); // no bytes to read
        assert!(matches!(
            bipush(&mut t).unwrap_err().downcast_ref::<InstructionError>(),
            Some(InstructionError::InternalError)
        ));
    }

    // ── 0x11  sipush ──────────────────────────────────────────────────────────
    // Reads a big-endian signed 16-bit value and pushes it sign-extended to int.

    #[test]
    fn test_sipush_positive_value() {
        // [0x01, 0x00] = 256 big-endian
        let mut t = make_thread_with_code(vec![0x01, 0x00]);
        assert!(sipush(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 256);
    }

    #[test]
    fn test_sipush_sign_extends_negative_short() {
        // [0xFF, 0x00] = 0xFF00 = -256 as i16
        let mut t = make_thread_with_code(vec![0xFF, 0x00]);
        assert!(sipush(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), -256);
    }

    #[test]
    fn test_sipush_max_short() {
        // [0x7F, 0xFF] = 32767
        let mut t = make_thread_with_code(vec![0x7F, 0xFF]);
        assert!(sipush(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 32767);
    }

    #[test]
    fn test_sipush_min_short() {
        // [0x80, 0x00] = -32768
        let mut t = make_thread_with_code(vec![0x80, 0x00]);
        assert!(sipush(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), -32768);
    }

    // ── 0x12–0x14  ldc / ldc_w / ldc2_w ──────────────────────────────────────
    // These instructions require constant-pool infrastructure (MethodArea) that
    // is not yet implemented.  Full tests will be added once the constant-pool
    // lookup is available.  For now they assert that calling the instruction
    // without proper infrastructure panics (todo!).

    #[test]
    #[should_panic]
    fn test_ldc_requires_constant_pool() {
        // TODO: once constant pool is available, test that ldc pushes an int,
        // float, or String reference from the pool entry at the 1-byte index.
        let _ = ldc(&mut make_thread_with_code(vec![0]));
    }

    #[test]
    #[should_panic]
    fn test_ldc_w_requires_constant_pool() {
        // TODO: same as ldc but with a 2-byte (wide) index.
        let _ = ldc_w(&mut make_thread_with_code(vec![0x00, 0x00]));
    }

    #[test]
    #[should_panic]
    fn test_ldc2_w_requires_constant_pool() {
        // TODO: pushes a long or double from the pool; uses a 2-byte index.
        let _ = ldc2_w(&mut make_thread_with_code(vec![0x00, 0x00]));
    }

    // ── 0x15–0x19  generic loads ──────────────────────────────────────────────
    // Each instruction reads an index from the bytecode (1 byte normally,
    // 2 bytes in wide mode) then pushes the corresponding local variable.

    #[test]
    fn test_iload_pushes_int_from_local_var() {
        let mut t = make_thread_with_code(vec![2]); // index = 2
        t.local_vars.set::<i32>(2, 42);
        assert!(iload(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 42);
    }

    #[test]
    fn test_iload_wide_mode_uses_two_byte_index() {
        let mut t = make_thread_with_code(vec![0x00, 0x03]); // index = 3
        t.wide_mode = true;
        t.local_vars.set::<i32>(3, 99);
        assert!(iload(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 99);
    }

    #[test]
    fn test_lload_pushes_long_from_local_var() {
        let mut t = make_thread_with_code(vec![1]); // index = 1
        t.local_vars.set::<i64>(1, 123456789012345i64);
        assert!(lload(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 123456789012345i64);
    }

    #[test]
    fn test_fload_pushes_float_from_local_var() {
        let mut t = make_thread_with_code(vec![0]); // index = 0
        t.local_vars.set::<f32>(0, 3.14f32);
        assert!(fload(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f32>().unwrap(), 3.14f32);
    }

    #[test]
    fn test_dload_pushes_double_from_local_var() {
        let mut t = make_thread_with_code(vec![0]); // index = 0
        t.local_vars.set::<f64>(0, 2.718f64);
        assert!(dload(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f64>().unwrap(), 2.718f64);
    }

    #[test]
    fn test_aload_pushes_reference_from_local_var() {
        let mut t = make_thread_with_code(vec![1]); // index = 1
        t.local_vars.set::<Reference>(1, Reference::new(7));
        assert!(aload(&mut t).is_ok());
        let r = t.thread_stack.pop::<Reference>().unwrap();
        assert_eq!(r.get_ref_index(), 7);
    }

    // ── 0x1a–0x2d  indexed loads ──────────────────────────────────────────────
    // iload_N: push int from local variable slot N onto the operand stack.

    #[test]
    fn test_iload_0_pushes_int_from_slot_0() {
        let mut t = make_thread();
        t.local_vars.set::<i32>(0, 10);
        assert!(iload_0(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 10);
    }
    #[test]
    fn test_iload_1_pushes_int_from_slot_1() {
        let mut t = make_thread();
        t.local_vars.set::<i32>(1, 20);
        assert!(iload_1(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 20);
    }
    #[test]
    fn test_iload_2_pushes_int_from_slot_2() {
        let mut t = make_thread();
        t.local_vars.set::<i32>(2, 30);
        assert!(iload_2(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 30);
    }
    #[test]
    fn test_iload_3_pushes_int_from_slot_3() {
        let mut t = make_thread();
        t.local_vars.set::<i32>(3, 40);
        assert!(iload_3(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 40);
    }

    // lload_N: push long from local variable slot N.
    #[test]
    fn test_lload_0_pushes_long_from_slot_0() {
        let mut t = make_thread();
        t.local_vars.set::<i64>(0, 100i64);
        assert!(lload_0(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 100i64);
    }
    #[test]
    fn test_lload_1_pushes_long_from_slot_1() {
        let mut t = make_thread();
        t.local_vars.set::<i64>(1, 200i64);
        assert!(lload_1(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 200i64);
    }
    #[test]
    fn test_lload_2_pushes_long_from_slot_2() {
        let mut t = make_thread();
        t.local_vars.set::<i64>(2, 300i64);
        assert!(lload_2(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 300i64);
    }
    #[test]
    fn test_lload_3_pushes_long_from_slot_3() {
        let mut t = make_thread();
        t.local_vars.set::<i64>(3, 400i64);
        assert!(lload_3(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 400i64);
    }

    // fload_N: push float from local variable slot N.
    #[test]
    fn test_fload_0_pushes_float_from_slot_0() {
        let mut t = make_thread();
        t.local_vars.set::<f32>(0, 1.5f32);
        assert!(fload_0(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f32>().unwrap(), 1.5f32);
    }
    #[test]
    fn test_fload_1_pushes_float_from_slot_1() {
        let mut t = make_thread();
        t.local_vars.set::<f32>(1, 2.5f32);
        assert!(fload_1(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f32>().unwrap(), 2.5f32);
    }
    #[test]
    fn test_fload_2_pushes_float_from_slot_2() {
        let mut t = make_thread();
        t.local_vars.set::<f32>(2, 3.5f32);
        assert!(fload_2(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f32>().unwrap(), 3.5f32);
    }
    #[test]
    fn test_fload_3_pushes_float_from_slot_3() {
        let mut t = make_thread();
        t.local_vars.set::<f32>(3, 4.5f32);
        assert!(fload_3(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f32>().unwrap(), 4.5f32);
    }

    // dload_N: push double from local variable slot N.
    #[test]
    fn test_dload_0_pushes_double_from_slot_0() {
        let mut t = make_thread();
        t.local_vars.set::<f64>(0, 1.1f64);
        assert!(dload_0(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f64>().unwrap(), 1.1f64);
    }
    #[test]
    fn test_dload_1_pushes_double_from_slot_1() {
        let mut t = make_thread();
        t.local_vars.set::<f64>(1, 2.2f64);
        assert!(dload_1(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f64>().unwrap(), 2.2f64);
    }
    #[test]
    fn test_dload_2_pushes_double_from_slot_2() {
        let mut t = make_thread();
        t.local_vars.set::<f64>(2, 3.3f64);
        assert!(dload_2(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f64>().unwrap(), 3.3f64);
    }
    #[test]
    fn test_dload_3_pushes_double_from_slot_3() {
        let mut t = make_thread();
        t.local_vars.set::<f64>(3, 4.4f64);
        assert!(dload_3(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f64>().unwrap(), 4.4f64);
    }

    // aload_N: push reference from local variable slot N.
    #[test]
    fn test_aload_0_pushes_reference_from_slot_0() {
        let mut t = make_thread();
        t.local_vars.set::<Reference>(0, Reference::new(5));
        assert!(aload_0(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<Reference>().unwrap().get_ref_index(), 5);
    }
    #[test]
    fn test_aload_1_pushes_reference_from_slot_1() {
        let mut t = make_thread();
        t.local_vars.set::<Reference>(1, Reference::new(6));
        assert!(aload_1(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<Reference>().unwrap().get_ref_index(), 6);
    }
    #[test]
    fn test_aload_2_pushes_reference_from_slot_2() {
        let mut t = make_thread();
        t.local_vars.set::<Reference>(2, Reference::new(7));
        assert!(aload_2(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<Reference>().unwrap().get_ref_index(), 7);
    }
    #[test]
    fn test_aload_3_pushes_reference_from_slot_3() {
        let mut t = make_thread();
        t.local_vars.set::<Reference>(3, Reference::new(8));
        assert!(aload_3(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<Reference>().unwrap().get_ref_index(), 8);
    }

    // ── 0x2e–0x35  array loads ────────────────────────────────────────────────

    #[test]
    fn test_iaload_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        t.thread_stack.push::<i32>(0);
        assert!(matches!(iaload(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    #[test]
    fn test_laload_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        t.thread_stack.push::<i32>(0);
        assert!(matches!(laload(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    #[test]
    fn test_faload_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        t.thread_stack.push::<i32>(0);
        assert!(matches!(faload(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    #[test]
    fn test_daload_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        t.thread_stack.push::<i32>(0);
        assert!(matches!(daload(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    #[test]
    fn test_aaload_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        t.thread_stack.push::<i32>(0);
        assert!(matches!(aaload(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    #[test]
    fn test_baload_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        t.thread_stack.push::<i32>(0);
        assert!(matches!(baload(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    #[test]
    fn test_caload_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        t.thread_stack.push::<i32>(0);
        assert!(matches!(caload(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    #[test]
    fn test_saload_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        t.thread_stack.push::<i32>(0);
        assert!(matches!(saload(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    // ── 0x36–0x3a  generic stores ─────────────────────────────────────────────
    // Each instruction reads a slot index from the bytecode stream then pops
    // a value from the operand stack and stores it in that local-variable slot.

    #[test]
    fn test_istore_pops_int_into_indexed_local_var() {
        let mut t = make_thread_with_code(vec![2]); // index = 2
        t.thread_stack.push::<i32>(55);
        assert!(istore(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<i32>(2).unwrap(), 55);
        assert!(t.thread_stack.pop::<i32>().is_none()); // stack drained
    }

    #[test]
    fn test_lstore_pops_long_into_indexed_local_var() {
        let mut t = make_thread_with_code(vec![1]); // index = 1
        t.thread_stack.push::<i64>(9876543210i64);
        assert!(lstore(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<i64>(1).unwrap(), 9876543210i64);
    }

    #[test]
    fn test_fstore_pops_float_into_indexed_local_var() {
        let mut t = make_thread_with_code(vec![0]); // index = 0
        t.thread_stack.push::<f32>(7.77f32);
        assert!(fstore(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<f32>(0).unwrap(), 7.77f32);
    }

    #[test]
    fn test_dstore_pops_double_into_indexed_local_var() {
        let mut t = make_thread_with_code(vec![0]); // index = 0
        t.thread_stack.push::<f64>(3.14159f64);
        assert!(dstore(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<f64>(0).unwrap(), 3.14159f64);
    }

    #[test]
    fn test_astore_pops_reference_into_indexed_local_var() {
        let mut t = make_thread_with_code(vec![3]); // index = 3
        t.thread_stack.push::<Reference>(Reference::new(9));
        assert!(astore(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<Reference>(3).unwrap().get_ref_index(), 9);
    }

    // ── 0x3b–0x4e  indexed stores ─────────────────────────────────────────────
    // istore_N: pop int and store in local variable slot N.

    #[test]
    fn test_istore_0_stores_int_into_slot_0() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(11);
        assert!(istore_0(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<i32>(0).unwrap(), 11);
        assert!(t.thread_stack.pop::<i32>().is_none());
    }
    #[test]
    fn test_istore_1_stores_int_into_slot_1() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(22);
        assert!(istore_1(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<i32>(1).unwrap(), 22);
    }
    #[test]
    fn test_istore_2_stores_int_into_slot_2() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(33);
        assert!(istore_2(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<i32>(2).unwrap(), 33);
    }
    #[test]
    fn test_istore_3_stores_int_into_slot_3() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(44);
        assert!(istore_3(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<i32>(3).unwrap(), 44);
    }

    // lstore_N: pop long and store in local variable slot N.
    #[test]
    fn test_lstore_0_stores_long_into_slot_0() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(111i64);
        assert!(lstore_0(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<i64>(0).unwrap(), 111i64);
    }
    #[test]
    fn test_lstore_1_stores_long_into_slot_1() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(222i64);
        assert!(lstore_1(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<i64>(1).unwrap(), 222i64);
    }
    #[test]
    fn test_lstore_2_stores_long_into_slot_2() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(333i64);
        assert!(lstore_2(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<i64>(2).unwrap(), 333i64);
    }
    #[test]
    fn test_lstore_3_stores_long_into_slot_3() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(444i64);
        assert!(lstore_3(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<i64>(3).unwrap(), 444i64);
    }

    // fstore_N: pop float and store in local variable slot N.
    #[test]
    fn test_fstore_0_stores_float_into_slot_0() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(1.1f32);
        assert!(fstore_0(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<f32>(0).unwrap(), 1.1f32);
    }
    #[test]
    fn test_fstore_1_stores_float_into_slot_1() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(2.2f32);
        assert!(fstore_1(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<f32>(1).unwrap(), 2.2f32);
    }
    #[test]
    fn test_fstore_2_stores_float_into_slot_2() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(3.3f32);
        assert!(fstore_2(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<f32>(2).unwrap(), 3.3f32);
    }
    #[test]
    fn test_fstore_3_stores_float_into_slot_3() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(4.4f32);
        assert!(fstore_3(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<f32>(3).unwrap(), 4.4f32);
    }

    // dstore_N: pop double and store in local variable slot N.
    #[test]
    fn test_dstore_0_stores_double_into_slot_0() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(0.1f64);
        assert!(dstore_0(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<f64>(0).unwrap(), 0.1f64);
    }
    #[test]
    fn test_dstore_1_stores_double_into_slot_1() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(0.2f64);
        assert!(dstore_1(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<f64>(1).unwrap(), 0.2f64);
    }
    #[test]
    fn test_dstore_2_stores_double_into_slot_2() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(0.3f64);
        assert!(dstore_2(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<f64>(2).unwrap(), 0.3f64);
    }
    #[test]
    fn test_dstore_3_stores_double_into_slot_3() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(0.4f64);
        assert!(dstore_3(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<f64>(3).unwrap(), 0.4f64);
    }

    // astore_N: pop reference and store in local variable slot N.
    #[test]
    fn test_astore_0_stores_reference_into_slot_0() {
        let mut t = make_thread();
        t.thread_stack.push::<Reference>(Reference::new(10));
        assert!(astore_0(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<Reference>(0).unwrap().get_ref_index(), 10);
    }
    #[test]
    fn test_astore_1_stores_reference_into_slot_1() {
        let mut t = make_thread();
        t.thread_stack.push::<Reference>(Reference::new(11));
        assert!(astore_1(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<Reference>(1).unwrap().get_ref_index(), 11);
    }
    #[test]
    fn test_astore_2_stores_reference_into_slot_2() {
        let mut t = make_thread();
        t.thread_stack.push::<Reference>(Reference::new(12));
        assert!(astore_2(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<Reference>(2).unwrap().get_ref_index(), 12);
    }
    #[test]
    fn test_astore_3_stores_reference_into_slot_3() {
        let mut t = make_thread();
        t.thread_stack.push::<Reference>(Reference::new(13));
        assert!(astore_3(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<Reference>(3).unwrap().get_ref_index(), 13);
    }

    // ── 0x4f–0x56  array stores ───────────────────────────────────────────────

    #[test]
    fn test_iastore_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        t.thread_stack.push::<i32>(0);
        t.thread_stack.push::<i32>(42);
        assert!(matches!(iastore(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    #[test]
    fn test_lastore_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        t.thread_stack.push::<i32>(0);
        t.thread_stack.push::<i64>(42);
        assert!(matches!(lastore(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    #[test]
    fn test_fastore_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        t.thread_stack.push::<i32>(0);
        t.thread_stack.push::<f32>(1.0);
        assert!(matches!(fastore(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    #[test]
    fn test_dastore_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        t.thread_stack.push::<i32>(0);
        t.thread_stack.push::<f64>(1.0);
        assert!(matches!(dastore(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    #[test]
    fn test_aastore_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        t.thread_stack.push::<i32>(0);
        t.thread_stack.push(Reference::new(5));
        assert!(matches!(aastore(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    #[test]
    fn test_bastore_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        t.thread_stack.push::<i32>(0);
        t.thread_stack.push::<i32>(1);
        assert!(matches!(bastore(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    #[test]
    fn test_castore_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        t.thread_stack.push::<i32>(0);
        t.thread_stack.push::<i32>(65);
        assert!(matches!(castore(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    #[test]
    fn test_sastore_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        t.thread_stack.push::<i32>(0);
        t.thread_stack.push::<i32>(100);
        assert!(matches!(sastore(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    // ── 0x57  pop ─────────────────────────────────────────────────────────────

    #[test]
    fn test_pop_removes_top_short() {
        let mut t = make_thread();
        t.thread_stack.push::<i16>(42);
        assert!(pop(&mut t).is_ok());
        assert!(t.thread_stack.pop::<i16>().is_none());
    }

    #[test]
    fn test_pop_empty_stack_returns_internal_error() {
        let mut t = make_thread();
        assert!(matches!(pop(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::InternalError)));
    }

    // ── 0x58  pop2 ────────────────────────────────────────────────────────────

    #[test]
    fn test_pop2_removes_two_shorts() {
        let mut t = make_thread();
        t.thread_stack.push::<i16>(1);
        t.thread_stack.push::<i16>(2);
        assert!(pop2(&mut t).is_ok());
        assert!(t.thread_stack.pop::<i16>().is_none());
    }

    #[test]
    fn test_pop2_empty_stack_returns_internal_error() {
        let mut t = make_thread();
        assert!(matches!(pop2(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::InternalError)));
    }

    // ── 0x59–0x5e  dup variants ───────────────────────────────────────────────
    // All dup variants operate on single-slot (category 1) values.
    // ThreadStack is raw bytes, so i32 and Reference are both 4 bytes and
    // interchangeable at the byte level in these tests.
    //
    // JVM spec stack notation (bottom → top):

    // dup: ..., v → ..., v, v
    #[test]
    fn test_dup_duplicates_top_value() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(42);
        assert!(dup(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 42);
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 42);
        assert!(t.thread_stack.pop::<i32>().is_none());
    }

    #[test]
    fn test_dup_empty_stack_returns_internal_error() {
        let mut t = make_thread();
        assert!(matches!(
            dup(&mut t).unwrap_err().downcast_ref::<InstructionError>(),
            Some(InstructionError::InternalError)
        ));
    }

    // dup_x1: ..., v2, v1 → ..., v1, v2, v1
    #[test]
    fn test_dup_x1_inserts_copy_below_second_value() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(2); // v2
        t.thread_stack.push::<i32>(1); // v1 (top)
        assert!(dup_x1(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 1); // top
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 2);
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 1); // deepest copy
        assert!(t.thread_stack.pop::<i32>().is_none());
    }

    // dup_x2 (form 1: three category-1 values): ..., v3, v2, v1 → ..., v1, v3, v2, v1
    #[test]
    fn test_dup_x2_inserts_copy_below_third_value() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(3); // v3
        t.thread_stack.push::<i32>(2); // v2
        t.thread_stack.push::<i32>(1); // v1 (top)
        assert!(dup_x2(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 1);
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 2);
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 3);
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 1);
        assert!(t.thread_stack.pop::<i32>().is_none());
    }

    // dup2 (form 1: two category-1 values): ..., v2, v1 → ..., v2, v1, v2, v1
    #[test]
    fn test_dup2_duplicates_top_two_values() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(2); // v2
        t.thread_stack.push::<i32>(1); // v1 (top)
        assert!(dup2(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 1);
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 2);
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 1);
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 2);
        assert!(t.thread_stack.pop::<i32>().is_none());
    }

    // dup2_x1 (form 1): ..., v3, v2, v1 → ..., v2, v1, v3, v2, v1
    #[test]
    fn test_dup2_x1_inserts_two_copies_below_third_value() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(3); // v3
        t.thread_stack.push::<i32>(2); // v2
        t.thread_stack.push::<i32>(1); // v1 (top)
        assert!(dup2_x1(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 1);
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 2);
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 3);
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 1);
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 2);
        assert!(t.thread_stack.pop::<i32>().is_none());
    }

    // dup2_x2 (form 1): ..., v4, v3, v2, v1 → ..., v2, v1, v4, v3, v2, v1
    #[test]
    fn test_dup2_x2_inserts_two_copies_below_fourth_value() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(4); // v4
        t.thread_stack.push::<i32>(3); // v3
        t.thread_stack.push::<i32>(2); // v2
        t.thread_stack.push::<i32>(1); // v1 (top)
        assert!(dup2_x2(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 1);
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 2);
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 3);
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 4);
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 1);
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 2);
        assert!(t.thread_stack.pop::<i32>().is_none());
    }

    // ── 0x5f  swap ────────────────────────────────────────────────────────────

    #[test]
    fn test_swap_reverses_top_two_shorts() {
        let mut t = make_thread();
        t.thread_stack.push::<i16>(10);
        t.thread_stack.push::<i16>(20);
        assert!(swap(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i16>().unwrap(), 10);
        assert_eq!(t.thread_stack.pop::<i16>().unwrap(), 20);
    }

    #[test]
    fn test_swap_empty_stack_returns_internal_error() {
        let mut t = make_thread();
        assert!(matches!(swap(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::InternalError)));
    }

    // ── 0x60–0x63  add ────────────────────────────────────────────────────────
    // JVM integer arithmetic is modulo 2^32/2^64 (wrapping).
    // Float/double follow IEEE 754.

    #[test]
    fn test_iadd_adds_two_ints() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(3); // value1
        t.thread_stack.push::<i32>(4); // value2
        assert!(iadd(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 7);
    }
    #[test]
    fn test_iadd_wraps_on_overflow() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(i32::MAX);
        t.thread_stack.push::<i32>(1);
        assert!(iadd(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), i32::MIN);
    }

    #[test]
    fn test_ladd_adds_two_longs() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(100i64);
        t.thread_stack.push::<i64>(200i64);
        assert!(ladd(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 300i64);
    }
    #[test]
    fn test_ladd_wraps_on_overflow() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(i64::MAX);
        t.thread_stack.push::<i64>(1);
        assert!(ladd(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), i64::MIN);
    }

    #[test]
    fn test_fadd_adds_two_floats() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(1.5f32);
        t.thread_stack.push::<f32>(2.5f32);
        assert!(fadd(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f32>().unwrap(), 4.0f32);
    }

    #[test]
    fn test_dadd_adds_two_doubles() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(1.5f64);
        t.thread_stack.push::<f64>(2.5f64);
        assert!(dadd(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f64>().unwrap(), 4.0f64);
    }

    // ── 0x64–0x67  sub ────────────────────────────────────────────────────────
    // Result = value1 − value2  (value2 is on top of stack).

    #[test]
    fn test_isub_subtracts_top_from_second() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(10); // value1
        t.thread_stack.push::<i32>(3);  // value2 (top)
        assert!(isub(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 7);
    }
    #[test]
    fn test_isub_wraps_on_underflow() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(i32::MIN);
        t.thread_stack.push::<i32>(1);
        assert!(isub(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), i32::MAX);
    }

    #[test]
    fn test_lsub_subtracts_longs() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(50i64);
        t.thread_stack.push::<i64>(20i64);
        assert!(lsub(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 30i64);
    }

    #[test]
    fn test_fsub_subtracts_floats() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(5.5f32);
        t.thread_stack.push::<f32>(2.5f32);
        assert!(fsub(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f32>().unwrap(), 3.0f32);
    }

    #[test]
    fn test_dsub_subtracts_doubles() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(5.5f64);
        t.thread_stack.push::<f64>(2.5f64);
        assert!(dsub(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f64>().unwrap(), 3.0f64);
    }

    // ── 0x68–0x6b  mul ────────────────────────────────────────────────────────

    #[test]
    fn test_imul_multiplies_two_ints() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(6);
        t.thread_stack.push::<i32>(7);
        assert!(imul(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 42);
    }
    #[test]
    fn test_imul_wraps_on_overflow() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(i32::MAX);
        t.thread_stack.push::<i32>(2);
        assert!(imul(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), -2);
    }

    #[test]
    fn test_lmul_multiplies_two_longs() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(1_000_000i64);
        t.thread_stack.push::<i64>(1_000_000i64);
        assert!(lmul(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 1_000_000_000_000i64);
    }

    #[test]
    fn test_fmul_multiplies_two_floats() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(3.0f32);
        t.thread_stack.push::<f32>(4.0f32);
        assert!(fmul(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f32>().unwrap(), 12.0f32);
    }

    #[test]
    fn test_dmul_multiplies_two_doubles() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(2.5f64);
        t.thread_stack.push::<f64>(4.0f64);
        assert!(dmul(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f64>().unwrap(), 10.0f64);
    }

    // ── 0x6c–0x6f  div ────────────────────────────────────────────────────────
    // Integer division truncates toward zero and throws ArithmeticException on
    // divide-by-zero.  Float/double follow IEEE 754 (no exception on /0).

    #[test]
    fn test_idiv_by_zero_returns_arithmetic_exception() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(10);
        t.thread_stack.push::<i32>(0);
        assert!(matches!(idiv(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::ArithmeticException)));
    }

    #[test]
    fn test_idiv_truncates_toward_zero() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(7);  // value1
        t.thread_stack.push::<i32>(2);  // value2
        assert!(idiv(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 3);
    }

    #[test]
    fn test_idiv_negative_truncates_toward_zero() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(-7);
        t.thread_stack.push::<i32>(2);
        assert!(idiv(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), -3);
    }

    #[test]
    fn test_ldiv_by_zero_returns_arithmetic_exception() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(10);
        t.thread_stack.push::<i64>(0);
        assert!(matches!(ldiv(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::ArithmeticException)));
    }

    #[test]
    fn test_ldiv_divides_longs() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(100i64);
        t.thread_stack.push::<i64>(4i64);
        assert!(ldiv(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 25i64);
    }

    #[test]
    fn test_fdiv_divides_floats() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(10.0f32);
        t.thread_stack.push::<f32>(4.0f32);
        assert!(fdiv(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f32>().unwrap(), 2.5f32);
    }

    #[test]
    fn test_fdiv_by_zero_produces_infinity() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(1.0f32);
        t.thread_stack.push::<f32>(0.0f32);
        assert!(fdiv(&mut t).is_ok());
        assert!(t.thread_stack.pop::<f32>().unwrap().is_infinite());
    }

    #[test]
    fn test_ddiv_divides_doubles() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(10.0f64);
        t.thread_stack.push::<f64>(4.0f64);
        assert!(ddiv(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f64>().unwrap(), 2.5f64);
    }

    // ── 0x70–0x73  rem ────────────────────────────────────────────────────────
    // Integer remainder: result sign matches the dividend.

    #[test]
    fn test_irem_by_zero_returns_arithmetic_exception() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(10);
        t.thread_stack.push::<i32>(0);
        assert!(matches!(irem(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::ArithmeticException)));
    }

    #[test]
    fn test_irem_positive_remainder() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(10);
        t.thread_stack.push::<i32>(3);
        assert!(irem(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 1);
    }

    #[test]
    fn test_irem_negative_dividend_sign_matches_dividend() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(-10);
        t.thread_stack.push::<i32>(3);
        assert!(irem(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), -1);
    }

    #[test]
    fn test_lrem_by_zero_returns_arithmetic_exception() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(10);
        t.thread_stack.push::<i64>(0);
        assert!(matches!(lrem(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::ArithmeticException)));
    }

    #[test]
    fn test_lrem_positive_remainder() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(17i64);
        t.thread_stack.push::<i64>(5i64);
        assert!(lrem(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 2i64);
    }

    #[test]
    fn test_frem_float_remainder() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(10.0f32);
        t.thread_stack.push::<f32>(3.0f32);
        assert!(frem(&mut t).is_ok());
        let r = t.thread_stack.pop::<f32>().unwrap();
        assert!((r - 1.0f32).abs() < 1e-6);
    }

    #[test]
    fn test_drem_double_remainder() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(10.0f64);
        t.thread_stack.push::<f64>(3.0f64);
        assert!(drem(&mut t).is_ok());
        let r = t.thread_stack.pop::<f64>().unwrap();
        assert!((r - 1.0f64).abs() < 1e-10);
    }

    // ── 0x74–0x77  neg ────────────────────────────────────────────────────────
    // Arithmetic negation.  Integer MIN_VALUE negated wraps to MIN_VALUE.

    #[test]
    fn test_ineg_negates_positive_int() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(5);
        assert!(ineg(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), -5);
    }
    #[test]
    fn test_ineg_min_value_wraps() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(i32::MIN);
        assert!(ineg(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), i32::MIN);
    }

    #[test]
    fn test_lneg_negates_positive_long() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(42i64);
        assert!(lneg(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), -42i64);
    }

    #[test]
    fn test_fneg_negates_float() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(3.0f32);
        assert!(fneg(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f32>().unwrap(), -3.0f32);
    }

    #[test]
    fn test_dneg_negates_double() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(2.5f64);
        assert!(dneg(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f64>().unwrap(), -2.5f64);
    }

    // ── 0x78–0x7d  shifts ─────────────────────────────────────────────────────
    // JVM spec: shift count is masked — int uses low 5 bits, long uses low 6 bits.
    // value1 is the value to shift; value2 (always int) is the shift amount.

    // ishl: result = value1 << (value2 & 0x1f)
    #[test]
    fn test_ishl_shifts_int_left() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(1);  // value1
        t.thread_stack.push::<i32>(3);  // value2 (shift amount)
        assert!(ishl(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 8); // 1 << 3
    }
    #[test]
    fn test_ishl_masks_shift_count_to_5_bits() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(1);
        t.thread_stack.push::<i32>(33); // 33 & 0x1f = 1
        assert!(ishl(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 2); // 1 << 1
    }

    // lshl: result = value1 << (value2 & 0x3f)  (value1 = long, value2 = int)
    #[test]
    fn test_lshl_shifts_long_left() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(1i64); // value1 (long)
        t.thread_stack.push::<i32>(4);    // value2 (int shift amount)
        assert!(lshl(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 16i64); // 1 << 4
    }
    #[test]
    fn test_lshl_masks_shift_count_to_6_bits() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(1i64);
        t.thread_stack.push::<i32>(65); // 65 & 0x3f = 1
        assert!(lshl(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 2i64); // 1 << 1
    }

    // ishr: arithmetic (sign-extending) right shift, value1 >> (value2 & 0x1f)
    #[test]
    fn test_ishr_arithmetic_right_shift() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(16); // value1
        t.thread_stack.push::<i32>(2);  // value2
        assert!(ishr(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 4); // 16 >> 2
    }
    #[test]
    fn test_ishr_sign_extends_negative_value() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(-16);
        t.thread_stack.push::<i32>(2);
        assert!(ishr(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), -4); // sign-preserving
    }

    // lshr: arithmetic right shift for long, value1 >> (value2 & 0x3f)
    #[test]
    fn test_lshr_arithmetic_right_shift_long() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(64i64); // value1 (long)
        t.thread_stack.push::<i32>(3);     // value2 (int)
        assert!(lshr(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 8i64); // 64 >> 3
    }
    #[test]
    fn test_lshr_sign_extends_negative_long() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(-64i64);
        t.thread_stack.push::<i32>(3);
        assert!(lshr(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), -8i64);
    }

    // iushr: logical (zero-filling) right shift, (value1 as u32 >> (value2 & 0x1f)) as i32
    #[test]
    fn test_iushr_logical_right_shift_positive() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(16);
        t.thread_stack.push::<i32>(2);
        assert!(iushr(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 4);
    }
    #[test]
    fn test_iushr_fills_with_zeros_for_negative_value() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(-1); // 0xFFFFFFFF
        t.thread_stack.push::<i32>(28);
        assert!(iushr(&mut t).is_ok());
        // 0xFFFF_FFFF >>> 28 = 0x0000_000F = 15 (zero-filled)
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 0x0F);
    }

    // lushr: logical right shift for long, (value1 as u64 >> (value2 & 0x3f)) as i64
    #[test]
    fn test_lushr_logical_right_shift_long() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(64i64);
        t.thread_stack.push::<i32>(3);
        assert!(lushr(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 8i64);
    }
    #[test]
    fn test_lushr_fills_with_zeros_for_negative_long() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(-1i64); // 0xFFFF_FFFF_FFFF_FFFF
        t.thread_stack.push::<i32>(60);
        assert!(lushr(&mut t).is_ok());
        // 0xFFFF...FFFF >>> 60 = 0x0F (zero-filled)
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 0x0Fi64);
    }

    // ── 0x7e–0x83  bitwise ────────────────────────────────────────────────────

    // iand: result = value1 & value2
    #[test]
    fn test_iand_bitwise_and_ints() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(0b1010); // value1
        t.thread_stack.push::<i32>(0b1100); // value2
        assert!(iand(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 0b1000);
    }

    // land: result = value1 & value2 (long)
    #[test]
    fn test_land_bitwise_and_longs() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(0b1010i64);
        t.thread_stack.push::<i64>(0b1100i64);
        assert!(land(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 0b1000i64);
    }

    // ior: result = value1 | value2
    #[test]
    fn test_ior_bitwise_or_ints() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(0b1010);
        t.thread_stack.push::<i32>(0b1100);
        assert!(ior(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 0b1110);
    }

    // lor: result = value1 | value2 (long)
    #[test]
    fn test_lor_bitwise_or_longs() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(0b1010i64);
        t.thread_stack.push::<i64>(0b1100i64);
        assert!(lor(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 0b1110i64);
    }

    // ixor: result = value1 ^ value2
    #[test]
    fn test_ixor_bitwise_xor_ints() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(0b1010);
        t.thread_stack.push::<i32>(0b1100);
        assert!(ixor(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 0b0110);
    }

    // lxor: result = value1 ^ value2 (long)
    #[test]
    fn test_lxor_bitwise_xor_longs() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(0b1010i64);
        t.thread_stack.push::<i64>(0b1100i64);
        assert!(lxor(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 0b0110i64);
    }

    // ── 0x84  iinc ────────────────────────────────────────────────────────────
    // Reads a 1-byte local-variable index then a 1-byte signed constant from the
    // bytecode stream.  Increments local_vars[index] by that constant.

    #[test]
    fn test_iinc_increments_local_var_by_positive_const() {
        let mut t = make_thread_with_code(vec![2, 5]); // index=2, const=+5
        t.local_vars.set::<i32>(2, 10);
        assert!(iinc(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<i32>(2).unwrap(), 15);
    }
    #[test]
    fn test_iinc_decrements_local_var_with_negative_const() {
        let mut t = make_thread_with_code(vec![0, 0xFF]); // index=0, const=-1 (signed byte)
        t.local_vars.set::<i32>(0, 100);
        assert!(iinc(&mut t).is_ok());
        assert_eq!(t.local_vars.get::<i32>(0).unwrap(), 99);
    }

    // ── 0x85–0x93  type conversions ───────────────────────────────────────────

    // i2l: int → long (sign-extending widening conversion)
    #[test]
    fn test_i2l_sign_extends_positive_int_to_long() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(42);
        assert!(i2l(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 42i64);
    }
    #[test]
    fn test_i2l_sign_extends_negative_int_to_long() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(-1);
        assert!(i2l(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), -1i64);
    }

    // i2f: int → float (widening; may lose precision for large values)
    #[test]
    fn test_i2f_converts_int_to_float() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(100);
        assert!(i2f(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f32>().unwrap(), 100.0f32);
    }

    // i2d: int → double (widening; always exact for 32-bit int)
    #[test]
    fn test_i2d_converts_int_to_double() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(100);
        assert!(i2d(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f64>().unwrap(), 100.0f64);
    }

    // l2i: long → int (truncates to low 32 bits)
    #[test]
    fn test_l2i_truncates_long_to_low_32_bits() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(0x1_0000_0005i64); // high bits are discarded
        assert!(l2i(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 5);
    }
    #[test]
    fn test_l2i_converts_simple_long_to_int() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(42i64);
        assert!(l2i(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 42);
    }

    // l2f: long → float (widening; may lose precision)
    #[test]
    fn test_l2f_converts_long_to_float() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(10i64);
        assert!(l2f(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f32>().unwrap(), 10.0f32);
    }

    // l2d: long → double (widening; may lose precision for very large values)
    #[test]
    fn test_l2d_converts_long_to_double() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(10i64);
        assert!(l2d(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f64>().unwrap(), 10.0f64);
    }

    // f2i: float → int  (truncate toward zero; NaN → 0; overflow clamps to MAX/MIN)
    #[test]
    fn test_f2i_truncates_float_toward_zero() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(3.9f32);
        assert!(f2i(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 3);
    }
    #[test]
    fn test_f2i_nan_becomes_zero() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(f32::NAN);
        assert!(f2i(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 0);
    }
    #[test]
    fn test_f2i_positive_infinity_clamps_to_max_int() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(f32::INFINITY);
        assert!(f2i(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), i32::MAX);
    }
    #[test]
    fn test_f2i_negative_infinity_clamps_to_min_int() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(f32::NEG_INFINITY);
        assert!(f2i(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), i32::MIN);
    }

    // f2l: float → long  (truncate toward zero; NaN → 0; overflow clamps)
    #[test]
    fn test_f2l_truncates_float_to_long() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(5.9f32);
        assert!(f2l(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 5i64);
    }
    #[test]
    fn test_f2l_nan_becomes_zero() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(f32::NAN);
        assert!(f2l(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 0i64);
    }

    // f2d: float → double (widening; always exact)
    #[test]
    fn test_f2d_widens_float_to_double() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(1.5f32);
        assert!(f2d(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f64>().unwrap(), 1.5f64);
    }

    // d2i: double → int  (truncate toward zero; NaN → 0; overflow clamps)
    #[test]
    fn test_d2i_truncates_double_toward_zero() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(7.9f64);
        assert!(d2i(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 7);
    }
    #[test]
    fn test_d2i_nan_becomes_zero() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(f64::NAN);
        assert!(d2i(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 0);
    }

    // d2l: double → long  (truncate toward zero; NaN → 0; overflow clamps)
    #[test]
    fn test_d2l_truncates_double_to_long() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(9.9f64);
        assert!(d2l(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 9i64);
    }
    #[test]
    fn test_d2l_nan_becomes_zero() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(f64::NAN);
        assert!(d2l(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i64>().unwrap(), 0i64);
    }

    // d2f: double → float (narrowing; may lose precision)
    #[test]
    fn test_d2f_narrows_double_to_float() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(1.5f64);
        assert!(d2f(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<f32>().unwrap(), 1.5f32);
    }

    // i2b: int → byte (sign-extend low 8 bits); result pushed as int
    #[test]
    fn test_i2b_sign_extends_to_byte() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(0x1FF); // low 8 bits = 0xFF = -1 as signed byte
        assert!(i2b(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), -1);
    }
    #[test]
    fn test_i2b_positive_byte_remains_positive() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(0x105); // low 8 bits = 5
        assert!(i2b(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 5);
    }

    // i2c: int → char (zero-extend low 16 bits); result pushed as int
    #[test]
    fn test_i2c_zero_extends_to_char_range() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(0x1_0041); // low 16 bits = 0x0041 = 65 ('A')
        assert!(i2c(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 65);
    }
    #[test]
    fn test_i2c_negative_int_zero_extends_low_16_bits() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(-1); // 0xFFFFFFFF; low 16 = 0xFFFF = 65535
        assert!(i2c(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 65535);
    }

    // i2s: int → short (sign-extend low 16 bits); result pushed as int
    #[test]
    fn test_i2s_sign_extends_to_short() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(0x1_8000); // low 16 bits = 0x8000 = -32768 as signed short
        assert!(i2s(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), -32768);
    }
    #[test]
    fn test_i2s_positive_short_remains_positive() {
        let mut t = make_thread();
        t.thread_stack.push::<i32>(0x1_0064); // low 16 bits = 100
        assert!(i2s(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 100);
    }

    // ── 0x94–0x98  comparisons ────────────────────────────────────────────────
    // Pushes int result: 0 if equal, 1 if value1 > value2, -1 if value1 < value2.
    // fcmpl/dcmpl: NaN comparison → -1
    // fcmpg/dcmpg: NaN comparison → +1

    // lcmp
    #[test]
    fn test_lcmp_equal_longs_push_zero() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(5i64); // value1
        t.thread_stack.push::<i64>(5i64); // value2
        assert!(lcmp(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 0);
    }
    #[test]
    fn test_lcmp_value1_greater_pushes_one() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(10i64);
        t.thread_stack.push::<i64>(5i64);
        assert!(lcmp(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 1);
    }
    #[test]
    fn test_lcmp_value1_less_pushes_minus_one() {
        let mut t = make_thread();
        t.thread_stack.push::<i64>(2i64);
        t.thread_stack.push::<i64>(5i64);
        assert!(lcmp(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), -1);
    }

    // fcmpl: NaN → -1
    #[test]
    fn test_fcmpl_equal_floats_push_zero() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(3.0f32);
        t.thread_stack.push::<f32>(3.0f32);
        assert!(fcmpl(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 0);
    }
    #[test]
    fn test_fcmpl_value1_greater_pushes_one() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(5.0f32);
        t.thread_stack.push::<f32>(3.0f32);
        assert!(fcmpl(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 1);
    }
    #[test]
    fn test_fcmpl_nan_pushes_minus_one() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(f32::NAN);
        t.thread_stack.push::<f32>(1.0f32);
        assert!(fcmpl(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), -1);
    }

    // fcmpg: NaN → +1
    #[test]
    fn test_fcmpg_equal_floats_push_zero() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(3.0f32);
        t.thread_stack.push::<f32>(3.0f32);
        assert!(fcmpg(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 0);
    }
    #[test]
    fn test_fcmpg_nan_pushes_positive_one() {
        let mut t = make_thread();
        t.thread_stack.push::<f32>(f32::NAN);
        t.thread_stack.push::<f32>(1.0f32);
        assert!(fcmpg(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 1);
    }

    // dcmpl: NaN → -1
    #[test]
    fn test_dcmpl_equal_doubles_push_zero() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(2.0f64);
        t.thread_stack.push::<f64>(2.0f64);
        assert!(dcmpl(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 0);
    }
    #[test]
    fn test_dcmpl_nan_pushes_minus_one() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(f64::NAN);
        t.thread_stack.push::<f64>(1.0f64);
        assert!(dcmpl(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), -1);
    }

    // dcmpg: NaN → +1
    #[test]
    fn test_dcmpg_equal_doubles_push_zero() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(2.0f64);
        t.thread_stack.push::<f64>(2.0f64);
        assert!(dcmpg(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 0);
    }
    #[test]
    fn test_dcmpg_nan_pushes_positive_one() {
        let mut t = make_thread();
        t.thread_stack.push::<f64>(f64::NAN);
        t.thread_stack.push::<f64>(1.0f64);
        assert!(dcmpg(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop::<i32>().unwrap(), 1);
    }

    // ── 0x99–0xa6  branches ───────────────────────────────────────────────────
    // Convention: use make_thread_at_pc(1, code) so the opcode sits at address 0
    // (already consumed by dispatch).  The two offset bytes are at code[1..=2].
    //
    //   target       = opcode_address + branchoffset
    //                = (pc_at_entry - 1) + branchoffset
    //                = 0 + branchoffset  (when pc_at_entry == 1)
    //
    // code layout used: [0x00, offset_hi, offset_lo]
    //   branch taken  → pc == branchoffset (== 10 with offset 0x000A)
    //   fall through  → pc == 3  (1 + 2 offset bytes consumed)

    // ifeq: branch if value == 0
    #[test]
    fn test_ifeq_branches_when_zero() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]); // offset = 10
        t.thread_stack.push::<i32>(0);
        assert!(ifeq(&mut t).is_ok());
        assert_eq!(t.pc, 10);
    }
    #[test]
    fn test_ifeq_falls_through_when_nonzero() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(1);
        assert!(ifeq(&mut t).is_ok());
        assert_eq!(t.pc, 3);
    }

    // ifne: branch if value != 0
    #[test]
    fn test_ifne_branches_when_nonzero() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(5);
        assert!(ifne(&mut t).is_ok());
        assert_eq!(t.pc, 10);
    }
    #[test]
    fn test_ifne_falls_through_when_zero() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(0);
        assert!(ifne(&mut t).is_ok());
        assert_eq!(t.pc, 3);
    }

    // iflt: branch if value < 0
    #[test]
    fn test_iflt_branches_when_negative() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(-1);
        assert!(iflt(&mut t).is_ok());
        assert_eq!(t.pc, 10);
    }
    #[test]
    fn test_iflt_falls_through_when_nonnegative() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(0);
        assert!(iflt(&mut t).is_ok());
        assert_eq!(t.pc, 3);
    }

    // ifge: branch if value >= 0
    #[test]
    fn test_ifge_branches_when_nonnegative() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(0);
        assert!(ifge(&mut t).is_ok());
        assert_eq!(t.pc, 10);
    }
    #[test]
    fn test_ifge_falls_through_when_negative() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(-1);
        assert!(ifge(&mut t).is_ok());
        assert_eq!(t.pc, 3);
    }

    // ifgt: branch if value > 0
    #[test]
    fn test_ifgt_branches_when_positive() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(1);
        assert!(ifgt(&mut t).is_ok());
        assert_eq!(t.pc, 10);
    }
    #[test]
    fn test_ifgt_falls_through_when_nonpositive() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(0);
        assert!(ifgt(&mut t).is_ok());
        assert_eq!(t.pc, 3);
    }

    // ifle: branch if value <= 0
    #[test]
    fn test_ifle_branches_when_nonpositive() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(0);
        assert!(ifle(&mut t).is_ok());
        assert_eq!(t.pc, 10);
    }
    #[test]
    fn test_ifle_falls_through_when_positive() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(1);
        assert!(ifle(&mut t).is_ok());
        assert_eq!(t.pc, 3);
    }

    // if_icmpeq: branch if value1 == value2
    #[test]
    fn test_if_icmpeq_branches_when_equal() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(5); // value1
        t.thread_stack.push::<i32>(5); // value2
        assert!(if_icmpeq(&mut t).is_ok());
        assert_eq!(t.pc, 10);
    }
    #[test]
    fn test_if_icmpeq_falls_through_when_not_equal() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(5);
        t.thread_stack.push::<i32>(6);
        assert!(if_icmpeq(&mut t).is_ok());
        assert_eq!(t.pc, 3);
    }

    // if_icmpne: branch if value1 != value2
    #[test]
    fn test_if_icmpne_branches_when_not_equal() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(3);
        t.thread_stack.push::<i32>(7);
        assert!(if_icmpne(&mut t).is_ok());
        assert_eq!(t.pc, 10);
    }
    #[test]
    fn test_if_icmpne_falls_through_when_equal() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(5);
        t.thread_stack.push::<i32>(5);
        assert!(if_icmpne(&mut t).is_ok());
        assert_eq!(t.pc, 3);
    }

    // if_icmplt: branch if value1 < value2
    #[test]
    fn test_if_icmplt_branches_when_value1_less() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(2); // value1
        t.thread_stack.push::<i32>(5); // value2
        assert!(if_icmplt(&mut t).is_ok());
        assert_eq!(t.pc, 10);
    }
    #[test]
    fn test_if_icmplt_falls_through_when_value1_not_less() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(5);
        t.thread_stack.push::<i32>(5);
        assert!(if_icmplt(&mut t).is_ok());
        assert_eq!(t.pc, 3);
    }

    // if_icmpge: branch if value1 >= value2
    #[test]
    fn test_if_icmpge_branches_when_value1_ge() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(5);
        t.thread_stack.push::<i32>(5);
        assert!(if_icmpge(&mut t).is_ok());
        assert_eq!(t.pc, 10);
    }
    #[test]
    fn test_if_icmpge_falls_through_when_value1_less() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(3);
        t.thread_stack.push::<i32>(5);
        assert!(if_icmpge(&mut t).is_ok());
        assert_eq!(t.pc, 3);
    }

    // if_icmpgt: branch if value1 > value2
    #[test]
    fn test_if_icmpgt_branches_when_value1_greater() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(10);
        t.thread_stack.push::<i32>(5);
        assert!(if_icmpgt(&mut t).is_ok());
        assert_eq!(t.pc, 10);
    }
    #[test]
    fn test_if_icmpgt_falls_through_when_value1_not_greater() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(5);
        t.thread_stack.push::<i32>(5);
        assert!(if_icmpgt(&mut t).is_ok());
        assert_eq!(t.pc, 3);
    }

    // if_icmple: branch if value1 <= value2
    #[test]
    fn test_if_icmple_branches_when_value1_le() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(5);
        t.thread_stack.push::<i32>(5);
        assert!(if_icmple(&mut t).is_ok());
        assert_eq!(t.pc, 10);
    }
    #[test]
    fn test_if_icmple_falls_through_when_value1_greater() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push::<i32>(10);
        t.thread_stack.push::<i32>(5);
        assert!(if_icmple(&mut t).is_ok());
        assert_eq!(t.pc, 3);
    }

    // if_acmpeq: branch if reference1 == reference2 (same object identity)
    #[test]
    fn test_if_acmpeq_branches_when_same_ref() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push(Reference::new(7)); // value1
        t.thread_stack.push(Reference::new(7)); // value2
        assert!(if_acmpeq(&mut t).is_ok());
        assert_eq!(t.pc, 10);
    }
    #[test]
    fn test_if_acmpeq_falls_through_when_different_refs() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push(Reference::new(5));
        t.thread_stack.push(Reference::new(6));
        assert!(if_acmpeq(&mut t).is_ok());
        assert_eq!(t.pc, 3);
    }

    // if_acmpne: branch if reference1 != reference2
    #[test]
    fn test_if_acmpne_branches_when_different_refs() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push(Reference::new(5));
        t.thread_stack.push(Reference::new(6));
        assert!(if_acmpne(&mut t).is_ok());
        assert_eq!(t.pc, 10);
    }
    #[test]
    fn test_if_acmpne_falls_through_when_same_ref() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push(Reference::new(7));
        t.thread_stack.push(Reference::new(7));
        assert!(if_acmpne(&mut t).is_ok());
        assert_eq!(t.pc, 3);
    }

    // ── 0xa7–0xa9  control flow ───────────────────────────────────────────────

    // goto: unconditionally jump to target = (pc-1) + branchoffset
    #[test]
    fn test_goto_jumps_to_branch_target() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x14]); // offset = 20
        assert!(goto(&mut t).is_ok());
        assert_eq!(t.pc, 20); // (1-1) + 20 = 20
    }

    // ── 0xaa–0xab  switch ─────────────────────────────────────────────────────

    // ── 0xac–0xb1  return ─────────────────────────────────────────────────────

    // ── 0xb2–0xb5  field access ───────────────────────────────────────────────

    // ── 0xb6–0xba  invokes ────────────────────────────────────────────────────

    // ── 0xbb–0xbd  array/object creation ─────────────────────────────────────

    // ── 0xbe  arraylength ─────────────────────────────────────────────────────

    #[test]
    fn test_arraylength_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        assert!(matches!(arraylength(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    // ── 0xbf  athrow ──────────────────────────────────────────────────────────

    #[test]
    fn test_athrow_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        assert!(matches!(athrow(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    // ── 0xc0–0xc1  checkcast / instanceof ────────────────────────────────────

    // ── 0xc2–0xc3  monitor ────────────────────────────────────────────────────

    #[test]
    fn test_monitorenter_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        assert!(matches!(monitorenter(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    #[test]
    fn test_monitorexit_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push(Reference::new(0));
        assert!(matches!(monitorexit(&mut t).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::NullPointerException)));
    }

    // ── 0xc4  wide ────────────────────────────────────────────────────────────

    #[test]
    fn test_wide_sets_wide_mode() {
        let mut t = make_thread();
        assert!(!t.wide_mode);
        assert!(wide(&mut t).is_ok());
        assert!(t.wide_mode);
    }

    // ── 0xc5–0xc9  extended ───────────────────────────────────────────────────

    // ifnull: branch if reference is null
    #[test]
    fn test_ifnull_branches_when_null() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]); // offset = 10
        t.thread_stack.push(Reference::new(0)); // null reference
        assert!(ifnull(&mut t).is_ok());
        assert_eq!(t.pc, 10);
    }
    #[test]
    fn test_ifnull_falls_through_when_nonnull() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push(Reference::new(5)); // non-null reference
        assert!(ifnull(&mut t).is_ok());
        assert_eq!(t.pc, 3);
    }

    // ifnonnull: branch if reference is not null
    #[test]
    fn test_ifnonnull_branches_when_nonnull() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push(Reference::new(5));
        assert!(ifnonnull(&mut t).is_ok());
        assert_eq!(t.pc, 10);
    }
    #[test]
    fn test_ifnonnull_falls_through_when_null() {
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x0A]);
        t.thread_stack.push(Reference::new(0)); // null
        assert!(ifnonnull(&mut t).is_ok());
        assert_eq!(t.pc, 3);
    }

    // goto_w: unconditional jump with a 4-byte big-endian offset
    #[test]
    fn test_goto_w_jumps_to_wide_branch_target() {
        // pc=1; 4 offset bytes at code[1..=4]; offset = 0x0000_0064 = 100
        let mut t = make_thread_at_pc(1, vec![0x00, 0x00, 0x00, 0x00, 0x64]);
        assert!(goto_w(&mut t).is_ok());
        assert_eq!(t.pc, 100); // (1-1) + 100 = 100
    }

    // ── 0xca–0xff  reserved ───────────────────────────────────────────────────

    #[test]
    fn test_reserved_opcode_returns_unknown_error() {
        let mut t = make_thread();
        assert!(matches!(call_by_opcode(&mut t, 0xca).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::UnknownError)));
        assert!(matches!(call_by_opcode(&mut t, 0xff).unwrap_err().downcast_ref::<InstructionError>(), Some(InstructionError::UnknownError)));
    }
}
