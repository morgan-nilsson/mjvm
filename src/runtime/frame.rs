use crate::runtime::runtime_const_pool::RuntimeConstantPool;

pub struct Frame {
    local_variables: Vec<LocalVariable>,
    operand_stack: Vec<Operand>,
    runtime_constant_pool: Vec<RuntimeConstantPool>,
}

pub struct LocalVariable {

}

pub struct Operand {

}