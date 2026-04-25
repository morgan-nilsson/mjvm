
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JVMError {
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

    #[error("Attempt to pop from an empty stack during instruction execution")]
    EmptyStack,

    #[error("A type mismatch occurred in local variable access during instruction execution")]
    LocalVarTypeMismatch,

    #[error("Local variable index out of bounds during instruction execution")]
    LocalVarOutOfBounds,

    #[error("An invalid reference was encountered during instruction execution")]
    InvalidReference,

    #[error("An attempt was made to invoke a method on a null reference during instruction execution")]
    NotAnInstance,
}