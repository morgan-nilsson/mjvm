use crate::jvm_error::JVMError;
use crate::runtime::frame::Frame;

static THREAD_STACK_INITIAL_FRAME_COUNT: usize = 32;

pub struct ThreadStack {
    frames: Vec<Frame>,
    max_frames: usize,
}


impl ThreadStack {
    pub fn new(max_frames: usize) -> Self {
        Self {
            frames: Vec::with_capacity(THREAD_STACK_INITIAL_FRAME_COUNT),
            max_frames,
        }
    }

    pub fn push(&mut self, frame: Frame) -> Result<(), JVMError> {
        if self.frames.len() >= self.max_frames {
            return Err(JVMError::StackOverflowError.into());
        }
        self.frames.push(frame);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Frame, JVMError> {
        self.frames.pop().ok_or(JVMError::EmptyStack.into())
    }

    pub fn current_frame(&self) -> Result<&Frame, JVMError> {
        self.frames.last().ok_or(JVMError::EmptyStack.into())
    }

    pub fn current_frame_mut(&mut self) -> Result<&mut Frame, JVMError> {
        self.frames.last_mut().ok_or(JVMError::EmptyStack.into())
    }

    pub fn len(&self) -> usize {
        self.frames.len()
    }

    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
}