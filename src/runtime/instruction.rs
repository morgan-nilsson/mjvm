use crate::runtime::thread::Thread;
use crate::runtime::reference_table::Reference;
use crate::runtime::reference_table::NULL_REF;

use thiserror::Error;

pub fn preform_instruction(thread: &mut Thread) {
    todo!("Preform instruction for thread");

}

fn call_by_opcode(thread: &mut Thread, opcode: u8) -> Result<(), InstructionError> {
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
        0xca..=0xff => { Err(InstructionError::UnknownError) },


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

fn aaload(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the array referenced

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException

    todo!("Implement aaload instruction");

    // push the value at the index onto the stack
}

fn aastore(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;
    let index = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the array referenced

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException

    // set the value at the index to the value popped from the stack

    todo!("Implement aastore instruction");
}

fn aconst_null(thread: &mut Thread) -> Result<(), InstructionError> {

    thread.thread_stack.push_ref(NULL_REF.get_ref_index());

    Ok(())
}

fn aload(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError

    // ensure the local variable at the index is a reference else throw InstructionsError::InternalError

    // push the reference onto the stack

    todo!("Implement aload instruction");
}

fn aload_0(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 0 is a reference else throw InstructionsError::InternalError

    // push the reference from the local variable at index 0 onto the stack

    todo!("Implement aload_0 instruction");
}

fn aload_1(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 1 is a reference else throw InstructionsError::InternalError

    // push the reference from the local variable at index 1 onto the stack

    todo!("Implement aload_1 instruction");
}

fn aload_2(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 2 is a reference else throw InstructionsError::InternalError

    // push the reference from the local variable at index 2 onto the stack

    todo!("Implement aload_2 instruction");
}

fn aload_3(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 3 is a reference else throw InstructionsError::InternalError

    // push the reference from the local variable at index 3 onto the stack

    todo!("Implement aload_3 instruction");
}

fn anewarray(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_byte_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a class reference else throw InstructionsError::InternalError

    // get the class referenced

    // pop the count from the stack

    // ensure count is non-negative else throw NegativeArraySizeException

    // create a new array of the class referenced with the count and push a reference to it onto the stack

    todo!("Implement anewarray instruction");
}

fn areturn(thread: &mut Thread) -> Result<(), InstructionError> {

    todo!("Implement areturn instruction");

}

fn arraylength(thread: &mut Thread) -> Result<(), InstructionError> {

    let array_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the array referenced

    // push the length of the array onto the stack

    todo!("Implement arraylength instruction");

}

fn astore(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError

    // ensure the local variable at the index is a reference else throw InstructionsError::InternalError

    // pop the reference from the stack and store it in the local variable at the index

    todo!("Implement astore instruction");
}

fn astore_0(thread: &mut Thread) -> Result<(), InstructionError> {
    // pop the reference from the stack and store it in the local variable at index 0

    todo!("Implement astore_0 instruction");
}

fn astore_1(thread: &mut Thread) -> Result<(), InstructionError> {
    // pop the reference from the stack and store it in the local variable at index 1

    todo!("Implement astore_1 instruction");
}

fn astore_2(thread: &mut Thread) -> Result<(), InstructionError> {
    // pop the reference from the stack and store it in the local variable at index 2

    todo!("Implement astore_2 instruction");
}

fn astore_3(thread: &mut Thread) -> Result<(), InstructionError> {
    // pop the reference from the stack and store it in the local variable at index 3

    todo!("Implement astore_3 instruction");
}

fn athrow(thread: &mut Thread) -> Result<(), InstructionError> {
    let exception_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if exception_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the exception object referenced

    // throw the exception

    todo!("Implement athrow instruction");
}

fn baload(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the array referenced

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException

    // push the value at the index onto the stack

    todo!("Implement baload instruction");
}

fn bastore(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let index = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the array referenced

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException

    // set the value at the index to the value popped from the stack

    todo!("Implement bastore instruction");
}

fn bipush(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.read_byte_from_pc().ok_or(InstructionError::InternalError)? as i8;

    thread.thread_stack.push_int(value as i32);

    Ok(())
}

fn caload(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the array referenced

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException

    // push the value at the index onto the stack

    todo!("Implement caload instruction");
}

fn castore(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let index = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the array referenced

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException

    // set the value at the index to the value popped from the stack

    todo!("Implement castore instruction");
}

fn checkcast(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a class reference else throw InstructionsError::InternalError

    // get the class referenced

    // ensure the object reference on top of the stack is an instance of the class referenced else throw InstructionsError::InternalError

    todo!("Implement checkcast instruction");
}

fn d2f(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;

    // convert the double value to a float value

    // push the float value onto the stack

    todo!("Implement d2f instruction");
}

fn d2i(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;

    // convert the double value to an int value

    // push the int value onto the stack

    todo!("Implement d2i instruction");
}

fn d2l(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;

    // convert the double value to a long value

    // push the long value onto the stack

    todo!("Implement d2l instruction");
}

fn dadd(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;

    // add the two double values

    // push the result onto the stack

    todo!("Implement dadd instruction");
}

fn daload(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the array referenced

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException

    // push the value at the index onto the stack

    todo!("Implement daload instruction");
}

fn dastore(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;
    let index = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the array referenced

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException

    // set the value at the index to the value popped from the stack

    todo!("Implement dastore instruction");
}

fn dcmpg(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;

    // compare the two double values

    // push the result onto the stack

    todo!("Implement dcmpg instruction");
}

fn dcmpl(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;

    // compare the two double values

    // push the result onto the stack

    todo!("Implement dcmpl instruction");
}

fn dconst_0(thread: &mut Thread) -> Result<(), InstructionError> {
    thread.thread_stack.push_double(0.0);

    Ok(())
}

fn dconst_1(thread: &mut Thread) -> Result<(), InstructionError> {
    thread.thread_stack.push_double(1.0);

    Ok(())
}

fn ddiv(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;

    // divide the two double values

    // push the result onto the stack

    todo!("Implement ddiv instruction");
}

fn dload(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError

    // ensure the local variable at the index is a double else throw InstructionsError::InternalError

    // push the double value from the local variable at the index onto the stack

    todo!("Implement dload instruction");
}

fn dload_0(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 0 is a double else throw InstructionsError::InternalError

    // push the double value from the local variable at index 0 onto the stack

    todo!("Implement dload_0 instruction");
}

fn dload_1(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 1 is a double else throw InstructionsError::InternalError

    // push the double value from the local variable at index 1 onto the stack

    todo!("Implement dload_1 instruction");
}

fn dload_2(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 2 is a double else throw InstructionsError::InternalError

    // push the double value from the local variable at index 2 onto the stack

    todo!("Implement dload_2 instruction");
}

fn dload_3(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 3 is a double else throw InstructionsError::InternalError

    // push the double value from the local variable at index 3 onto the stack

    todo!("Implement dload_3 instruction");
}

fn dmul(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;

    // multiply the two double values

    // push the result onto the stack

    todo!("Implement dmul instruction");
}

fn dneg(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;

    // negate the double value

    // push the result onto the stack

    todo!("Implement dneg instruction");
}

fn drem(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;

    // compute the remainder of the division of the two double values

    // push the result onto the stack

    todo!("Implement drem instruction");
}

fn dreturn(thread: &mut Thread) -> Result<(), InstructionError> {

    todo!("Implement dreturn instruction");

}

fn dstore(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError

    // ensure the local variable at the index is a double else throw InstructionsError::InternalError

    // pop the double value from the stack and store it in the local variable at the index

    todo!("Implement dstore instruction");
}

fn dstore_0(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 0 is a double else throw InstructionsError::InternalError

    // pop the double value from the stack and store it in the local variable at index 0

    todo!("Implement dstore_0 instruction");
}

fn dstore_1(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 1 is a double else throw InstructionsError::InternalError

    // pop the double value from the stack and store it in the local variable at index 1

    todo!("Implement dstore_1 instruction");
}

fn dstore_2(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 2 is a double else throw InstructionsError::InternalError

    // pop the double value from the stack and store it in the local variable at index 2

    todo!("Implement dstore_2 instruction");
}

fn dstore_3(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 3 is a double else throw InstructionsError::InternalError

    // pop the double value from the stack and store it in the local variable at index 3

    todo!("Implement dstore_3 instruction");
}

fn dsub(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_double().ok_or(InstructionError::InternalError)?;

    // subtract the two double values

    // push the result onto the stack

    todo!("Implement dsub instruction");
}

fn dup(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    // push the value back onto the stack twice

    todo!("Implement dup instruction");
}

fn dup_x1(thread: &mut Thread) -> Result<(), InstructionError> {
    let value1 = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;
    let value2 = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    // push value1 back onto the stack, then push value2, then push value1 again

    todo!("Implement dup_x1 instruction");
}

fn dup_x2(thread: &mut Thread) -> Result<(), InstructionError> {
    let value1 = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;
    let value2 = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;
    let value3 = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    // push value1 back onto the stack, then push value3, then push value2, then push value1 again

    todo!("Implement dup_x2 instruction");
}

fn dup2(thread: &mut Thread) -> Result<(), InstructionError> {
    let value1 = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;
    let value2 = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    // push value2 back onto the stack, then push value1, then push value2 again, then push value1 again

    todo!("Implement dup2 instruction");
}

fn dup2_x1(thread: &mut Thread) -> Result<(), InstructionError> {
    let value1 = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;
    let value2 = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;
    let value3 = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    // push value2 back onto the stack, then push value1, then push value3, then push value2 again, then push value1 again

    todo!("Implement dup2_x1 instruction");
}

fn dup2_x2(thread: &mut Thread) -> Result<(), InstructionError> {
    let value1 = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;
    let value2 = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;
    let value3 = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;
    let value4 = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    // push value2 back onto the stack, then push value1, then push value4, then push value3, then push value2 again, then push value1 again

    todo!("Implement dup2_x2 instruction");
}

fn f2d(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;

    // convert the float value to a double value

    // push the double value onto the stack

    todo!("Implement f2d instruction");
}

fn f2i(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;

    // convert the float value to an int value

    // push the int value onto the stack

    todo!("Implement f2i instruction");
}

fn f2l(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;

    // convert the float value to a long value

    // push the long value onto the stack

    todo!("Implement f2l instruction");
}

fn fadd(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;

    // add the two float values

    // push the result onto the stack

    todo!("Implement fadd instruction");
}

fn faload(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the array referenced

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException

    // push the value at the index onto the stack

    todo!("Implement faload instruction");
}

fn fastore(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;
    let index = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the array referenced

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException

    // set the value at the index to the value popped from the stack

    todo!("Implement fastore instruction");
}

fn fcmpg(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;

    // compare the two float values

    // push the result onto the stack

    todo!("Implement fcmpg instruction");
}

fn fcmpl(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;

    // compare the two float values

    // push the result onto the stack

    todo!("Implement fcmpl instruction");
}

fn fconst_0(thread: &mut Thread) -> Result<(), InstructionError> {
    thread.thread_stack.push_float(0.0);

    Ok(())
}

fn fconst_1(thread: &mut Thread) -> Result<(), InstructionError> {
    thread.thread_stack.push_float(1.0);

    Ok(())
}

fn fconst_2(thread: &mut Thread) -> Result<(), InstructionError> {
    thread.thread_stack.push_float(2.0);

    Ok(())
}

fn fdiv(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;

    // divide the two float values

    // push the result onto the stack

    todo!("Implement fdiv instruction");
}

fn fload(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError

    // ensure the local variable at the index is a float else throw InstructionsError::InternalError

    // push the float value from the local variable at the index onto the stack

    todo!("Implement fload instruction");
}

fn fload_0(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 0 is a float else throw InstructionsError::InternalError

    // push the float value from the local variable at index 0 onto the stack

    todo!("Implement fload_0 instruction");
}

fn fload_1(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 1 is a float else throw InstructionsError::InternalError

    // push the float value from the local variable at index 1 onto the stack

    todo!("Implement fload_1 instruction");
}

fn fload_2(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 2 is a float else throw InstructionsError::InternalError

    // push the float value from the local variable at index 2 onto the stack

    todo!("Implement fload_2 instruction");
}

fn fload_3(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 3 is a float else throw InstructionsError::InternalError

    // push the float value from the local variable at index 3 onto the stack

    todo!("Implement fload_3 instruction");
}

fn fmul(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;

    // multiply the two float values

    // push the result onto the stack

    todo!("Implement fmul instruction");
}

fn fneg(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;

    // negate the float value

    // push the result onto the stack

    todo!("Implement fneg instruction");
}

fn frem(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;

    // compute the remainder of the division of the two float values

    // push the result onto the stack

    todo!("Implement frem instruction");
}

fn freturn(thread: &mut Thread) -> Result<(), InstructionError> {

    todo!("Implement freturn instruction");

}

fn fstore(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError

    // ensure the local variable at the index is a float else throw InstructionsError::InternalError

    // pop the float value from the stack and store it in the local variable at the index

    todo!("Implement fstore instruction");
}

fn fstore_0(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 0 is a float else throw InstructionsError::InternalError

    // pop the float value from the stack and store it in the local variable at index 0

    todo!("Implement fstore_0 instruction");
}

fn fstore_1(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 1 is a float else throw InstructionsError::InternalError

    // pop the float value from the stack and store it in the local variable at index 1

    todo!("Implement fstore_1 instruction");
}

fn fstore_2(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 2 is a float else throw InstructionsError::InternalError

    // pop the float value from the stack and store it in the local variable at index 2

    todo!("Implement fstore_2 instruction");
}

fn fstore_3(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 3 is a float else throw InstructionsError::InternalError

    // pop the float value from the stack and store it in the local variable at index 3

    todo!("Implement fstore_3 instruction");
}

fn fsub(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_float().ok_or(InstructionError::InternalError)?;

    // subtract the two float values

    // push the result onto the stack

    todo!("Implement fsub instruction");
}

fn getfield(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a field reference else throw InstructionsError::InternalError

    // get the field referenced

    // ensure the object reference on top of the stack is not null else throw InstructionsError::NullPointerException

    // get the value of the field from the object reference on top of the stack

    // push the value onto the stack

    todo!("Implement getfield instruction");
}

fn getstatic(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a field reference else throw InstructionsError::InternalError

    // get the field referenced

    // get the value of the static field

    // push the value onto the stack

    todo!("Implement getstatic instruction");
}

fn goto(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;

    // jump to the instruction at the offset from the current instruction

    todo!("Implement goto instruction");
}

fn goto_w(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_int_from_pc().ok_or(InstructionError::InternalError)? as i32;

    // jump to the instruction at the offset from the current instruction

    todo!("Implement goto_w instruction");
}

fn i2b(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // convert the int value to a byte value

    // push the byte value onto the stack

    todo!("Implement i2b instruction");
}

fn i2c(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // convert the int value to a char value

    // push the char value onto the stack

    todo!("Implement i2c instruction");
}

fn i2d(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // convert the int value to a double value

    // push the double value onto the stack

    todo!("Implement i2d instruction");
}

fn i2f(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // convert the int value to a float value

    // push the float value onto the stack

    todo!("Implement i2f instruction");
}

fn i2l(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // convert the int value to a long value

    // push the long value onto the stack

    todo!("Implement i2l instruction");
}

fn i2s(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // convert the int value to a short value

    // push the short value onto the stack

    todo!("Implement i2s instruction");
}

fn iadd(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // add the two int values

    // push the result onto the stack

    todo!("Implement iadd instruction");
}

fn iaload(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the array referenced

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException

    // push the value at the index onto the stack

    todo!("Implement iaload instruction");
}

fn iand(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // perform a bitwise AND operation on the two int values

    // push the result onto the stack

    todo!("Implement iand instruction");
}

fn iastore(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let index = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the array referenced

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException

    // set the value at the index to the value popped from the stack

    todo!("Implement iastore instruction");
}

fn iconst_m1(thread: &mut Thread) -> Result<(), InstructionError> {
    thread.thread_stack.push_int(-1);

    Ok(())
}

fn iconst_0(thread: &mut Thread) -> Result<(), InstructionError> {
    thread.thread_stack.push_int(0);

    Ok(())
}

fn iconst_1(thread: &mut Thread) -> Result<(), InstructionError> {
    thread.thread_stack.push_int(1);

    Ok(())
}

fn iconst_2(thread: &mut Thread) -> Result<(), InstructionError> {
    thread.thread_stack.push_int(2);

    Ok(())
}

fn iconst_3(thread: &mut Thread) -> Result<(), InstructionError> {
    thread.thread_stack.push_int(3);

    Ok(())
}

fn iconst_4(thread: &mut Thread) -> Result<(), InstructionError> {
    thread.thread_stack.push_int(4);

    Ok(())
}

fn iconst_5(thread: &mut Thread) -> Result<(), InstructionError> {
    thread.thread_stack.push_int(5);

    Ok(())
}

fn idiv(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    if value2 == 0 {
        return Err(InstructionError::ArithmeticException);
    }

    // divide the two int values

    // push the result onto the stack

    todo!("Implement idiv instruction");
}

fn if_acmpeq(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value2 = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    // if the two object references are equal, jump to the instruction at the offset from the current instruction

    todo!("Implement if_acmpeq instruction");
}

fn if_acmpne(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value2 = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    // if the two object references are not equal, jump to the instruction at the offset from the current instruction

    todo!("Implement if_acmpne instruction");
}

fn if_icmpeq(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // if the two int values are equal, jump to the instruction at the offset from the current instruction

    todo!("Implement if_icmpeq instruction");
}

fn if_icmpne(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // if the two int values are not equal, jump to the instruction at the offset from the current instruction

    todo!("Implement if_icmpne instruction");
}

fn if_icmplt(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // if value1 is less than value2, jump to the instruction at the offset from the current instruction

    todo!("Implement if_icmplt instruction");
}

fn if_icmpge(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // if value1 is greater than or equal to value2, jump to the instruction at the offset from the current instruction

    todo!("Implement if_icmpge instruction");
}

fn if_icmpgt(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // if value1 is greater than value2, jump to the instruction at the offset from the current instruction

    todo!("Implement if_icmpgt instruction");
}

fn if_icmple(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // if value1 is less than or equal to value2, jump to the instruction at the offset from the current instruction

    todo!("Implement if_icmple instruction");
}

fn ifeq(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // if the int value is equal to 0, jump to the instruction at the offset from the current instruction

    todo!("Implement ifeq instruction");
}

fn ifne(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // if the int value is not equal to 0, jump to the instruction at the offset from the current instruction

    todo!("Implement ifne instruction");
}

fn iflt(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // if the int value is less than 0, jump to the instruction at the offset from the current instruction

    todo!("Implement iflt instruction");
}

fn ifge(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // if the int value is greater than or equal to 0, jump to the instruction at the offset from the current instruction

    todo!("Implement ifge instruction");
}

fn ifgt(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // if the int value is greater than 0, jump to the instruction at the offset from the current instruction

    todo!("Implement ifgt instruction");
}

fn ifle(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // if the int value is less than or equal to 0, jump to the instruction at the offset from the current instruction

    todo!("Implement ifle instruction");
}

fn ifnonnull(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    // if the object reference is not null, jump to the instruction at the offset from the current instruction

    todo!("Implement ifnonnull instruction");
}

fn ifnull(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;
    let value = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    // if the object reference is null, jump to the instruction at the offset from the current instruction

    todo!("Implement ifnull instruction");
}

fn iinc(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_byte_from_pc().ok_or(InstructionError::InternalError)? as u16;
    let constant = thread.read_byte_from_pc().ok_or(InstructionError::InternalError)? as i8;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError

    // ensure the local variable at the index is an int else throw InstructionsError::InternalError

    // increment the int value in the local variable at the index by the constant

    todo!("Implement iinc instruction");
}

fn iload(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError

    // ensure the local variable at the index is an int else throw InstructionsError::InternalError

    // push the int value from the local variable at the index onto the stack

    todo!("Implement iload instruction");
}

fn iload_0(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 0 is an int else throw InstructionsError::InternalError

    // push the int value from the local variable at index 0 onto the stack

    todo!("Implement iload_0 instruction");
}

fn iload_1(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 1 is an int else throw InstructionsError::InternalError

    // push the int value from the local variable at index 1 onto the stack

    todo!("Implement iload_1 instruction");
}

fn iload_2(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 2 is an int else throw InstructionsError::InternalError

    // push the int value from the local variable at index 2 onto the stack

    todo!("Implement iload_2 instruction");
}

fn iload_3(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 3 is an int else throw InstructionsError::InternalError

    // push the int value from the local variable at index 3 onto the stack

    todo!("Implement iload_3 instruction");
}

fn imul(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // multiply the two int values

    // push the result onto the stack

    todo!("Implement imul instruction");
}

fn ineg(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // negate the int value

    // push the result onto the stack

    todo!("Implement ineg instruction");
}

fn instanceof(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a class reference else throw InstructionsError::InternalError

    // get the class referenced

    // ensure the object reference on top of the stack is not null else throw InstructionsError::NullPointerException

    // check if the object reference on top of the stack is an instance of the class referenced

    // push 1 onto the stack if it is an instance, otherwise push 0

    todo!("Implement instanceof instruction");
}

fn invokedynamic(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a method reference else throw InstructionsError::InternalError

    // get the method referenced

    // invoke the method dynamically

    todo!("Implement invokedynamic instruction");
}

fn invokeinterface(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;
    let count = thread.read_byte_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a method reference else throw InstructionsError::InternalError

    // get the method referenced

    // ensure the object reference on top of the stack is not null else throw InstructionsError::NullPointerException

    // invoke the method on the object reference on top of the stack

    todo!("Implement invokeinterface instruction");
}

fn invokespecial(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a method reference else throw InstructionsError::InternalError

    // get the method referenced

    // ensure the object reference on top of the stack is not null else throw InstructionsError::NullPointerException

    // invoke the method on the object reference on top of the stack

    todo!("Implement invokespecial instruction");
}

fn invokestatic(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a method reference else throw InstructionsError::InternalError

    // get the method referenced

    // invoke the static method

    todo!("Implement invokestatic instruction");
}

fn invokevirtual(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a method reference else throw InstructionsError::InternalError

    // get the method referenced

    // ensure the object reference on top of the stack is not null else throw InstructionsError::NullPointerException

    // invoke the method on the object reference on top of the stack

    todo!("Implement invokevirtual instruction");
}

fn ior(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // perform a bitwise OR operation on the two int values

    // push the result onto the stack

    todo!("Implement ior instruction");
}

fn irem(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    if value2 == 0 {
        return Err(InstructionError::ArithmeticException);
    }

    // compute the remainder of the division of the two int values

    // push the result onto the stack

    todo!("Implement irem instruction");
}

fn ireturn(thread: &mut Thread) -> Result<(), InstructionError> {

    todo!("Implement ireturn instruction");

}

fn ishl(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // shift the first int value to the left by the number of bits specified by the second int value

    // push the result onto the stack

    todo!("Implement ishl instruction");
}

fn ishr(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // shift the first int value to the right by the number of bits specified by the second int value, using sign extension

    // push the result onto the stack

    todo!("Implement ishr instruction");
}

fn istore(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError

    // ensure the local variable at the index is an int else throw InstructionsError::InternalError

    // pop the int value from the stack and store it in the local variable at the index

    todo!("Implement istore instruction");
}

fn istore_0(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 0 is an int else throw InstructionsError::InternalError

    // pop the int value from the stack and store it in the local variable at index 0

    todo!("Implement istore_0 instruction");
}

fn istore_1(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 1 is an int else throw InstructionsError::InternalError

    // pop the int value from the stack and store it in the local variable at index 1

    todo!("Implement istore_1 instruction");
}

fn istore_2(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 2 is an int else throw InstructionsError::InternalError

    // pop the int value from the stack and store it in the local variable at index 2

    todo!("Implement istore_2 instruction");
}

fn istore_3(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 3 is an int else throw InstructionsError::InternalError

    // pop the int value from the stack and store it in the local variable at index 3

    todo!("Implement istore_3 instruction");
}

fn isub(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // subtract the two int values

    // push the result onto the stack

    todo!("Implement isub instruction");
}

fn iushr(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // shift the first int value to the right by the number of bits specified by the second int value, using zero extension

    // push the result onto the stack

    todo!("Implement iushr instruction");
}

fn ixor(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    // perform a bitwise XOR operation on the two int values

    // push the result onto the stack

    todo!("Implement ixor instruction");
}

fn jsr(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_short_from_pc().ok_or(InstructionError::InternalError)? as i16;

    // push the address of the next instruction onto the stack

    // jump to the instruction at the offset from the current instruction

    todo!("Implement jsr instruction");
}

fn jsr_w(thread: &mut Thread) -> Result<(), InstructionError> {
    let offset = thread.read_int_from_pc().ok_or(InstructionError::InternalError)? as i32;

    // push the address of the next instruction onto the stack

    // jump to the instruction at the offset from the current instruction

    todo!("Implement jsr_w instruction");
}

fn l2d(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;

    // convert the long value to a double value

    // push the double value onto the stack

    todo!("Implement l2d instruction");
}

fn l2f(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;

    // convert the long value to a float value

    // push the float value onto the stack

    todo!("Implement l2f instruction");
}

fn l2i(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;

    // convert the long value to an int value

    // push the int value onto the stack

    todo!("Implement l2i instruction");
}

fn ladd(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;

    // add the two long values

    // push the result onto the stack

    todo!("Implement ladd instruction");
}

fn laload(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the array referenced

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException

    // push the value at the index onto the stack

    todo!("Implement laload instruction");
}

fn land(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;

    // perform a bitwise AND operation on the two long values

    // push the result onto the stack

    todo!("Implement land instruction");
}

fn lastore(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;
    let index = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the array referenced

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException

    // set the value at the index to the value popped from the stack

    todo!("Implement lastore instruction");
}

fn lcmp(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;

    // compare the two long values

    // push 0 onto the stack if the values are equal, 1 if value1 is greater than value2, and -1 if value1 is less than value2

    todo!("Implement lcmp instruction");
}

fn lconst_0(thread: &mut Thread) -> Result<(), InstructionError> {
    thread.thread_stack.push_long(0);

    Ok(())
}

fn lconst_1(thread: &mut Thread) -> Result<(), InstructionError> {
    thread.thread_stack.push_long(1);

    Ok(())
}

fn ldc(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_byte_from_pc().ok_or(InstructionError::InternalError)? as u16;

    // ensure the constant pool entry at the index is a class reference, field reference, method reference, string reference, or integer constant else throw InstructionsError::InternalError

    // get the constant referenced

    // push the constant onto the stack

    todo!("Implement ldc instruction");
}

fn ldc_w(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a class reference, field reference, method reference, string reference, or integer constant else throw InstructionsError::InternalError

    // get the constant referenced

    // push the constant onto the stack

    todo!("Implement ldc_w instruction");
}

fn ldc2_w(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a long constant or double constant else throw InstructionsError::InternalError

    // get the constant referenced

    // push the constant onto the stack

    todo!("Implement ldc2_w instruction");
}

fn ldiv(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;

    if value2 == 0 {
        return Err(InstructionError::ArithmeticException);
    }

    // divide the two long values

    // push the result onto the stack

    todo!("Implement ldiv instruction");
}

fn lload(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError

    // ensure the local variable at the index is a long else throw InstructionsError::InternalError

    // push the long value from the local variable at the index onto the stack

    todo!("Implement lload instruction");
}

fn lload_0(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 0 is a long else throw InstructionsError::InternalError

    // push the long value from the local variable at index 0 onto the stack

    todo!("Implement lload_0 instruction");
}

fn lload_1(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 1 is a long else throw InstructionsError::InternalError

    // push the long value from the local variable at index 1 onto the stack

    todo!("Implement lload_1 instruction");
}

fn lload_2(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 2 is a long else throw InstructionsError::InternalError

    // push the long value from the local variable at index 2 onto the stack

    todo!("Implement lload_2 instruction");
}

fn lload_3(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 3 is a long else throw InstructionsError::InternalError

    // push the long value from the local variable at index 3 onto the stack

    todo!("Implement lload_3 instruction");
}

fn lmul(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;

    // multiply the two long values

    // push the result onto the stack

    todo!("Implement lmul instruction");
}

fn lneg(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;

    // negate the long value

    // push the result onto the stack

    todo!("Implement lneg instruction");
}

fn lookupswitch(thread: &mut Thread) -> Result<(), InstructionError> {
    // skip padding bytes

    let default_offset = thread.read_int_from_pc().ok_or(InstructionError::InternalError)?;
    let npairs = thread.read_int_from_pc().ok_or(InstructionError::InternalError)?;

    // read the match-offset pairs

    // get the int value on top of the stack

    // search the match-offset pairs for a match with the int value

    // if a match is found, jump to the instruction at the offset from the current instruction specified by the match, otherwise jump to the instruction at the default offset from the current instruction

    todo!("Implement lookupswitch instruction");
}

fn lor(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;

    // perform a bitwise OR operation on the two long values

    // push the result onto the stack

    todo!("Implement lor instruction");
}

fn lrem(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;

    if value2 == 0 {
        return Err(InstructionError::ArithmeticException);
    }

    // compute the remainder of the division of the two long values

    // push the result onto the stack

    todo!("Implement lrem instruction");
}

fn lreturn(thread: &mut Thread) -> Result<(), InstructionError> {

    todo!("Implement lreturn instruction");

}

fn lshl(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;

    // shift the long value to the left by the number of bits specified by the int value

    // push the result onto the stack

    todo!("Implement lshl instruction");
}

fn lshr(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;

    // shift the long value to the right by the number of bits specified by the int value, using sign extension

    // push the result onto the stack

    todo!("Implement lshr instruction");
}

fn lstore(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = (match thread.wide_mode {
        true => thread.read_short_from_pc(),
        false => thread.read_byte_from_pc().map(|b| b as u16),
    }).ok_or(InstructionError::InternalError)?;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError

    // ensure the local variable at the index is a long else throw InstructionsError::InternalError

    // pop the long value from the stack and store it in the local variable at the index

    todo!("Implement lstore instruction");
}

fn lstore_0(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 0 is a long else throw InstructionsError::InternalError

    // pop the long value from the stack and store it in the local variable at index 0

    todo!("Implement lstore_0 instruction");
}

fn lstore_1(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 1 is a long else throw InstructionsError::InternalError

    // pop the long value from the stack and store it in the local variable at index 1

    todo!("Implement lstore_1 instruction");
}

fn lstore_2(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 2 is a long else throw InstructionsError::InternalError

    // pop the long value from the stack and store it in the local variable at index 2

    todo!("Implement lstore_2 instruction");
}

fn lstore_3(thread: &mut Thread) -> Result<(), InstructionError> {
    // ensure the local variable at index 3 is a long else throw InstructionsError::InternalError

    // pop the long value from the stack and store it in the local variable at index 3

    todo!("Implement lstore_3 instruction");
}

fn lsub(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;

    // subtract the two long values

    // push the result onto the stack

    todo!("Implement lsub instruction");
}

fn lushr(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;

    // shift the long value to the right by the number of bits specified by the int value, using zero extension

    // push the result onto the stack

    todo!("Implement lushr instruction");
}

fn lxor(thread: &mut Thread) -> Result<(), InstructionError> {
    let value2 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;
    let value1 = thread.thread_stack.pop_long().ok_or(InstructionError::InternalError)?;

    // perform a bitwise XOR operation on the two long values

    // push the result onto the stack

    todo!("Implement lxor instruction");
}

fn monitorenter(thread: &mut Thread) -> Result<(), InstructionError> {
    let object_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if object_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the object monitor for the object reference

    // enter the monitor, blocking if necessary until it is available

    todo!("Implement monitorenter instruction");
}

fn monitorexit(thread: &mut Thread) -> Result<(), InstructionError> {
    let object_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if object_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the object monitor for the object reference

    // exit the monitor

    todo!("Implement monitorexit instruction");
}

fn multianewarray(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;
    let dimensions = thread.read_byte_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a class reference else throw InstructionsError::InternalError

    // get the class referenced

    // pop the dimension sizes from the stack

    // create a new multi-dimensional array with the specified dimensions and push a reference to it onto the stack

    todo!("Implement multianewarray instruction");
}

fn new(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a class reference else throw InstructionsError::InternalError

    // get the class referenced

    // create a new instance of the class and push a reference to it onto the stack

    todo!("Implement new instruction");
}

fn newarray(thread: &mut Thread) -> Result<(), InstructionError> {
    let atype = thread.read_byte_from_pc().ok_or(InstructionError::InternalError)?;
    let count = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;

    if count < 0 {
        return Err(InstructionError::NegativeArraySizeException);
    }

    // create a new array of the specified type and length and push a reference to it onto the stack

    todo!("Implement newarray instruction");
}

fn nop(thread: &mut Thread) -> Result<(), InstructionError> {
    // do nothing

    Ok(())
}

fn pop(thread: &mut Thread) -> Result<(), InstructionError> {
    thread.thread_stack.pop_short().ok_or(InstructionError::InternalError)?;

    Ok(())
}

fn pop2(thread: &mut Thread) -> Result<(), InstructionError> {
    thread.thread_stack.pop_short().ok_or(InstructionError::InternalError)?;
    thread.thread_stack.pop_short().ok_or(InstructionError::InternalError)?;

    Ok(())
}

fn putfield(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a field reference else throw InstructionsError::InternalError

    // get the field referenced

    // ensure the object reference on top of the stack is not null else throw InstructionsError::NullPointerException

    // pop the value from the stack and set the field of the object reference on top of the stack to the value

    todo!("Implement putfield instruction");
}

fn putstatic(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // ensure the constant pool entry at the index is a field reference else throw InstructionsError::InternalError

    // get the field referenced

    // pop the value from the stack and set the static field to the value

    todo!("Implement putstatic instruction");
}

fn ret(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.read_byte_from_pc().ok_or(InstructionError::InternalError)? as u16;

    // ensure index is within bounds of local variable array else throw InstructionsError::InternalError

    // ensure the local variable at the index is an int else throw InstructionsError::InternalError

    // return from the subroutine at the address specified by the local variable at the index

    todo!("Implement ret instruction");
}

fn return_(thread: &mut Thread) -> Result<(), InstructionError> {

    todo!("Implement return instruction");

}

fn saload(thread: &mut Thread) -> Result<(), InstructionError> {
    let index = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the array referenced

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException

    // push the value at the index onto the stack

    todo!("Implement saload instruction");
}

fn sastore(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let index = thread.thread_stack.pop_int().ok_or(InstructionError::InternalError)?;
    let array_ref = thread.thread_stack.pop_ref().ok_or(InstructionError::InternalError)?;

    if array_ref.is_null() {
        return Err(InstructionError::NullPointerException);
    }

    // get the array referenced

    // ensure array index is within bounds else throw ArrayIndexOutOfBoundsException

    // set the value at the index to the value popped from the stack

    todo!("Implement sastore instruction");
}

fn sipush(thread: &mut Thread) -> Result<(), InstructionError> {
    let value = thread.read_short_from_pc().ok_or(InstructionError::InternalError)?;

    // push the short value onto the stack as an int

    todo!("Implement sipush instruction");
}

fn swap(thread: &mut Thread) -> Result<(), InstructionError> {
    let value1 = thread.thread_stack.pop_short().ok_or(InstructionError::InternalError)?;
    let value2 = thread.thread_stack.pop_short().ok_or(InstructionError::InternalError)?;

    // push the values back onto the stack in reverse order

    thread.thread_stack.push_short(value1);
    thread.thread_stack.push_short(value2);

    Ok(())
}

fn tableswitch(thread: &mut Thread) -> Result<(), InstructionError> {
    // skip padding bytes

    let default_offset = thread.read_int_from_pc().ok_or(InstructionError::InternalError)?;
    let low = thread.read_int_from_pc().ok_or(InstructionError::InternalError)?;
    let high = thread.read_int_from_pc().ok_or(InstructionError::InternalError)?;

    // read the jump offsets

    // get the int value on top of the stack

    // if the int value is between low and high, jump to the instruction at the offset from the current instruction specified by the int value, otherwise jump to the instruction at the default offset from the current instruction

    todo!("Implement tableswitch instruction");
}

fn wide(thread: &mut Thread) -> Result<(), InstructionError> {
    thread.wide_mode = true;

    Ok(())
}

