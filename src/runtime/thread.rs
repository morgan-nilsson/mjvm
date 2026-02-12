use std::sync::{Arc, Mutex};

pub struct Thread {
    pc: usize,
    thread_stack: ThreadStack,
    jvm_heap: Arc<Mutex<JVMHeap>>,
    method_area: Arc<Mutex<MethodArea>>,
    native_method_stack: NativeMethodStack,
}

pub struct ThreadStack {

}

pub struct JVMHeap {

}

pub struct MethodArea {

}

pub struct NativeMethodStack {

}