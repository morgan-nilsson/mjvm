use std::sync::{Arc, Mutex};

use crate::runtime::frame::Frame;

pub mod thread_stack;
use thread_stack::ThreadStack;

pub mod jvm_heap;
use jvm_heap::JVMHeap;

pub mod method_area;
use method_area::MethodArea;

pub mod native_method_stack;
use native_method_stack::NativeMethodStack;

pub struct Thread {
    pub current_frame: Frame,
    pub thread_stack: ThreadStack,
    pub jvm_heap: Arc<Mutex<JVMHeap>>,
    pub method_area: Arc<Mutex<MethodArea>>,
    pub native_method_stack: NativeMethodStack,
}