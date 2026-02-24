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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::thread::{JVMHeap, MethodArea, NativeMethodStack, Thread, ThreadStack};
    use std::sync::{Arc, Mutex};

    fn make_thread() -> Thread {
        Thread {
            pc: 0,
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
        assert!(t.thread_stack.pop_int().is_none());
    }

    // ── 0x01  aconst_null ─────────────────────────────────────────────────────

    #[test]
    fn test_aconst_null_pushes_null_reference() {
        let mut t = make_thread();
        assert!(aconst_null(&mut t).is_ok());
        let r = t.thread_stack.pop_ref().unwrap();
        assert!(r.is_null());
    }

    // ── 0x02–0x08  iconst ─────────────────────────────────────────────────────

    #[test]
    fn test_iconst_m1_pushes_negative_one() {
        let mut t = make_thread();
        assert!(iconst_m1(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop_int().unwrap(), -1);
    }

    #[test]
    fn test_iconst_0_pushes_zero() {
        let mut t = make_thread();
        assert!(iconst_0(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop_int().unwrap(), 0);
    }

    #[test]
    fn test_iconst_1_pushes_one() {
        let mut t = make_thread();
        assert!(iconst_1(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop_int().unwrap(), 1);
    }

    #[test]
    fn test_iconst_2_pushes_two() {
        let mut t = make_thread();
        assert!(iconst_2(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop_int().unwrap(), 2);
    }

    #[test]
    fn test_iconst_3_pushes_three() {
        let mut t = make_thread();
        assert!(iconst_3(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop_int().unwrap(), 3);
    }

    #[test]
    fn test_iconst_4_pushes_four() {
        let mut t = make_thread();
        assert!(iconst_4(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop_int().unwrap(), 4);
    }

    #[test]
    fn test_iconst_5_pushes_five() {
        let mut t = make_thread();
        assert!(iconst_5(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop_int().unwrap(), 5);
    }

    // ── 0x09–0x0a  lconst ─────────────────────────────────────────────────────

    #[test]
    fn test_lconst_0_pushes_zero_long() {
        let mut t = make_thread();
        assert!(lconst_0(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop_long().unwrap(), 0i64);
    }

    #[test]
    fn test_lconst_1_pushes_one_long() {
        let mut t = make_thread();
        assert!(lconst_1(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop_long().unwrap(), 1i64);
    }

    // ── 0x0b–0x0d  fconst ─────────────────────────────────────────────────────

    #[test]
    fn test_fconst_0_pushes_zero_float() {
        let mut t = make_thread();
        assert!(fconst_0(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop_float().unwrap(), 0.0f32);
    }

    #[test]
    fn test_fconst_1_pushes_one_float() {
        let mut t = make_thread();
        assert!(fconst_1(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop_float().unwrap(), 1.0f32);
    }

    #[test]
    fn test_fconst_2_pushes_two_float() {
        let mut t = make_thread();
        assert!(fconst_2(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop_float().unwrap(), 2.0f32);
    }

    // ── 0x0e–0x0f  dconst ─────────────────────────────────────────────────────

    #[test]
    fn test_dconst_0_pushes_zero_double() {
        let mut t = make_thread();
        assert!(dconst_0(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop_double().unwrap(), 0.0f64);
    }

    #[test]
    fn test_dconst_1_pushes_one_double() {
        let mut t = make_thread();
        assert!(dconst_1(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop_double().unwrap(), 1.0f64);
    }

    // ── 0x10  bipush ──────────────────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_bipush_unimplemented() {
        let _ = bipush(&mut make_thread());
    }

    // ── 0x11  sipush ──────────────────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_sipush_unimplemented() {
        let _ = sipush(&mut make_thread());
    }

    // ── 0x12–0x14  ldc / ldc_w / ldc2_w ──────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_ldc_unimplemented() {
        let _ = ldc(&mut make_thread());
    }

    #[test]
    #[should_panic]
    fn test_ldc_w_unimplemented() {
        let _ = ldc_w(&mut make_thread());
    }

    #[test]
    #[should_panic]
    fn test_ldc2_w_unimplemented() {
        let _ = ldc2_w(&mut make_thread());
    }

    // ── 0x15–0x19  generic loads ──────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_iload_unimplemented() {
        let _ = iload(&mut make_thread());
    }

    #[test]
    #[should_panic]
    fn test_lload_unimplemented() {
        let _ = lload(&mut make_thread());
    }

    #[test]
    #[should_panic]
    fn test_fload_unimplemented() {
        let _ = fload(&mut make_thread());
    }

    #[test]
    #[should_panic]
    fn test_dload_unimplemented() {
        let _ = dload(&mut make_thread());
    }

    #[test]
    #[should_panic]
    fn test_aload_unimplemented() {
        let _ = aload(&mut make_thread());
    }

    // ── 0x1a–0x2d  indexed loads ──────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_iload_0_unimplemented() { let _ = iload_0(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_iload_1_unimplemented() { let _ = iload_1(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_iload_2_unimplemented() { let _ = iload_2(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_iload_3_unimplemented() { let _ = iload_3(&mut make_thread()); }

    #[test]
    #[should_panic]
    fn test_lload_0_unimplemented() { let _ = lload_0(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_lload_1_unimplemented() { let _ = lload_1(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_lload_2_unimplemented() { let _ = lload_2(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_lload_3_unimplemented() { let _ = lload_3(&mut make_thread()); }

    #[test]
    #[should_panic]
    fn test_fload_0_unimplemented() { let _ = fload_0(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_fload_1_unimplemented() { let _ = fload_1(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_fload_2_unimplemented() { let _ = fload_2(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_fload_3_unimplemented() { let _ = fload_3(&mut make_thread()); }

    #[test]
    #[should_panic]
    fn test_dload_0_unimplemented() { let _ = dload_0(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_dload_1_unimplemented() { let _ = dload_1(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_dload_2_unimplemented() { let _ = dload_2(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_dload_3_unimplemented() { let _ = dload_3(&mut make_thread()); }

    #[test]
    #[should_panic]
    fn test_aload_0_unimplemented() { let _ = aload_0(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_aload_1_unimplemented() { let _ = aload_1(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_aload_2_unimplemented() { let _ = aload_2(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_aload_3_unimplemented() { let _ = aload_3(&mut make_thread()); }

    // ── 0x2e–0x35  array loads ────────────────────────────────────────────────

    #[test]
    fn test_iaload_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        t.thread_stack.push_int(0);
        assert!(matches!(iaload(&mut t), Err(InstructionError::NullPointerException)));
    }
    #[test]
    #[should_panic]
    fn test_iaload_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_int(0);
        let _ = iaload(&mut t);
    }

    #[test]
    fn test_laload_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        t.thread_stack.push_int(0);
        assert!(matches!(laload(&mut t), Err(InstructionError::NullPointerException)));
    }
    #[test]
    #[should_panic]
    fn test_laload_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_int(0);
        let _ = laload(&mut t);
    }

    #[test]
    fn test_faload_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        t.thread_stack.push_int(0);
        assert!(matches!(faload(&mut t), Err(InstructionError::NullPointerException)));
    }
    #[test]
    #[should_panic]
    fn test_faload_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_int(0);
        let _ = faload(&mut t);
    }

    #[test]
    fn test_daload_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        t.thread_stack.push_int(0);
        assert!(matches!(daload(&mut t), Err(InstructionError::NullPointerException)));
    }
    #[test]
    #[should_panic]
    fn test_daload_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_int(0);
        let _ = daload(&mut t);
    }

    #[test]
    fn test_aaload_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        t.thread_stack.push_int(0);
        assert!(matches!(aaload(&mut t), Err(InstructionError::NullPointerException)));
    }
    #[test]
    #[should_panic]
    fn test_aaload_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_int(0);
        let _ = aaload(&mut t);
    }

    #[test]
    fn test_baload_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        t.thread_stack.push_int(0);
        assert!(matches!(baload(&mut t), Err(InstructionError::NullPointerException)));
    }
    #[test]
    #[should_panic]
    fn test_baload_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_int(0);
        let _ = baload(&mut t);
    }

    #[test]
    fn test_caload_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        t.thread_stack.push_int(0);
        assert!(matches!(caload(&mut t), Err(InstructionError::NullPointerException)));
    }
    #[test]
    #[should_panic]
    fn test_caload_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_int(0);
        let _ = caload(&mut t);
    }

    #[test]
    fn test_saload_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        t.thread_stack.push_int(0);
        assert!(matches!(saload(&mut t), Err(InstructionError::NullPointerException)));
    }
    #[test]
    #[should_panic]
    fn test_saload_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_int(0);
        let _ = saload(&mut t);
    }

    // ── 0x36–0x3a  generic stores ─────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_istore_unimplemented() { let _ = istore(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_lstore_unimplemented() { let _ = lstore(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_fstore_unimplemented() { let _ = fstore(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_dstore_unimplemented() { let _ = dstore(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_astore_unimplemented() { let _ = astore(&mut make_thread()); }

    // ── 0x3b–0x4e  indexed stores ─────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_istore_0_unimplemented() { let _ = istore_0(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_istore_1_unimplemented() { let _ = istore_1(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_istore_2_unimplemented() { let _ = istore_2(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_istore_3_unimplemented() { let _ = istore_3(&mut make_thread()); }

    #[test]
    #[should_panic]
    fn test_lstore_0_unimplemented() { let _ = lstore_0(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_lstore_1_unimplemented() { let _ = lstore_1(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_lstore_2_unimplemented() { let _ = lstore_2(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_lstore_3_unimplemented() { let _ = lstore_3(&mut make_thread()); }

    #[test]
    #[should_panic]
    fn test_fstore_0_unimplemented() { let _ = fstore_0(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_fstore_1_unimplemented() { let _ = fstore_1(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_fstore_2_unimplemented() { let _ = fstore_2(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_fstore_3_unimplemented() { let _ = fstore_3(&mut make_thread()); }

    #[test]
    #[should_panic]
    fn test_dstore_0_unimplemented() { let _ = dstore_0(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_dstore_1_unimplemented() { let _ = dstore_1(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_dstore_2_unimplemented() { let _ = dstore_2(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_dstore_3_unimplemented() { let _ = dstore_3(&mut make_thread()); }

    #[test]
    #[should_panic]
    fn test_astore_0_unimplemented() { let _ = astore_0(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_astore_1_unimplemented() { let _ = astore_1(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_astore_2_unimplemented() { let _ = astore_2(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_astore_3_unimplemented() { let _ = astore_3(&mut make_thread()); }

    // ── 0x4f–0x56  array stores ───────────────────────────────────────────────

    #[test]
    fn test_iastore_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        t.thread_stack.push_int(0);
        t.thread_stack.push_int(42);
        assert!(matches!(iastore(&mut t), Err(InstructionError::NullPointerException)));
    }
    #[test]
    #[should_panic]
    fn test_iastore_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_int(0);
        t.thread_stack.push_int(42);
        let _ = iastore(&mut t);
    }

    #[test]
    fn test_lastore_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        t.thread_stack.push_int(0);
        t.thread_stack.push_long(42);
        assert!(matches!(lastore(&mut t), Err(InstructionError::NullPointerException)));
    }
    #[test]
    #[should_panic]
    fn test_lastore_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_int(0);
        t.thread_stack.push_long(42);
        let _ = lastore(&mut t);
    }

    #[test]
    fn test_fastore_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        t.thread_stack.push_int(0);
        t.thread_stack.push_float(1.0);
        assert!(matches!(fastore(&mut t), Err(InstructionError::NullPointerException)));
    }
    #[test]
    #[should_panic]
    fn test_fastore_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_int(0);
        t.thread_stack.push_float(1.0);
        let _ = fastore(&mut t);
    }

    #[test]
    fn test_dastore_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        t.thread_stack.push_int(0);
        t.thread_stack.push_double(1.0);
        assert!(matches!(dastore(&mut t), Err(InstructionError::NullPointerException)));
    }
    #[test]
    #[should_panic]
    fn test_dastore_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_int(0);
        t.thread_stack.push_double(1.0);
        let _ = dastore(&mut t);
    }

    #[test]
    fn test_aastore_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        t.thread_stack.push_int(0);
        t.thread_stack.push_ref(5);
        assert!(matches!(aastore(&mut t), Err(InstructionError::NullPointerException)));
    }
    #[test]
    #[should_panic]
    fn test_aastore_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_int(0);
        t.thread_stack.push_ref(5);
        let _ = aastore(&mut t);
    }

    #[test]
    fn test_bastore_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        t.thread_stack.push_int(0);
        t.thread_stack.push_int(1);
        assert!(matches!(bastore(&mut t), Err(InstructionError::NullPointerException)));
    }
    #[test]
    #[should_panic]
    fn test_bastore_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_int(0);
        t.thread_stack.push_int(1);
        let _ = bastore(&mut t);
    }

    #[test]
    fn test_castore_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        t.thread_stack.push_int(0);
        t.thread_stack.push_int(65);
        assert!(matches!(castore(&mut t), Err(InstructionError::NullPointerException)));
    }
    #[test]
    #[should_panic]
    fn test_castore_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_int(0);
        t.thread_stack.push_int(65);
        let _ = castore(&mut t);
    }

    #[test]
    fn test_sastore_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        t.thread_stack.push_int(0);
        t.thread_stack.push_int(100);
        assert!(matches!(sastore(&mut t), Err(InstructionError::NullPointerException)));
    }
    #[test]
    #[should_panic]
    fn test_sastore_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_int(0);
        t.thread_stack.push_int(100);
        let _ = sastore(&mut t);
    }

    // ── 0x57  pop ─────────────────────────────────────────────────────────────

    #[test]
    fn test_pop_removes_top_short() {
        let mut t = make_thread();
        t.thread_stack.push_short(42);
        assert!(pop(&mut t).is_ok());
        assert!(t.thread_stack.pop_short().is_none());
    }

    #[test]
    fn test_pop_empty_stack_returns_internal_error() {
        let mut t = make_thread();
        assert!(matches!(pop(&mut t), Err(InstructionError::InternalError)));
    }

    // ── 0x58  pop2 ────────────────────────────────────────────────────────────

    #[test]
    fn test_pop2_removes_two_shorts() {
        let mut t = make_thread();
        t.thread_stack.push_short(1);
        t.thread_stack.push_short(2);
        assert!(pop2(&mut t).is_ok());
        assert!(t.thread_stack.pop_short().is_none());
    }

    #[test]
    fn test_pop2_empty_stack_returns_internal_error() {
        let mut t = make_thread();
        assert!(matches!(pop2(&mut t), Err(InstructionError::InternalError)));
    }

    // ── 0x59–0x5e  dup variants ───────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_dup_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        let _ = dup(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_dup_x1_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_ref(2);
        let _ = dup_x1(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_dup_x2_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_ref(2);
        t.thread_stack.push_ref(3);
        let _ = dup_x2(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_dup2_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_ref(2);
        let _ = dup2(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_dup2_x1_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_ref(2);
        t.thread_stack.push_ref(3);
        let _ = dup2_x1(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_dup2_x2_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        t.thread_stack.push_ref(2);
        t.thread_stack.push_ref(3);
        t.thread_stack.push_ref(4);
        let _ = dup2_x2(&mut t);
    }

    // ── 0x5f  swap ────────────────────────────────────────────────────────────

    #[test]
    fn test_swap_reverses_top_two_shorts() {
        let mut t = make_thread();
        t.thread_stack.push_short(10);
        t.thread_stack.push_short(20);
        assert!(swap(&mut t).is_ok());
        assert_eq!(t.thread_stack.pop_short().unwrap(), 10);
        assert_eq!(t.thread_stack.pop_short().unwrap(), 20);
    }

    #[test]
    fn test_swap_empty_stack_returns_internal_error() {
        let mut t = make_thread();
        assert!(matches!(swap(&mut t), Err(InstructionError::InternalError)));
    }

    // ── 0x60–0x63  add ────────────────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_iadd_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_int(1);
        t.thread_stack.push_int(2);
        let _ = iadd(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_ladd_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_long(1);
        t.thread_stack.push_long(2);
        let _ = ladd(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_fadd_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_float(1.0);
        t.thread_stack.push_float(2.0);
        let _ = fadd(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_dadd_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_double(1.0);
        t.thread_stack.push_double(2.0);
        let _ = dadd(&mut t);
    }

    // ── 0x64–0x67  sub ────────────────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_isub_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_int(5);
        t.thread_stack.push_int(3);
        let _ = isub(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_lsub_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_long(5);
        t.thread_stack.push_long(3);
        let _ = lsub(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_fsub_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_float(5.0);
        t.thread_stack.push_float(3.0);
        let _ = fsub(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_dsub_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_double(5.0);
        t.thread_stack.push_double(3.0);
        let _ = dsub(&mut t);
    }

    // ── 0x68–0x6b  mul ────────────────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_imul_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_int(3);
        t.thread_stack.push_int(4);
        let _ = imul(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_lmul_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_long(3);
        t.thread_stack.push_long(4);
        let _ = lmul(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_fmul_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_float(3.0);
        t.thread_stack.push_float(4.0);
        let _ = fmul(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_dmul_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_double(3.0);
        t.thread_stack.push_double(4.0);
        let _ = dmul(&mut t);
    }

    // ── 0x6c–0x6f  div ────────────────────────────────────────────────────────

    #[test]
    fn test_idiv_by_zero_returns_arithmetic_exception() {
        let mut t = make_thread();
        t.thread_stack.push_int(10);
        t.thread_stack.push_int(0);
        assert!(matches!(idiv(&mut t), Err(InstructionError::ArithmeticException)));
    }

    #[test]
    #[should_panic]
    fn test_idiv_nonzero_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_int(10);
        t.thread_stack.push_int(2);
        let _ = idiv(&mut t);
    }

    #[test]
    fn test_ldiv_by_zero_returns_arithmetic_exception() {
        let mut t = make_thread();
        t.thread_stack.push_long(10);
        t.thread_stack.push_long(0);
        assert!(matches!(ldiv(&mut t), Err(InstructionError::ArithmeticException)));
    }

    #[test]
    #[should_panic]
    fn test_ldiv_nonzero_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_long(10);
        t.thread_stack.push_long(2);
        let _ = ldiv(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_fdiv_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_float(10.0);
        t.thread_stack.push_float(2.0);
        let _ = fdiv(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_ddiv_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_double(10.0);
        t.thread_stack.push_double(2.0);
        let _ = ddiv(&mut t);
    }

    // ── 0x70–0x73  rem ────────────────────────────────────────────────────────

    #[test]
    fn test_irem_by_zero_returns_arithmetic_exception() {
        let mut t = make_thread();
        t.thread_stack.push_int(10);
        t.thread_stack.push_int(0);
        assert!(matches!(irem(&mut t), Err(InstructionError::ArithmeticException)));
    }

    #[test]
    #[should_panic]
    fn test_irem_nonzero_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_int(10);
        t.thread_stack.push_int(3);
        let _ = irem(&mut t);
    }

    #[test]
    fn test_lrem_by_zero_returns_arithmetic_exception() {
        let mut t = make_thread();
        t.thread_stack.push_long(10);
        t.thread_stack.push_long(0);
        assert!(matches!(lrem(&mut t), Err(InstructionError::ArithmeticException)));
    }

    #[test]
    #[should_panic]
    fn test_lrem_nonzero_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_long(10);
        t.thread_stack.push_long(3);
        let _ = lrem(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_frem_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_float(10.0);
        t.thread_stack.push_float(3.0);
        let _ = frem(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_drem_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_double(10.0);
        t.thread_stack.push_double(3.0);
        let _ = drem(&mut t);
    }

    // ── 0x74–0x77  neg ────────────────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_ineg_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_int(5);
        let _ = ineg(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_lneg_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_long(5);
        let _ = lneg(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_fneg_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_float(5.0);
        let _ = fneg(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_dneg_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_double(5.0);
        let _ = dneg(&mut t);
    }

    // ── 0x78–0x7d  shifts ─────────────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_ishl_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_int(1);
        t.thread_stack.push_int(2);
        let _ = ishl(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_lshl_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_long(1); // value1 (long)
        t.thread_stack.push_int(2);  // value2 (int shift amount)
        let _ = lshl(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_ishr_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_int(8);
        t.thread_stack.push_int(2);
        let _ = ishr(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_lshr_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_long(8); // value1 (long)
        t.thread_stack.push_int(2);  // value2 (int shift amount)
        let _ = lshr(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_iushr_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_int(8);
        t.thread_stack.push_int(2);
        let _ = iushr(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_lushr_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_long(8); // value1 (long)
        t.thread_stack.push_int(2);  // value2 (int shift amount)
        let _ = lushr(&mut t);
    }

    // ── 0x7e–0x83  bitwise ────────────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_iand_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_int(0b1010);
        t.thread_stack.push_int(0b1100);
        let _ = iand(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_land_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_long(0b1010);
        t.thread_stack.push_long(0b1100);
        let _ = land(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_ior_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_int(0b1010);
        t.thread_stack.push_int(0b1100);
        let _ = ior(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_lor_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_long(0b1010);
        t.thread_stack.push_long(0b1100);
        let _ = lor(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_ixor_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_int(0b1010);
        t.thread_stack.push_int(0b1100);
        let _ = ixor(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_lxor_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_long(0b1010);
        t.thread_stack.push_long(0b1100);
        let _ = lxor(&mut t);
    }

    // ── 0x84  iinc ────────────────────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_iinc_unimplemented() {
        let _ = iinc(&mut make_thread());
    }

    // ── 0x85–0x93  type conversions ───────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_i2l_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_int(5);
        let _ = i2l(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_i2f_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_int(5);
        let _ = i2f(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_i2d_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_int(5);
        let _ = i2d(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_l2i_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_long(5);
        let _ = l2i(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_l2f_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_long(5);
        let _ = l2f(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_l2d_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_long(5);
        let _ = l2d(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_f2i_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_float(5.0);
        let _ = f2i(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_f2l_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_float(5.0);
        let _ = f2l(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_f2d_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_float(5.0);
        let _ = f2d(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_d2i_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_double(5.0);
        let _ = d2i(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_d2l_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_double(5.0);
        let _ = d2l(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_d2f_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_double(5.0);
        let _ = d2f(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_i2b_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_int(5);
        let _ = i2b(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_i2c_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_int(65);
        let _ = i2c(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_i2s_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_int(5);
        let _ = i2s(&mut t);
    }

    // ── 0x94–0x98  comparisons ────────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_lcmp_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_long(1);
        t.thread_stack.push_long(2);
        let _ = lcmp(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_fcmpl_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_float(1.0);
        t.thread_stack.push_float(2.0);
        let _ = fcmpl(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_fcmpg_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_float(1.0);
        t.thread_stack.push_float(2.0);
        let _ = fcmpg(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_dcmpl_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_double(1.0);
        t.thread_stack.push_double(2.0);
        let _ = dcmpl(&mut t);
    }

    #[test]
    #[should_panic]
    fn test_dcmpg_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_double(1.0);
        t.thread_stack.push_double(2.0);
        let _ = dcmpg(&mut t);
    }

    // ── 0x99–0xa6  branches ───────────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_ifeq_unimplemented() { let _ = ifeq(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_ifne_unimplemented() { let _ = ifne(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_iflt_unimplemented() { let _ = iflt(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_ifge_unimplemented() { let _ = ifge(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_ifgt_unimplemented() { let _ = ifgt(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_ifle_unimplemented() { let _ = ifle(&mut make_thread()); }

    #[test]
    #[should_panic]
    fn test_if_icmpeq_unimplemented() { let _ = if_icmpeq(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_if_icmpne_unimplemented() { let _ = if_icmpne(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_if_icmplt_unimplemented() { let _ = if_icmplt(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_if_icmpge_unimplemented() { let _ = if_icmpge(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_if_icmpgt_unimplemented() { let _ = if_icmpgt(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_if_icmple_unimplemented() { let _ = if_icmple(&mut make_thread()); }

    #[test]
    #[should_panic]
    fn test_if_acmpeq_unimplemented() { let _ = if_acmpeq(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_if_acmpne_unimplemented() { let _ = if_acmpne(&mut make_thread()); }

    // ── 0xa7–0xa9  control flow ───────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_goto_unimplemented() { let _ = goto(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_jsr_unimplemented() { let _ = jsr(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_ret_unimplemented() { let _ = ret(&mut make_thread()); }

    // ── 0xaa–0xab  switch ─────────────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_tableswitch_unimplemented() { let _ = tableswitch(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_lookupswitch_unimplemented() { let _ = lookupswitch(&mut make_thread()); }

    // ── 0xac–0xb1  return ─────────────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_ireturn_unimplemented() { let _ = ireturn(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_lreturn_unimplemented() { let _ = lreturn(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_freturn_unimplemented() { let _ = freturn(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_dreturn_unimplemented() { let _ = dreturn(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_areturn_unimplemented() { let _ = areturn(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_return_unimplemented() { let _ = return_(&mut make_thread()); }

    // ── 0xb2–0xb5  field access ───────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_getstatic_unimplemented() { let _ = getstatic(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_putstatic_unimplemented() { let _ = putstatic(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_getfield_unimplemented() { let _ = getfield(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_putfield_unimplemented() { let _ = putfield(&mut make_thread()); }

    // ── 0xb6–0xba  invokes ────────────────────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_invokevirtual_unimplemented() { let _ = invokevirtual(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_invokespecial_unimplemented() { let _ = invokespecial(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_invokestatic_unimplemented() { let _ = invokestatic(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_invokeinterface_unimplemented() { let _ = invokeinterface(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_invokedynamic_unimplemented() { let _ = invokedynamic(&mut make_thread()); }

    // ── 0xbb–0xbd  array/object creation ─────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_new_unimplemented() { let _ = new(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_newarray_unimplemented() { let _ = newarray(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_anewarray_unimplemented() { let _ = anewarray(&mut make_thread()); }

    // ── 0xbe  arraylength ─────────────────────────────────────────────────────

    #[test]
    fn test_arraylength_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        assert!(matches!(arraylength(&mut t), Err(InstructionError::NullPointerException)));
    }

    #[test]
    #[should_panic]
    fn test_arraylength_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        let _ = arraylength(&mut t);
    }

    // ── 0xbf  athrow ──────────────────────────────────────────────────────────

    #[test]
    fn test_athrow_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        assert!(matches!(athrow(&mut t), Err(InstructionError::NullPointerException)));
    }

    #[test]
    #[should_panic]
    fn test_athrow_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        let _ = athrow(&mut t);
    }

    // ── 0xc0–0xc1  checkcast / instanceof ────────────────────────────────────

    #[test]
    #[should_panic]
    fn test_checkcast_unimplemented() { let _ = checkcast(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_instanceof_unimplemented() { let _ = instanceof(&mut make_thread()); }

    // ── 0xc2–0xc3  monitor ────────────────────────────────────────────────────

    #[test]
    fn test_monitorenter_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        assert!(matches!(monitorenter(&mut t), Err(InstructionError::NullPointerException)));
    }

    #[test]
    #[should_panic]
    fn test_monitorenter_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        let _ = monitorenter(&mut t);
    }

    #[test]
    fn test_monitorexit_null_ref_throws() {
        let mut t = make_thread();
        t.thread_stack.push_ref(0);
        assert!(matches!(monitorexit(&mut t), Err(InstructionError::NullPointerException)));
    }

    #[test]
    #[should_panic]
    fn test_monitorexit_unimplemented() {
        let mut t = make_thread();
        t.thread_stack.push_ref(1);
        let _ = monitorexit(&mut t);
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

    #[test]
    #[should_panic]
    fn test_multianewarray_unimplemented() { let _ = multianewarray(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_ifnull_unimplemented() { let _ = ifnull(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_ifnonnull_unimplemented() { let _ = ifnonnull(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_goto_w_unimplemented() { let _ = goto_w(&mut make_thread()); }
    #[test]
    #[should_panic]
    fn test_jsr_w_unimplemented() { let _ = jsr_w(&mut make_thread()); }

    // ── 0xca–0xff  reserved ───────────────────────────────────────────────────

    #[test]
    fn test_reserved_opcode_returns_unknown_error() {
        let mut t = make_thread();
        assert!(matches!(call_by_opcode(&mut t, 0xca), Err(InstructionError::UnknownError)));
        assert!(matches!(call_by_opcode(&mut t, 0xff), Err(InstructionError::UnknownError)));
    }
}
