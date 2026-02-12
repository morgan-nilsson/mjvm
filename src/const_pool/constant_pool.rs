pub struct RunTimeConstantPool {

}

pub enum ConstantPoolEntry {
    SymbolicReference {
    },
    StaticConstant {
    },
}

use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ConstantPoolInfoTag {
    ConstantClass = 7,
    ConstantFieldRef = 9,
    ConstantMethodRef = 10,
    ConstantInterfaceMethodRef = 11,
    ConstantString = 8,
    ConstantInteger = 3,
    ConstantFloat = 4,
    ConstantLong = 5,
    ConstantDouble = 6,
    ConstantNameAndType = 12,
    ConstantUtf8 = 1,
    ConstantMethodHandle = 15,
    ConstantMethodType = 16,
    ConstantDynamic = 17,
    ConstantInvokeDynamic = 18,
    ConstantModule = 19,
    ConstantPackage = 20,

}

impl TryFrom<u8> for ConstantPoolInfoTag {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            7 => Ok(ConstantPoolInfoTag::ConstantClass),
            9 => Ok(ConstantPoolInfoTag::ConstantFieldRef),
            10 => Ok(ConstantPoolInfoTag::ConstantMethodRef),
            11 => Ok(ConstantPoolInfoTag::ConstantInterfaceMethodRef),
            8 => Ok(ConstantPoolInfoTag::ConstantString),
            3 => Ok(ConstantPoolInfoTag::ConstantInteger),
            4 => Ok(ConstantPoolInfoTag::ConstantFloat),
            5 => Ok(ConstantPoolInfoTag::ConstantLong),
            6 => Ok(ConstantPoolInfoTag::ConstantDouble),
            12 => Ok(ConstantPoolInfoTag::ConstantNameAndType),
            1 => Ok(ConstantPoolInfoTag::ConstantUtf8),
            15 => Ok(ConstantPoolInfoTag::ConstantMethodHandle),
            16 => Ok(ConstantPoolInfoTag::ConstantMethodType),
            17 => Ok(ConstantPoolInfoTag::ConstantDynamic),
            18 => Ok(ConstantPoolInfoTag::ConstantInvokeDynamic),
            19 => Ok(ConstantPoolInfoTag::ConstantModule),
            20 => Ok(ConstantPoolInfoTag::ConstantPackage),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum ConstantPoolInfo {
    ConstantClass { name_index: u16 },
    ConstantFieldRef { class_index: u16, name_and_type_index: u16 },
    ConstantMethodRef { class_index: u16, name_and_type_index: u16 },
    ConstantInterfaceMethodRef { class_index: u16, name_and_type_index: u16 },
    ConstantString { string_index: u16 },
    ConstantInteger { bytes: u32 },
    ConstantFloat { bytes: u32 },
    ConstantLong { high_bytes: u32, low_bytes: u32 },
    ConstantDouble { high_bytes: u32, low_bytes: u32 },
    ConstantNameAndType { name_index: u16, descriptor_index: u16 },
    ConstantUtf8 { length: u16, bytes: Vec<u8> },
    ConstantMethodHandle { reference_kind: u8, reference_index: u16 },
    ConstantMethodType { descriptor_index: u16 },
    ConstantDynamic { bootstrap_method_attr_index: u16, name_and_type_index: u16 },
    ConstantInvokeDynamic { bootstrap_method_attr_index: u16, name_and_type_index: u16 },
    ConstantModule { name_index: u16 },
    ConstantPackage { name_index: u16 },
}