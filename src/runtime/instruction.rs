use crate::runtime::thread::Thread;
use crate::jvm_error::JVMError;

use anyhow::Ok;
use anyhow::Result;

pub fn preform_instruction(thread: &mut Thread) -> Result<()> {
    let opcode = thread.current_frame.read_byte_from_pc()?;

    if let Err(e) = call_by_opcode(thread, opcode) {
        eprintln!("Error executing instruction with opcode {:#x}: {}", opcode, e);
    }

    Ok(())
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
        0xca..=0xff => { Err(JVMError::UnknownError.into()) },


    }
}


fn aaload(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn aastore(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn aconst_null(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn aload(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn aload_0(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn aload_1(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn aload_2(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn aload_3(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn anewarray(thread: &mut Thread) -> Result<()> {
    todo!("Implement anewarray instruction");
}

fn areturn(thread: &mut Thread) -> Result<()> {

    todo!("Implement areturn instruction");

}

fn arraylength(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn astore(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn astore_0(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn astore_1(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn astore_2(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn astore_3(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn athrow(thread: &mut Thread) -> Result<()> {
    todo!("Implement athrow instruction");
}

fn baload(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn bastore(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")

}

fn bipush(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn caload(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn castore(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn checkcast(thread: &mut Thread) -> Result<()> {
    todo!("Implement checkcast instruction");
}

fn d2f(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn d2i(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn d2l(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dadd(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn daload(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")

}

fn dastore(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dcmpg(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dcmpl(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dconst_0(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dconst_1(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn ddiv(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dload(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dload_0(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dload_1(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dload_2(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dload_3(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dmul(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dneg(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn drem(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dreturn(thread: &mut Thread) -> Result<()> {

    todo!("Implement dreturn instruction");

}

fn dstore(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dstore_0(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dstore_1(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dstore_2(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dstore_3(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dsub(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dup(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dup_x1(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dup_x2(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dup2(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dup2_x1(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn dup2_x2(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn f2d(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn f2i(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn f2l(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fadd(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn faload(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fastore(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fcmpg(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fcmpl(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fconst_0(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fconst_1(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fconst_2(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fdiv(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fload(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fload_0(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fload_1(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fload_2(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fload_3(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fmul(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fneg(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn frem(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn freturn(thread: &mut Thread) -> Result<()> {

    todo!("Implement freturn instruction");

}

fn fstore(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fstore_0(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fstore_1(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fstore_2(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fstore_3(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn fsub(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn getfield(thread: &mut Thread) -> Result<()> {
    todo!("Implement getfield instruction");
}

fn getstatic(thread: &mut Thread) -> Result<()> {
    todo!("Implement getstatic instruction");
}

fn goto(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn goto_w(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn i2b(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn i2c(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn i2d(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn i2f(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn i2l(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn i2s(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iadd(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iaload(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iand(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iastore(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iconst_m1(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iconst_0(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iconst_1(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iconst_2(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iconst_3(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iconst_4(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iconst_5(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn idiv(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn if_acmpeq(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn if_acmpne(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn if_icmpeq(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn if_icmpne(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn if_icmplt(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn if_icmpge(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn if_icmpgt(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn if_icmple(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn ifeq(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn ifne(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iflt(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn ifge(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn ifgt(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn ifle(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn ifnonnull(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn ifnull(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iinc(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iload(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iload_0(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iload_1(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iload_2(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iload_3(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn imul(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn ineg(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn instanceof(thread: &mut Thread) -> Result<()> {
    todo!("Implement instanceof instruction");
}

fn invokedynamic(thread: &mut Thread) -> Result<()> {
    todo!("Implement invokedynamic instruction");
}

fn invokeinterface(thread: &mut Thread) -> Result<()> {
    todo!("Implement invokeinterface instruction");
}

fn invokespecial(thread: &mut Thread) -> Result<()> {
    todo!("Implement invokespecial instruction");
}

fn invokestatic(thread: &mut Thread) -> Result<()> {
    todo!("Implement invokestatic instruction");
}

fn invokevirtual(thread: &mut Thread) -> Result<()> {
    todo!("Implement invokevirtual instruction");
}

fn ior(thread: &mut Thread) -> Result<()> {
        todo!("Not impled")
}

fn irem(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn ireturn(thread: &mut Thread) -> Result<()> {

    todo!("Implement ireturn instruction");

}

fn ishl(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn ishr(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn istore(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn istore_0(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn istore_1(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn istore_2(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn istore_3(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn isub(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn iushr(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn ixor(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn jsr(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn jsr_w(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn l2d(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn l2f(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn l2i(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn ladd(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn laload(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn land(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lastore(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lcmp(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lconst_0(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lconst_1(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn ldc(thread: &mut Thread) -> Result<()> {

    todo!("Implement ldc instruction");
}

fn ldc_w(thread: &mut Thread) -> Result<()> {
    todo!("Implement ldc_w instruction");
}

fn ldc2_w(thread: &mut Thread) -> Result<()> {
    todo!("Implement ldc2_w instruction");
}

fn ldiv(thread: &mut Thread) -> Result<()> {

    todo!("Not impled")
}

fn lload(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lload_0(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lload_1(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lload_2(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lload_3(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lmul(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lneg(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lookupswitch(thread: &mut Thread) -> Result<()> {
    todo!("Implement lookupswitch instruction");
}

fn lor(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lrem(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lreturn(thread: &mut Thread) -> Result<()> {

    todo!("Implement lreturn instruction");

}

fn lshl(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lshr(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lstore(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lstore_0(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lstore_1(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lstore_2(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lstore_3(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lsub(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lushr(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn lxor(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn monitorenter(thread: &mut Thread) -> Result<()> {
    todo!("Implement monitorenter instruction");
}

fn monitorexit(thread: &mut Thread) -> Result<()> {
    todo!("Implement monitorexit instruction");
}

fn multianewarray(thread: &mut Thread) -> Result<()> {
    todo!("Implement multianewarray instruction");
}

fn new(thread: &mut Thread) -> Result<()> {
    todo!("Implement new instruction");
}

fn newarray(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn nop(_: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn pop(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn pop2(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn putfield(thread: &mut Thread) -> Result<()> {
    todo!("Implement putfield instruction");
}

fn putstatic(thread: &mut Thread) -> Result<()> {
    todo!("Implement putstatic instruction");
}

fn ret(thread: &mut Thread) -> Result<()> {
    todo!("Implement ret instruction");
}

fn return_(thread: &mut Thread) -> Result<()> {

    todo!("Implement return instruction");

}

fn saload(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn sastore(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn sipush(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn swap(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}

fn tableswitch(thread: &mut Thread) -> Result<()> {
    todo!("Implement tableswitch instruction");
}

fn wide(thread: &mut Thread) -> Result<()> {
    todo!("Not impled")
}