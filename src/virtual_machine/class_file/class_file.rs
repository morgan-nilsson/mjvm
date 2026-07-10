use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use anyhow::Ok;
use bitflags::bitflags;
use anyhow::Result;
use anyhow::anyhow;
use crate::virtual_machine::jvm_error::JVMError;

#[derive(Debug)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<ConstantPoolInfo>,
    pub access_flags: AccessFlags,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<FieldInfo>,
    pub methods_count: u16,
    pub methods: Vec<ConstantPoolMethodInfo>,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        let mut buf_reader = BufReader::new(reader);

        // magic number
        let mut magic_buf = [0; 4];
        buf_reader.read_exact(&mut magic_buf).expect("Failed to read magic number from class file");
        let magic = u32::from_be_bytes(magic_buf);

        // minor version
        let mut minor_version_buf = [0; 2];
        buf_reader.read_exact(&mut minor_version_buf).expect("Failed to read minor version from class file");
        let minor_version = u16::from_be_bytes(minor_version_buf);

        // major version
        let mut major_version_buf = [0; 2];
        buf_reader.read_exact(&mut major_version_buf).expect("Failed to read major version from class file");
        let major_version = u16::from_be_bytes(major_version_buf);

        // constant pool count
        let mut constant_pool_count_buf = [0; 2];
        buf_reader.read_exact(&mut constant_pool_count_buf).expect("Failed to read constant pool count from class file");
        let constant_pool_count = u16::from_be_bytes(constant_pool_count_buf) - 1; // constant pool count is 1-based, so we subtract 1 to get the actual number of entries

        // constant pool
        let mut constant_pool = Vec::with_capacity(constant_pool_count as usize);

        let mut remaining = constant_pool_count as usize;
        while remaining > 0 {
            let cp_info = ConstantPoolInfo::from_reader(&mut buf_reader)?;
            let is_wide = matches!(cp_info,
                ConstantPoolInfo::ConstantLong { .. } | ConstantPoolInfo::ConstantDouble { .. });
            constant_pool.push(cp_info);
            remaining -= 1;
            if is_wide && remaining > 0 {
                constant_pool.push(ConstantPoolInfo::LongDoublePhantom);
                remaining -= 1;
            }
        }

        // access flags
        let mut access_flags_buf = [0; 2];
        buf_reader.read_exact(&mut access_flags_buf).expect("Failed to read access flags from class file");
        let access_flags = AccessFlags::from_bits(u16::from_be_bytes(access_flags_buf)).expect("Invalid access flags in class file");

        // this class
        let mut this_class_buf = [0; 2];
        buf_reader.read_exact(&mut this_class_buf).expect("Failed to read this class index from class file");
        let this_class = u16::from_be_bytes(this_class_buf);

        // super class
        let mut super_class_buf = [0; 2];
        buf_reader.read_exact(&mut super_class_buf).expect("Failed to read super class index from class file");
        let super_class = u16::from_be_bytes(super_class_buf);

        // interfaces count
        let mut interfaces_count_buf = [0; 2];
        buf_reader.read_exact(&mut interfaces_count_buf).expect("Failed to read interfaces count from class file");
        let interfaces_count = u16::from_be_bytes(interfaces_count_buf);

        // interfaces
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);

        for _ in 0..interfaces_count {
            let mut interface_buf = [0; 2];
            buf_reader.read_exact(&mut interface_buf).expect("Failed to read interface index from class file");
            let interface = u16::from_be_bytes(interface_buf);
            interfaces.push(interface);
        }

        // fields count
        let mut fields_count_buf = [0; 2];
        buf_reader.read_exact(&mut fields_count_buf).expect("Failed to read fields count from class file");
        let fields_count = u16::from_be_bytes(fields_count_buf);

        // fields
        let mut fields = Vec::with_capacity(fields_count as usize);

        for _ in 0..fields_count {
            let field_info = FieldInfo::from_reader(&mut buf_reader, &constant_pool)?;
            fields.push(field_info);
        }

        // methods count
        let mut methods_count_buf = [0; 2];
        buf_reader.read_exact(&mut methods_count_buf).expect("Failed to read methods count from class file");
        let methods_count = u16::from_be_bytes(methods_count_buf);

        // methods
        let mut methods = Vec::with_capacity(methods_count as usize);

        for _ in 0..methods_count {
            let method_info = ConstantPoolMethodInfo::from_reader(&mut buf_reader, &constant_pool)?;
            methods.push(method_info);
        }

        // attributes count
        let mut attributes_count_buf = [0; 2];
        buf_reader.read_exact(&mut attributes_count_buf).expect("Failed to read attributes count from class file");
        let attributes_count = u16::from_be_bytes(attributes_count_buf);

        // attributes
        let mut attributes = Vec::with_capacity(attributes_count as usize);

        for _ in 0..attributes_count {
            let attribute_info = AttributeInfo::from_reader(&mut buf_reader, &constant_pool)?;
            attributes.push(attribute_info);
        }

        // reader must be fully consumed at this point, otherwise the class file is malformed
        let mut leftover_buf = [0; 1];
        if buf_reader.read(&mut leftover_buf)? != 0 {
            return Err(anyhow!("Class file has extra data after expected end of file"));
        }

        Ok(ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces,
            fields_count,
            fields,
            methods_count,
            methods,
            attributes_count,
            attributes,
        })
    }

    fn ensure_class_file_validity(&self) {
        todo!("Validity not checked")
    }
}


#[derive(Debug)]
pub struct ConstantPool {
    constant_pool_info: Vec<ConstantPoolInfo>,
}

impl ConstantPool {
    pub fn new(constant_pool_info: Vec<ConstantPoolInfo>) -> Self {
        Self { constant_pool_info }
    }

    pub fn get(&self, index: usize) -> Option<&ConstantPoolInfo> {
        self.constant_pool_info.get(index)
    }

    pub fn len(&self) -> usize {
        self.constant_pool_info.len()
    }
}

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

impl ConstantPoolInfoTag {
    fn try_from(value: u8) -> Result<Self> {
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
            _ => Err(anyhow!("Invalid constant pool tag in class file: {}", value)),
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
    LongDoublePhantom,
}

impl ConstantPoolInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut tag_buf = [0; 1];
        buf_reader.read_exact(&mut tag_buf)?;
        let tag = ConstantPoolInfoTag::try_from(u8::from_be_bytes(tag_buf)).map_err(|_| anyhow!("Invalid constant pool tag in class file: {}", tag_buf[0]))?;

        match tag {
            ConstantPoolInfoTag::ConstantClass => {
                let mut name_index_buf = [0; 2];
                buf_reader.read_exact(&mut name_index_buf)?;
                let name_index = u16::from_be_bytes(name_index_buf);
                Ok(ConstantPoolInfo::ConstantClass { name_index })
            }

            ConstantPoolInfoTag::ConstantFieldRef => {
                let mut class_index_buf = [0; 2];
                buf_reader.read_exact(&mut class_index_buf)?;
                let class_index = u16::from_be_bytes(class_index_buf);

                let mut name_and_type_index_buf = [0; 2];
                buf_reader.read_exact(&mut name_and_type_index_buf)?;
                let name_and_type_index = u16::from_be_bytes(name_and_type_index_buf);

                Ok(ConstantPoolInfo::ConstantFieldRef { class_index, name_and_type_index })
            }
            ConstantPoolInfoTag::ConstantMethodRef => {
                let mut class_index_buf = [0; 2];
                buf_reader.read_exact(&mut class_index_buf)?;
                let class_index = u16::from_be_bytes(class_index_buf);

                let mut name_and_type_index_buf = [0; 2];
                buf_reader.read_exact(&mut name_and_type_index_buf)?;
                let name_and_type_index = u16::from_be_bytes(name_and_type_index_buf);

                Ok(ConstantPoolInfo::ConstantMethodRef { class_index, name_and_type_index })
            }
            ConstantPoolInfoTag::ConstantInterfaceMethodRef => {
                let mut class_index_buf = [0; 2];
                buf_reader.read_exact(&mut class_index_buf)?;
                let class_index = u16::from_be_bytes(class_index_buf);

                let mut name_and_type_index_buf = [0; 2];
                buf_reader.read_exact(&mut name_and_type_index_buf)?;
                let name_and_type_index = u16::from_be_bytes(name_and_type_index_buf);

                Ok(ConstantPoolInfo::ConstantInterfaceMethodRef { class_index, name_and_type_index })
            }

            ConstantPoolInfoTag::ConstantString => {
                let mut string_index_buf = [0; 2];
                buf_reader.read_exact(&mut string_index_buf)?;
                let string_index = u16::from_be_bytes(string_index_buf);
                Ok(ConstantPoolInfo::ConstantString { string_index })
            }

            ConstantPoolInfoTag::ConstantInteger => {
                let mut bytes_buf = [0; 4];
                buf_reader.read_exact(&mut bytes_buf)?;
                let bytes = u32::from_be_bytes(bytes_buf);
                Ok(ConstantPoolInfo::ConstantInteger { bytes })
            }

            ConstantPoolInfoTag::ConstantFloat => {
                let mut bytes_buf = [0; 4];
                buf_reader.read_exact(&mut bytes_buf)?;
                let bytes = u32::from_be_bytes(bytes_buf);
                Ok(ConstantPoolInfo::ConstantFloat { bytes })
            }

            ConstantPoolInfoTag::ConstantLong => {
                let mut high_bytes_buf = [0; 4];
                buf_reader.read_exact(&mut high_bytes_buf)?;
                let high_bytes = u32::from_be_bytes(high_bytes_buf);

                let mut low_bytes_buf = [0; 4];
                buf_reader.read_exact(&mut low_bytes_buf)?;
                let low_bytes = u32::from_be_bytes(low_bytes_buf);

                Ok(ConstantPoolInfo::ConstantLong { high_bytes, low_bytes })

            }

            ConstantPoolInfoTag::ConstantDouble => {
                let mut high_bytes_buf = [0; 4];
                buf_reader.read_exact(&mut high_bytes_buf)?;
                let high_bytes = u32::from_be_bytes(high_bytes_buf);

                let mut low_bytes_buf = [0; 4];
                buf_reader.read_exact(&mut low_bytes_buf)?;
                let low_bytes = u32::from_be_bytes(low_bytes_buf);

                Ok(ConstantPoolInfo::ConstantDouble { high_bytes, low_bytes })
            }

            ConstantPoolInfoTag::ConstantNameAndType => {
                let mut name_index_buf = [0; 2];
                buf_reader.read_exact(&mut name_index_buf)?;
                let name_index = u16::from_be_bytes(name_index_buf);

                let mut descriptor_index_buf = [0; 2];
                buf_reader.read_exact(&mut descriptor_index_buf)?;
                let descriptor_index = u16::from_be_bytes(descriptor_index_buf);

                Ok(ConstantPoolInfo::ConstantNameAndType { name_index, descriptor_index })
            }

            ConstantPoolInfoTag::ConstantUtf8 => {
                let mut length_buf = [0; 2];
                buf_reader.read_exact(&mut length_buf)?;
                let length = u16::from_be_bytes(length_buf);

                let mut bytes = vec![0; length as usize];
                buf_reader.read_exact(&mut bytes)?;

                Ok(ConstantPoolInfo::ConstantUtf8 { length, bytes })
            }

            ConstantPoolInfoTag::ConstantMethodHandle => {
                let mut reference_kind_buf = [0; 1];
                buf_reader.read_exact(&mut reference_kind_buf)?;
                let reference_kind = u8::from_be_bytes(reference_kind_buf);

                let mut reference_index_buf = [0; 2];
                buf_reader.read_exact(&mut reference_index_buf)?;
                let reference_index = u16::from_be_bytes(reference_index_buf);

                Ok(ConstantPoolInfo::ConstantMethodHandle { reference_kind, reference_index })
            }

            ConstantPoolInfoTag::ConstantMethodType => {
                let mut descriptor_index_buf = [0; 2];
                buf_reader.read_exact(&mut descriptor_index_buf)?;
                let descriptor_index = u16::from_be_bytes(descriptor_index_buf);

                Ok(ConstantPoolInfo::ConstantMethodType { descriptor_index })
            }

            ConstantPoolInfoTag::ConstantDynamic => {
                let mut bootstrap_method_attr_index_buf = [0; 2];
                buf_reader.read_exact(&mut bootstrap_method_attr_index_buf)?;
                let bootstrap_method_attr_index = u16::from_be_bytes(bootstrap_method_attr_index_buf);

                let mut name_and_type_index_buf = [0; 2];
                buf_reader.read_exact(&mut name_and_type_index_buf)?;
                let name_and_type_index = u16::from_be_bytes(name_and_type_index_buf);

                Ok(ConstantPoolInfo::ConstantDynamic { bootstrap_method_attr_index, name_and_type_index })
            }

            ConstantPoolInfoTag::ConstantInvokeDynamic => {
                let mut bootstrap_method_attr_index_buf = [0; 2];
                buf_reader.read_exact(&mut bootstrap_method_attr_index_buf)?;
                let bootstrap_method_attr_index = u16::from_be_bytes(bootstrap_method_attr_index_buf);

                let mut name_and_type_index_buf = [0; 2];
                buf_reader.read_exact(&mut name_and_type_index_buf)?;
                let name_and_type_index = u16::from_be_bytes(name_and_type_index_buf);

                Ok(ConstantPoolInfo::ConstantInvokeDynamic { bootstrap_method_attr_index, name_and_type_index })
            }

            ConstantPoolInfoTag::ConstantModule => {
                let mut name_index_buf = [0; 2];
                buf_reader.read_exact(&mut name_index_buf)?;
                let name_index = u16::from_be_bytes(name_index_buf);
                Ok(ConstantPoolInfo::ConstantModule { name_index })
            }

            ConstantPoolInfoTag::ConstantPackage => {
                let mut name_index_buf = [0; 2];
                buf_reader.read_exact(&mut name_index_buf)?;
                let name_index = u16::from_be_bytes(name_index_buf);
                Ok(ConstantPoolInfo::ConstantPackage { name_index })
            }
        }
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct AccessFlags: u16 {
        const Public = 0x0001;
        const Final = 0x0010;
        const Super = 0x0020;
        const Interface = 0x0200;
        const Abstract = 0x0400;
        const Synthetic = 0x1000;
        const Annotation = 0x2000;
        const Enum = 0x4000;
    }
}

#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: FieldInfoAccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl FieldInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R, constant_pool_entries: &Vec<ConstantPoolInfo>) -> Result<Self> {

        // access flags
        let mut access_flags_buf = [0; 2];
        buf_reader.read_exact(&mut access_flags_buf)?;
        let access_flags = FieldInfoAccessFlags::from_bits(u16::from_be_bytes(access_flags_buf));

        // name index
        let mut name_index_buf = [0; 2];
        buf_reader.read_exact(&mut name_index_buf)?;
        let name_index = u16::from_be_bytes(name_index_buf);

        // descriptor index
        let mut descriptor_index_buf = [0; 2];
        buf_reader.read_exact(&mut descriptor_index_buf)?;
        let descriptor_index = u16::from_be_bytes(descriptor_index_buf);

        // attributes count
        let mut attributes_count_buf = [0; 2];
        buf_reader.read_exact(&mut attributes_count_buf)?;
        let attributes_count = u16::from_be_bytes(attributes_count_buf);

        // attributes
        let mut attributes = Vec::with_capacity(attributes_count as usize);

        for _ in 0..attributes_count {
            let attribute_info = AttributeInfo::from_reader(buf_reader, constant_pool_entries)?;
            attributes.push(attribute_info);
        }

        Ok(FieldInfo {
            access_flags: access_flags.unwrap(),
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct FieldInfoAccessFlags: u16 {
        const Public = 0x0001;
        const Private = 0x0002;
        const Protected = 0x0004;
        const Static = 0x0008;
        const Final = 0x0010;
        const Volatile = 0x0040;
        const Transient = 0x0080;
        const Synthetic = 0x1000;
        const Enum = 0x4000;
    }
}

#[derive(Debug)]
pub struct ConstantPoolMethodInfo {
    pub access_flags: MethodAccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl ConstantPoolMethodInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R, constant_pool_info: &Vec<ConstantPoolInfo>) -> Result<Self> {

        // access flags
        let mut access_flags_buf = [0; 2];
        buf_reader.read_exact(&mut access_flags_buf)?;
        let access_flags = MethodAccessFlags::from_bits(u16::from_be_bytes(access_flags_buf));
        if access_flags.is_none() {
            return Err(anyhow!("Invalid method access flags in class file"));
        }

        // name index
        let mut name_index_buf = [0; 2];
        buf_reader.read_exact(&mut name_index_buf)?;
        let name_index = u16::from_be_bytes(name_index_buf);

        // descriptor index
        let mut descriptor_index_buf = [0; 2];
        buf_reader.read_exact(&mut descriptor_index_buf)?;
        let descriptor_index = u16::from_be_bytes(descriptor_index_buf);

        // attributes count
        let mut attributes_count_buf = [0; 2];
        buf_reader.read_exact(&mut attributes_count_buf)?;
        let attributes_count = u16::from_be_bytes(attributes_count_buf);

        // attributes
        let mut attributes = Vec::with_capacity(attributes_count as usize);

        for _ in 0..attributes_count {
            let attribute_info = AttributeInfo::from_reader(buf_reader, constant_pool_info)?;
            if matches!(attribute_info, AttributeInfo::UnknownAttribute) {
                // skip unknown attributes but do not include them in the method info
                continue;
            }
            attributes.push(attribute_info);
        }

        Ok(ConstantPoolMethodInfo {
            access_flags: access_flags.unwrap(),
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct MethodAccessFlags: u16 {
        const Public = 0x0001;
        const Private = 0x0002;
        const Protected = 0x0004;
        const Static = 0x0008;
        const Final = 0x0010;
        const Synchronized = 0x0020;
        const Bridge = 0x0040;
        const Varargs = 0x0080;
        const Native = 0x0100;
        const Abstract = 0x0400;
        const Strict = 0x0800;
        const Synthetic = 0x1000;
    }
}

#[derive(Debug)]
pub enum AttributeInfo {
    ConstantValue { constant_value_index: u16 },
    Code { max_stack: u16, max_locals: u16, code_length: u32, code: Vec<u8>, exception_table_length: u16, exception_table: Vec<ExceptionTableEntry>, attributes_count: u16, attributes: Vec<AttributeInfo> },
    StackMapTable { number_of_entries: u16, entries: Vec<StackMapFrame> },
    Exceptions { number_of_exceptions: u16, exception_index_table: Vec<u16> },
    InnerClasses { number_of_classes: u16, classes: Vec<InnerClassInfo> },
    EnclosingMethod { class_index: u16, method_index: u16 },
    Synthetic,
    Signature { signature_index: u16 },
    SourceFile { sourcefile_index: u16 },
    SourceDebugExtension { debug_extension: Vec<u8> },
    LineNumberTable { line_number_table_length: u16, line_number_table: Vec<LineNumberInfo> },
    LocalVariableTable { local_variable_table_length: u16, local_variable_table: Vec<LocalVariableInfo> },
    LocalVariableTypeTable { local_variable_type_table_length: u16, local_variable_type_table: Vec<LocalVariableTypeInfo> },
    Deprecated,
    RuntimeVisibleAnnotations { num_annotations: u16, annotations: Vec<Annotation> },
    RuntimeInvisibleAnnotations { num_annotations: u16, annotations: Vec<Annotation> },
    RuntimeVisibleParameterAnnotations { num_parameters: u8, parameter_annotations: Vec<ParameterAnnotations> },
    RuntimeInvisibleParameterAnnotations { num_parameters: u8, parameter_annotations: Vec<ParameterAnnotations> },
    RuntimeVisibleTypeAnnotations { num_annotations: u16, annotations: Vec<TypeAnnotation> },
    RuntimeInvisibleTypeAnnotations { num_annotations: u16, annotations: Vec<TypeAnnotation> },
    AnnotationDefault { default_value: ElementValue },
    BootstrapMethods { num_bootstrap_methods: u16, bootstrap_methods: Vec<BootstrapMethod> },
    MethodParameters { parameters_count: u8, parameters: Vec<MethodParameter> },
    Module { module_name_index: u16, module_flags: ModuleFlags, module_version_index: u16, requires_count: u16, requires: Vec<ModuleRequire>, exports_count: u16, exports: Vec<ModuleExport>, opens_count: u16, opens: Vec<ModuleOpen>, uses_count: u16, uses_index: Vec<u16>, provides_count: u16, provides: Vec<ModuleProvide> },
    ModulePackages { package_count: u16, package_index: Vec<u16> },
    ModuleMainClass { main_class_index: u16 },
    NestHost { host_class_index: u16 },
    NestMembers { number_of_classes: u16, classes: Vec<u16> },
    Record { number_of_components: u16, components: Vec<RecordComponentInfo> },
    PermittedSubclasses { number_of_classes: u16, classes: Vec<u16> },
    UnknownAttribute,
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct ModuleFlags: u16 {
        const OPEN = 0x0020;
        const SYNTHETIC = 0x1000;
        const MANDATED = 0x8000;
    }
}

impl AttributeInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R, constant_pool_info: &Vec<ConstantPoolInfo>) -> Result<Self> {
        let mut attribute_name_index_buf = [0; 2];
        buf_reader.read_exact(&mut attribute_name_index_buf)?;
        let attribute_name_index = u16::from_be_bytes(attribute_name_index_buf);
        let attribute_name = match &constant_pool_info[attribute_name_index as usize - 1] {
            ConstantPoolInfo::ConstantUtf8 { length: _, bytes } => String::from_utf8(bytes.clone()).expect("Invalid UTF-8 in attribute name in class file"),
            _ => panic!("Invalid constant pool entry for attribute name index in class file"),
        };

        let mut attribute_length_buf = [0; 4];
        buf_reader.read_exact(&mut attribute_length_buf)?;
        let attribute_length = u32::from_be_bytes(attribute_length_buf);

        match attribute_name.as_str() {
            "ConstantValue" => {
                let mut constant_value_index_buf = [0; 2];
                buf_reader.read_exact(&mut constant_value_index_buf)?;
                let constant_value_index = u16::from_be_bytes(constant_value_index_buf);
                Ok(AttributeInfo::ConstantValue { constant_value_index })
            }

            "Code" => {
                let mut max_stack_buf = [0; 2];
                buf_reader.read_exact(&mut max_stack_buf).expect("Failed to read max stack for Code attribute from class file");
                let max_stack = u16::from_be_bytes(max_stack_buf);

                let mut max_locals_buf = [0; 2];
                buf_reader.read_exact(&mut max_locals_buf).expect("Failed to read max locals for Code attribute from class file");
                let max_locals = u16::from_be_bytes(max_locals_buf);

                let mut code_length_buf = [0; 4];
                buf_reader.read_exact(&mut code_length_buf).expect("Failed to read code length for Code attribute from class file");
                let code_length = u32::from_be_bytes(code_length_buf);

                let mut code = vec![0; code_length as usize];
                buf_reader.read_exact(&mut code).expect("Failed to read code for Code attribute from class file");

                let mut exception_table_length_buf = [0; 2];
                buf_reader.read_exact(&mut exception_table_length_buf).expect("Failed to read exception table length for Code attribute from class file");
                let exception_table_length = u16::from_be_bytes(exception_table_length_buf);

                let mut exception_table = Vec::with_capacity(exception_table_length as usize);
                for _ in 0..exception_table_length {
                    let entry = ExceptionTableEntry::from_reader(buf_reader)?;
                    exception_table.push(entry);
                }

                let mut attributes_count_buf = [0; 2];
                buf_reader.read_exact(&mut attributes_count_buf).expect("Failed to read attributes count for Code attribute from class file");
                let attributes_count = u16::from_be_bytes(attributes_count_buf);

                let mut attributes = Vec::with_capacity(attributes_count as usize);
                for _ in 0..attributes_count {
                    let attribute_info = AttributeInfo::from_reader(buf_reader, constant_pool_info)?;
                    attributes.push(attribute_info);
                }

                Ok(AttributeInfo::Code { 
                    max_stack, 
                    max_locals, 
                    code_length, 
                    code, 
                    exception_table_length, 
                    exception_table, 
                    attributes_count, 
                    attributes 
                })
            }

            "StackMapTable" => {
                let mut number_of_entries_buf = [0; 2];
                buf_reader.read_exact(&mut number_of_entries_buf).expect("Failed to read number of entries for StackMapTable attribute from class file");
                let number_of_entries = u16::from_be_bytes(number_of_entries_buf);

                let mut entries = Vec::with_capacity(number_of_entries as usize);
                for _ in 0..number_of_entries {
                    let entry = StackMapFrame::from_reader(buf_reader)?;
                    entries.push(entry);
                }

                Ok(AttributeInfo::StackMapTable { number_of_entries, entries })
            }

            "Exceptions" => {
                let mut number_of_exceptions_buf = [0; 2];
                buf_reader.read_exact(&mut number_of_exceptions_buf).expect("Failed to read number of exceptions for Exceptions attribute from class file");
                let number_of_exceptions = u16::from_be_bytes(number_of_exceptions_buf);

                let mut exception_index_table = Vec::with_capacity(number_of_exceptions as usize);
                for _ in 0..number_of_exceptions {
                    let mut exception_index_buf = [0; 2];
                    buf_reader.read_exact(&mut exception_index_buf).expect("Failed to read exception index for Exceptions attribute from class file");
                    let exception_index = u16::from_be_bytes(exception_index_buf);
                    exception_index_table.push(exception_index);
                }

                Ok(AttributeInfo::Exceptions { number_of_exceptions, exception_index_table })
            }

            "InnerClasses" => {
                let mut number_of_classes_buf = [0; 2];
                buf_reader.read_exact(&mut number_of_classes_buf).expect("Failed to read number of classes for InnerClasses attribute from class file");
                let number_of_classes = u16::from_be_bytes(number_of_classes_buf);

                let mut classes = Vec::with_capacity(number_of_classes as usize);
                for _ in 0..number_of_classes {
                    let class_info = InnerClassInfo::from_reader(buf_reader)?;
                    classes.push(class_info);
                }

                Ok(AttributeInfo::InnerClasses { number_of_classes, classes })
            }

            "EnclosingMethod" => {
                let mut class_index_buf = [0; 2];
                buf_reader.read_exact(&mut class_index_buf)?;
                let class_index = u16::from_be_bytes(class_index_buf);

                let mut method_index_buf = [0; 2];
                buf_reader.read_exact(&mut method_index_buf)?;
                let method_index = u16::from_be_bytes(method_index_buf);

                Ok(AttributeInfo::EnclosingMethod { class_index, method_index })
            }

            "Synthetic" => Ok(AttributeInfo::Synthetic),

            "Signature" => {
                let mut signature_index_buf = [0; 2];
                buf_reader.read_exact(&mut signature_index_buf)?;
                let signature_index = u16::from_be_bytes(signature_index_buf);
                Ok(AttributeInfo::Signature { signature_index })
            }

            "SourceFile" => {
                let mut sourcefile_index_buf = [0; 2];
                buf_reader.read_exact(&mut sourcefile_index_buf)?;
                let sourcefile_index = u16::from_be_bytes(sourcefile_index_buf);
                Ok(AttributeInfo::SourceFile { sourcefile_index })
            }

            "SourceDebugExtension" => {
                let mut debug_extension = vec![0; attribute_length as usize];
                buf_reader.read_exact(&mut debug_extension)?;
                Ok(AttributeInfo::SourceDebugExtension { debug_extension })
            }

            "LineNumberTable" => {
                let mut line_number_table_length_buf = [0; 2];
                buf_reader.read_exact(&mut line_number_table_length_buf).expect("Failed to read line number table length for LineNumberTable attribute from class file");
                let line_number_table_length = u16::from_be_bytes(line_number_table_length_buf);

                let mut line_number_table = Vec::with_capacity(line_number_table_length as usize);
                for _ in 0..line_number_table_length {
                    let entry = LineNumberInfo::from_reader(buf_reader)?;
                    line_number_table.push(entry);
                }

                Ok(AttributeInfo::LineNumberTable { line_number_table_length, line_number_table })
            }

            "LocalVariableTable" => {
                let mut local_variable_table_length_buf = [0; 2];
                buf_reader.read_exact(&mut local_variable_table_length_buf).expect("Failed to read local variable table length for LocalVariableTable attribute from class file");
                let local_variable_table_length = u16::from_be_bytes(local_variable_table_length_buf);

                let mut local_variable_table = Vec::with_capacity(local_variable_table_length as usize);
                for _ in 0..local_variable_table_length {
                    let entry = LocalVariableInfo::from_reader(buf_reader)?;
                    local_variable_table.push(entry);
                }

                Ok(AttributeInfo::LocalVariableTable { local_variable_table_length, local_variable_table })
            }

            "LocalVariableTypeTable" => {
                let mut local_variable_type_table_length_buf = [0; 2];
                buf_reader.read_exact(&mut local_variable_type_table_length_buf).expect("Failed to read local variable type table length for LocalVariableTypeTable attribute from class file");
                let local_variable_type_table_length = u16::from_be_bytes(local_variable_type_table_length_buf);

                let mut local_variable_type_table = Vec::with_capacity(local_variable_type_table_length as usize);
                for _ in 0..local_variable_type_table_length {
                    let entry = LocalVariableTypeInfo::from_reader(buf_reader)?;
                    local_variable_type_table.push(entry);
                }

                Ok(AttributeInfo::LocalVariableTypeTable { local_variable_type_table_length, local_variable_type_table })
            }

            "Deprecated" => Ok(AttributeInfo::Deprecated),

            "RuntimeVisibleAnnotations" => {
                let mut num_annotations_buf = [0; 2];
                buf_reader.read_exact(&mut num_annotations_buf)?;
                let num_annotations = u16::from_be_bytes(num_annotations_buf);

                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    let annotation = Annotation::from_reader(buf_reader)?;
                    annotations.push(annotation);
                }

                Ok(AttributeInfo::RuntimeVisibleAnnotations { num_annotations, annotations })
            }

            "RuntimeInvisibleAnnotations" => {
                let mut num_annotations_buf = [0; 2];
                buf_reader.read_exact(&mut num_annotations_buf)?;
                let num_annotations = u16::from_be_bytes(num_annotations_buf);

                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    let annotation = Annotation::from_reader(buf_reader)?;
                    annotations.push(annotation);
                }

                Ok(AttributeInfo::RuntimeInvisibleAnnotations { num_annotations, annotations })
            }

            "RuntimeVisibleParameterAnnotations" => {
                let mut num_parameters_buf = [0; 1];
                buf_reader.read_exact(&mut num_parameters_buf).expect("Failed to read number of parameters for RuntimeVisibleParameterAnnotations attribute from class file");
                let num_parameters = u8::from_be_bytes(num_parameters_buf);

                let mut parameter_annotations = Vec::with_capacity(num_parameters as usize);
                for _ in 0..num_parameters {
                    let parameter_annotation = ParameterAnnotations::from_reader(buf_reader)?;
                    parameter_annotations.push(parameter_annotation);
                }

                Ok(AttributeInfo::RuntimeVisibleParameterAnnotations { num_parameters, parameter_annotations })
            }

            "RuntimeInvisibleParameterAnnotations" => {
                let mut num_parameters_buf = [0; 1];
                buf_reader.read_exact(&mut num_parameters_buf).expect("Failed to read number of parameters for RuntimeInvisibleParameterAnnotations attribute from class file");
                let num_parameters = u8::from_be_bytes(num_parameters_buf);

                let mut parameter_annotations = Vec::with_capacity(num_parameters as usize);
                for _ in 0..num_parameters {
                    let parameter_annotation = ParameterAnnotations::from_reader(buf_reader)?;
                    parameter_annotations.push(parameter_annotation);
                }

                Ok(AttributeInfo::RuntimeInvisibleParameterAnnotations { num_parameters, parameter_annotations })
            }

            "RuntimeVisibleTypeAnnotations" => {
                let mut num_annotations_buf = [0; 2];
                buf_reader.read_exact(&mut num_annotations_buf)?;
                let num_annotations = u16::from_be_bytes(num_annotations_buf);

                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    let annotation = TypeAnnotation::from_reader(buf_reader)?;
                    annotations.push(annotation);
                }

                Ok(AttributeInfo::RuntimeVisibleTypeAnnotations { num_annotations, annotations })
            }

            "RuntimeInvisibleTypeAnnotations" => {
                let mut num_annotations_buf = [0; 2];
                buf_reader.read_exact(&mut num_annotations_buf).expect("Failed to read number of annotations for RuntimeInvisibleTypeAnnotations attribute from class file");
                let num_annotations = u16::from_be_bytes(num_annotations_buf);

                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    let annotation = TypeAnnotation::from_reader(buf_reader)?;
                    annotations.push(annotation);
                }

                Ok(AttributeInfo::RuntimeInvisibleTypeAnnotations { num_annotations, annotations })
            }

            "AnnotationDefault" => {
                let default_value = ElementValue::from_reader(buf_reader)?;
                Ok(AttributeInfo::AnnotationDefault { default_value })
            }

            "BootstrapMethods" => {
                let mut num_bootstrap_methods_buf = [0; 2];
                buf_reader.read_exact(&mut num_bootstrap_methods_buf)?;
                let num_bootstrap_methods = u16::from_be_bytes(num_bootstrap_methods_buf);

                let mut bootstrap_methods = Vec::with_capacity(num_bootstrap_methods as usize);
                for _ in 0..num_bootstrap_methods {
                    let bootstrap_method = BootstrapMethod::from_reader(buf_reader)?;
                    bootstrap_methods.push(bootstrap_method);
                }

                Ok(AttributeInfo::BootstrapMethods { num_bootstrap_methods, bootstrap_methods })
            }

            "MethodParameters" => {
                let mut parameters_count_buf = [0; 1];
                buf_reader.read_exact(&mut parameters_count_buf)?;
                let parameters_count = u8::from_be_bytes(parameters_count_buf);

                let mut parameters = Vec::with_capacity(parameters_count as usize);
                for _ in 0..parameters_count {
                    let parameter = MethodParameter::from_reader(buf_reader)?;
                    parameters.push(parameter);
                }

                Ok(AttributeInfo::MethodParameters { parameters_count, parameters })
            }

            "Module" => {
                let mut module_name_index_buf = [0; 2];
                buf_reader.read_exact(&mut module_name_index_buf)?;
                let module_name_index = u16::from_be_bytes(module_name_index_buf);

                let mut module_flags_buf = [0; 2];
                buf_reader.read_exact(&mut module_flags_buf)?;
                let module_flags = ModuleFlags::from_bits(u16::from_be_bytes(module_flags_buf)).expect("Invalid module flags in class file");

                let mut module_version_index_buf = [0; 2];
                buf_reader.read_exact(&mut module_version_index_buf)?;
                let module_version_index = u16::from_be_bytes(module_version_index_buf);

                let mut requires_count_buf = [0; 2];
                buf_reader.read_exact(&mut requires_count_buf)?;
                let requires_count = u16::from_be_bytes(requires_count_buf);

                let mut requires = Vec::with_capacity(requires_count as usize);
                for _ in 0..requires_count {
                    let require = ModuleRequire::from_reader(buf_reader)?;
                    requires.push(require);
                }

                let mut exports_count_buf = [0; 2];
                buf_reader.read_exact(&mut exports_count_buf)?;
                let exports_count = u16::from_be_bytes(exports_count_buf);

                let mut exports = Vec::with_capacity(exports_count as usize);
                for _ in 0..exports_count {
                    let export = ModuleExport::from_reader(buf_reader)?;
                    exports.push(export);
                }

                let mut opens_count_buf = [0; 2];
                buf_reader.read_exact(&mut opens_count_buf)?;
                let opens_count = u16::from_be_bytes(opens_count_buf);

                let mut opens = Vec::with_capacity(opens_count as usize);
                for _ in 0..opens_count {
                    let open = ModuleOpen::from_reader(buf_reader)?;
                    opens.push(open);
                }

                let mut uses_count_buf = [0; 2];
                buf_reader.read_exact(&mut uses_count_buf)?;
                let uses_count = u16::from_be_bytes(uses_count_buf);

                let mut uses_index = Vec::with_capacity(uses_count as usize);
                for _ in 0..uses_count {
                    let mut use_index_buf = [0; 2];
                    buf_reader.read_exact(&mut use_index_buf)?;
                    let use_index = u16::from_be_bytes(use_index_buf);
                    uses_index.push(use_index);
                }

                let mut provides_count_buf = [0; 2];
                buf_reader.read_exact(&mut provides_count_buf)?;
                let provides_count = u16::from_be_bytes(provides_count_buf);

                let mut provides = Vec::with_capacity(provides_count as usize);
                for _ in 0..provides_count {
                    let provide = ModuleProvide::from_reader(buf_reader)?;
                    provides.push(provide);
                }

                Ok(AttributeInfo::Module { 
                    module_name_index, 
                    module_flags, 
                    module_version_index, 
                    requires_count, 
                    requires, 
                    exports_count, 
                    exports, 
                    opens_count, 
                    opens, 
                    uses_count, 
                    uses_index, 
                    provides_count, 
                    provides 
                })
            }

            "ModulePackages" => {
                let mut package_count_buf = [0; 2];
                buf_reader.read_exact(&mut package_count_buf)?;
                let package_count = u16::from_be_bytes(package_count_buf);

                let mut package_index = Vec::with_capacity(package_count as usize);
                for _ in 0..package_count {
                    let mut package_index_buf = [0; 2];
                    buf_reader.read_exact(&mut package_index_buf)?;
                    let package_index_entry = u16::from_be_bytes(package_index_buf);
                    package_index.push(package_index_entry);
                }

                Ok(AttributeInfo::ModulePackages { package_count, package_index })
            }

            "ModuleMainClass" => {
                let mut main_class_index_buf = [0; 2];
                buf_reader.read_exact(&mut main_class_index_buf)?;
                let main_class_index = u16::from_be_bytes(main_class_index_buf);
                Ok(AttributeInfo::ModuleMainClass { main_class_index })
            }

            "NestHost" => {
                let mut host_class_index_buf = [0; 2];
                buf_reader.read_exact(&mut host_class_index_buf)?;
                let host_class_index = u16::from_be_bytes(host_class_index_buf);
                Ok(AttributeInfo::NestHost { host_class_index })
            }

            "NestMembers" => {
                let mut number_of_classes_buf = [0; 2];
                buf_reader.read_exact(&mut number_of_classes_buf)?;
                let number_of_classes = u16::from_be_bytes(number_of_classes_buf);

                let mut classes = Vec::with_capacity(number_of_classes as usize);
                for _ in 0..number_of_classes {
                    let mut class_index_buf = [0; 2];
                    buf_reader.read_exact(&mut class_index_buf)?;
                    let class_index = u16::from_be_bytes(class_index_buf);
                    classes.push(class_index);
                }

                Ok(AttributeInfo::NestMembers { number_of_classes, classes })
            }

            "Record" => {
                let mut number_of_components_buf = [0; 2];
                buf_reader.read_exact(&mut number_of_components_buf)?;
                let number_of_components = u16::from_be_bytes(number_of_components_buf);

                let mut components = Vec::with_capacity(number_of_components as usize);
                for _ in 0..number_of_components {
                    let component = RecordComponentInfo::from_reader(buf_reader, constant_pool_info)?;
                    components.push(component);
                }

                Ok(AttributeInfo::Record { number_of_components, components })
            }

            "PermittedSubclasses" => {
                let mut number_of_classes_buf = [0; 2];
                buf_reader.read_exact(&mut number_of_classes_buf)?;
                let number_of_classes = u16::from_be_bytes(number_of_classes_buf);

                let mut classes = Vec::with_capacity(number_of_classes as usize);
                for _ in 0..number_of_classes {
                    let mut class_index_buf = [0; 2];
                    buf_reader.read_exact(&mut class_index_buf)?;
                    let class_index = u16::from_be_bytes(class_index_buf);
                    classes.push(class_index);
                }

                Ok(AttributeInfo::PermittedSubclasses { number_of_classes, classes })
            }


            _ => {
                // skip the attribute content
                let mut skip_buffer = vec![0; attribute_length as usize];
                buf_reader.read_exact(&mut skip_buffer)?;
                Ok(AttributeInfo::UnknownAttribute)
            }
        }

    }
}

#[derive(Debug)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl ExceptionTableEntry {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut start_pc_buf = [0; 2];
        buf_reader.read_exact(&mut start_pc_buf)?;
        let start_pc = u16::from_be_bytes(start_pc_buf);

        let mut end_pc_buf = [0; 2];
        buf_reader.read_exact(&mut end_pc_buf)?;
        let end_pc = u16::from_be_bytes(end_pc_buf);

        let mut handler_pc_buf = [0; 2];
        buf_reader.read_exact(&mut handler_pc_buf)?;
        let handler_pc = u16::from_be_bytes(handler_pc_buf);

        let mut catch_type_buf = [0; 2];
        buf_reader.read_exact(&mut catch_type_buf)?;
        let catch_type = u16::from_be_bytes(catch_type_buf);

        Ok(ExceptionTableEntry {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        })
    }
}

#[derive(Debug)]
pub enum StackMapFrame {
    SameFrame { frame_type: u8 },
    SameLocals1StackItemFrame { frame_type: u8, stack: VerificationTypeInfo },
    SameLocals1StackItemFrameExtended { frame_type: u8, offset_delta: u16, stack: VerificationTypeInfo },
    ChopFrame { frame_type: u8, offset_delta: u16 },
    SameFrameExtended { frame_type: u8, offset_delta: u16 },
    AppendFrame { frame_type: u8, offset_delta: u16, locals: Vec<VerificationTypeInfo> },
    FullFrame { frame_type: u8, offset_delta: u16, number_of_locals: u16, locals: Vec<VerificationTypeInfo>, number_of_stack_items: u16, stack: Vec<VerificationTypeInfo> },
}

impl StackMapFrame {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut frame_type_buf = [0; 1];
        buf_reader.read_exact(&mut frame_type_buf)?;
        let frame_type = u8::from_be_bytes(frame_type_buf);

        match frame_type {
            0..=63 => Ok(StackMapFrame::SameFrame { frame_type }),
            64..=127 => {
                let stack = VerificationTypeInfo::from_reader(buf_reader)?;
                Ok(StackMapFrame::SameLocals1StackItemFrame { frame_type, stack })
            }
            247 => {
                let mut offset_delta_buf = [0; 2];
                buf_reader.read_exact(&mut offset_delta_buf)?;
                let offset_delta = u16::from_be_bytes(offset_delta_buf);
                let stack = VerificationTypeInfo::from_reader(buf_reader)?;
                Ok(StackMapFrame::SameLocals1StackItemFrameExtended { frame_type, offset_delta, stack })
            }
            248..=250 => {
                let mut offset_delta_buf = [0; 2];
                buf_reader.read_exact(&mut offset_delta_buf)?;
                let offset_delta = u16::from_be_bytes(offset_delta_buf);
                Ok(StackMapFrame::ChopFrame { frame_type, offset_delta })
            }
            251 => {
                let mut offset_delta_buf = [0; 2];
                buf_reader.read_exact(&mut offset_delta_buf)?;
                let offset_delta = u16::from_be_bytes(offset_delta_buf);
                Ok(StackMapFrame::SameFrameExtended { frame_type, offset_delta })
            }
            252..=254 => {
                let mut offset_delta_buf = [0; 2];
                buf_reader.read_exact(&mut offset_delta_buf)?;
                let offset_delta = u16::from_be_bytes(offset_delta_buf);

                let number_of_locals = (frame_type - 251) as usize;
                let mut locals = Vec::with_capacity(number_of_locals);
                for _ in 0..number_of_locals {
                    let local = VerificationTypeInfo::from_reader(buf_reader)?;
                    locals.push(local);
                }

                Ok(StackMapFrame::AppendFrame { frame_type, offset_delta, locals })
            }
            255 => {
                let mut offset_delta_buf = [0; 2];
                buf_reader.read_exact(&mut offset_delta_buf)?;
                let offset_delta = u16::from_be_bytes(offset_delta_buf);

                let mut number_of_locals_buf = [0; 2];
                buf_reader.read_exact(&mut number_of_locals_buf)?;
                let number_of_locals = u16::from_be_bytes(number_of_locals_buf);

                let mut locals = Vec::with_capacity(number_of_locals as usize);
                for _ in 0..number_of_locals {
                    let local = VerificationTypeInfo::from_reader(buf_reader)?;
                    locals.push(local);
                }

                let mut number_of_stack_items_buf = [0; 2];
                buf_reader.read_exact(&mut number_of_stack_items_buf)?;
                let number_of_stack_items = u16::from_be_bytes(number_of_stack_items_buf);

                let mut stack = Vec::with_capacity(number_of_stack_items as usize);

                for _ in 0..number_of_stack_items {
                    let stack_item = VerificationTypeInfo::from_reader(buf_reader)?;
                    stack.push(stack_item);
                }

                Ok(StackMapFrame::FullFrame { frame_type, offset_delta, number_of_locals, locals, number_of_stack_items, stack })

            }
            _ => Err(anyhow!("Invalid frame type in StackMapTable attribute in class file")), 
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationTypeInfo {
    TopVariableInfo,
    IntegerVariableInfo,
    FloatVariableInfo,
    LongVariableInfo,
    DoubleVariableInfo,
    NullVariableInfo,
    UninitializedThisVariableInfo,
    ObjectVariableInfo { cpool_index: u16 },
    UninitializedVariableInfo { offset: u16 },
}

impl VerificationTypeInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut tag_buf = [0; 1];
        buf_reader.read_exact(&mut tag_buf)?;
        let tag = u8::from_be_bytes(tag_buf);

        match tag {
            0 => Ok(VerificationTypeInfo::TopVariableInfo),
            1 => Ok(VerificationTypeInfo::IntegerVariableInfo),
            2 => Ok(VerificationTypeInfo::FloatVariableInfo),
            3 => Ok(VerificationTypeInfo::LongVariableInfo),
            4 => Ok(VerificationTypeInfo::DoubleVariableInfo),
            5 => Ok(VerificationTypeInfo::NullVariableInfo),
            6 => Ok(VerificationTypeInfo::UninitializedThisVariableInfo),
            7 => {
                let mut cpool_index_buf = [0; 2];
                buf_reader.read_exact(&mut cpool_index_buf)?;
                let cpool_index = u16::from_be_bytes(cpool_index_buf);
                Ok(VerificationTypeInfo::ObjectVariableInfo { cpool_index })
            }
            8 => {
                let mut offset_buf = [0; 2];
                buf_reader.read_exact(&mut offset_buf)?;
                let offset = u16::from_be_bytes(offset_buf);
                Ok(VerificationTypeInfo::UninitializedVariableInfo { offset })
            }
            _ => Err(anyhow!("Invalid verification type info tag in class file")),
        }
    }
}

#[derive(Debug)]
pub struct InnerClassInfo {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: InnerClassAccessFlags,
}

impl InnerClassInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut inner_class_info_index_buf = [0; 2];
        buf_reader.read_exact(&mut inner_class_info_index_buf)?;
        let inner_class_info_index = u16::from_be_bytes(inner_class_info_index_buf);

        let mut outer_class_info_index_buf = [0; 2];
        buf_reader.read_exact(&mut outer_class_info_index_buf)?;
        let outer_class_info_index = u16::from_be_bytes(outer_class_info_index_buf);

        let mut inner_name_index_buf = [0; 2];
        buf_reader.read_exact(&mut inner_name_index_buf)?;
        let inner_name_index = u16::from_be_bytes(inner_name_index_buf);

        let mut inner_class_access_flags_buf = [0; 2];
        buf_reader.read_exact(&mut inner_class_access_flags_buf)?;
        let inner_class_access_flags = InnerClassAccessFlags::from_bits(u16::from_be_bytes(inner_class_access_flags_buf));
        if inner_class_access_flags.is_none() {
            return Err(anyhow!("Invalid inner class access flags in class file"));
        }

        Ok(InnerClassInfo {
            inner_class_info_index,
            outer_class_info_index,
            inner_name_index,
            inner_class_access_flags: inner_class_access_flags.unwrap(),
        })
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct InnerClassAccessFlags: u16 {
        const Public = 0x0001;
        const Private = 0x0002;
        const Protected = 0x0004;
        const Static = 0x0008;
        const Final = 0x0010;
        const Interface = 0x0200;
        const Abstract = 0x0400;
        const Synthetic = 0x1000;
        const Annotation = 0x2000;
        const Enum = 0x4000;
    }
}

#[derive(Debug)]
pub struct LineNumberInfo {
    pub start_pc: u16,
    pub line_number: u16,
}

impl LineNumberInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut start_pc_buf = [0; 2];
        buf_reader.read_exact(&mut start_pc_buf)?;
        let start_pc = u16::from_be_bytes(start_pc_buf);

        let mut line_number_buf = [0; 2];
        buf_reader.read_exact(&mut line_number_buf)?;
        let line_number = u16::from_be_bytes(line_number_buf);

        Ok(LineNumberInfo {
            start_pc,
            line_number,
        })
    }
}

#[derive(Debug)]
pub struct LocalVariableInfo {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

impl LocalVariableInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut start_pc_buf = [0; 2];
        buf_reader.read_exact(&mut start_pc_buf)?;
        let start_pc = u16::from_be_bytes(start_pc_buf);

        let mut length_buf = [0; 2];
        buf_reader.read_exact(&mut length_buf)?;
        let length = u16::from_be_bytes(length_buf);

        let mut name_index_buf = [0; 2];
        buf_reader.read_exact(&mut name_index_buf)?;
        let name_index = u16::from_be_bytes(name_index_buf);

        let mut descriptor_index_buf = [0; 2];
        buf_reader.read_exact(&mut descriptor_index_buf)?;
        let descriptor_index = u16::from_be_bytes(descriptor_index_buf);

        let mut index_buf = [0; 2];
        buf_reader.read_exact(&mut index_buf)?;
        let index = u16::from_be_bytes(index_buf);

        Ok(LocalVariableInfo {
            start_pc,
            length,
            name_index,
            descriptor_index,
            index,
        })
    }
}

#[derive(Debug)]
pub struct LocalVariableTypeInfo {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub signature_index: u16,
    pub index: u16,
}

impl LocalVariableTypeInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut start_pc_buf = [0; 2];
        buf_reader.read_exact(&mut start_pc_buf)?;
        let start_pc = u16::from_be_bytes(start_pc_buf);

        let mut length_buf = [0; 2];
        buf_reader.read_exact(&mut length_buf)?;
        let length = u16::from_be_bytes(length_buf);

        let mut name_index_buf = [0; 2];
        buf_reader.read_exact(&mut name_index_buf)?;
        let name_index = u16::from_be_bytes(name_index_buf);

        let mut signature_index_buf = [0; 2];
        buf_reader.read_exact(&mut signature_index_buf)?;
        let signature_index = u16::from_be_bytes(signature_index_buf);

        let mut index_buf = [0; 2];
        buf_reader.read_exact(&mut index_buf)?;
        let index = u16::from_be_bytes(index_buf);

        Ok(LocalVariableTypeInfo {
            start_pc,
            length,
            name_index,
            signature_index,
            index,
        })
    }
}

#[derive(Debug)]
pub struct Annotation {
    pub type_index: u16,
    pub num_element_value_pairs: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

impl Annotation {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut type_index_buf = [0; 2];
        buf_reader.read_exact(&mut type_index_buf)?;
        let type_index = u16::from_be_bytes(type_index_buf);

        let mut num_element_value_pairs_buf = [0; 2];
        buf_reader.read_exact(&mut num_element_value_pairs_buf)?;
        let num_element_value_pairs = u16::from_be_bytes(num_element_value_pairs_buf);

        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
        for _ in 0..num_element_value_pairs {
            let mut element_name_index_buf = [0; 2];
            buf_reader.read_exact(&mut element_name_index_buf)?;
            let element_name_index = u16::from_be_bytes(element_name_index_buf);

            let value = ElementValue::from_reader(buf_reader)?;

            element_value_pairs.push(ElementValuePair { element_name_index, value });
        }

        Ok(Annotation {
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        })
    }
}

#[derive(Debug)]
pub struct ElementValuePair {
    pub element_name_index: u16,
    pub value: ElementValue,
}

#[derive(Debug)]
pub struct ParameterAnnotations {
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

impl ParameterAnnotations {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut num_annotations_buf = [0; 2];
        buf_reader.read_exact(&mut num_annotations_buf)?;
        let num_annotations = u16::from_be_bytes(num_annotations_buf);

        let mut annotations = Vec::with_capacity(num_annotations as usize);
        for _ in 0..num_annotations {
            let annotation = Annotation::from_reader(buf_reader)?;
            annotations.push(annotation);
        }

        Ok(ParameterAnnotations {
            num_annotations,
            annotations,
        })
    }
}

#[derive(Debug)]
pub struct TypeAnnotation {
    pub type_annotation_info: TypeAnnotationInfo,
    pub type_path: TypePath,
    pub type_index: u16,
    pub num_element_value_pairs: u16,
    pub element_value_pairs: Vec<ElementValuePair>,
}

impl TypeAnnotation {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let type_annotation_info = TypeAnnotationInfo::from_reader(buf_reader)?;
        let type_path = TypePath::from_reader(buf_reader)?;

        let mut type_index_buf = [0; 2];
        buf_reader.read_exact(&mut type_index_buf)?;
        let type_index = u16::from_be_bytes(type_index_buf);

        let mut num_element_value_pairs_buf = [0; 2];
        buf_reader.read_exact(&mut num_element_value_pairs_buf)?;
        let num_element_value_pairs = u16::from_be_bytes(num_element_value_pairs_buf);

        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);

        for _ in 0..num_element_value_pairs {
            let mut element_name_index_buf = [0; 2];
            buf_reader.read_exact(&mut element_name_index_buf)?;
            let element_name_index = u16::from_be_bytes(element_name_index_buf);

            let value = ElementValue::from_reader(buf_reader)?;

            element_value_pairs.push(ElementValuePair { element_name_index, value });
        }

        Ok(TypeAnnotation {
            type_annotation_info,
            type_path,
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        })
    }
}


#[derive(Debug, PartialEq, Eq)]
pub enum TypeAnnotationInfo {
    TypeParameterTarget { target_type: u8, type_parameter_index: u8 },
    SupertypeTarget { target_type: u8, supertype_index: u16 },
    TypeParameterBoundTarget { target_type: u8, type_parameter_index: u8, bound_index: u8 },
    EmptyTarget { target_type: u8 },
    FormalParameterTarget { target_type: u8, formal_parameter_index: u8 },
    ThrowsTarget { target_type: u8, throws_type_index: u16 },
    LocalvarTarget { target_type: u8, table_length: u16, table: Vec<LocalvarTargetTableEntry> },
    CatchTarget { target_type: u8, exception_table_index: u16 },
    OffsetTarget { target_type: u8, offset: u16 },
    TypeArgumentTarget { target_type: u8, offset: u16, type_argument_index: u8 },
}

impl TypeAnnotationInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut target_type_buf = [0; 1];
        buf_reader.read_exact(&mut target_type_buf)?;
        let target_type = u8::from_be_bytes(target_type_buf);

        match target_type {
            0x00 | 0x01 => {
                let mut type_parameter_index_buf = [0; 1];
                buf_reader.read_exact(&mut type_parameter_index_buf)?;
                let type_parameter_index = u8::from_be_bytes(type_parameter_index_buf);
                Ok(TypeAnnotationInfo::TypeParameterTarget { target_type, type_parameter_index })
            }
            0x10 => {
                let mut supertype_index_buf = [0; 2];
                buf_reader.read_exact(&mut supertype_index_buf)?;
                let supertype_index = u16::from_be_bytes(supertype_index_buf);
                Ok(TypeAnnotationInfo::SupertypeTarget { target_type, supertype_index })
            }
            0x11 | 0x12 => {
                let mut type_parameter_index_buf = [0; 1];
                buf_reader.read_exact(&mut type_parameter_index_buf)?;
                let type_parameter_index = u8::from_be_bytes(type_parameter_index_buf);

                let mut bound_index_buf = [0; 1];
                buf_reader.read_exact(&mut bound_index_buf)?;
                let bound_index = u8::from_be_bytes(bound_index_buf);

                Ok(TypeAnnotationInfo::TypeParameterBoundTarget { target_type, type_parameter_index, bound_index })
            }
            0x13 | 0x14 | 0x15 => {
                Ok(TypeAnnotationInfo::EmptyTarget { target_type })
            }
            0x16 => {
                let mut formal_parameter_index_buf = [0; 1];
                buf_reader.read_exact(&mut formal_parameter_index_buf)?;
                let formal_parameter_index = u8::from_be_bytes(formal_parameter_index_buf);
                Ok(TypeAnnotationInfo::FormalParameterTarget { target_type, formal_parameter_index })
            }
            0x17 => {
                let mut throws_type_index_buf = [0; 2];
                buf_reader.read_exact(&mut throws_type_index_buf)?;
                let throws_type_index = u16::from_be_bytes(throws_type_index_buf);
                Ok(TypeAnnotationInfo::ThrowsTarget { target_type, throws_type_index })
            }
            0x40 | 0x41 => {
                let mut table_length_buf = [0; 2];
                buf_reader.read_exact(&mut table_length_buf)?;
                let table_length = u16::from_be_bytes(table_length_buf);

                let mut table = Vec::with_capacity(table_length as usize);
                for _ in 0..table_length {
                    let entry = LocalvarTargetTableEntry::from_reader(buf_reader)?;
                    table.push(entry);
                }

                Ok(TypeAnnotationInfo::LocalvarTarget { target_type, table_length, table })
            }
            0x42 => {
                let mut exception_table_index_buf = [0; 2];
                buf_reader.read_exact(&mut exception_table_index_buf)?;
                let exception_table_index = u16::from_be_bytes(exception_table_index_buf);
                Ok(TypeAnnotationInfo::CatchTarget { target_type, exception_table_index })
            }
            0x43..=0x46 => {
                let mut offset_buf = [0; 2];
                buf_reader.read_exact(&mut offset_buf)?;
                let offset = u16::from_be_bytes(offset_buf);

                if target_type == 0x43 {
                    Ok(TypeAnnotationInfo::OffsetTarget { target_type, offset })
                } else {
                    let mut type_argument_index_buf = [0; 1];
                    buf_reader.read_exact(&mut type_argument_index_buf)?;
                    let type_argument_index = u8::from_be_bytes(type_argument_index_buf);
                    Ok(TypeAnnotationInfo::TypeArgumentTarget { target_type, offset, type_argument_index })
                }
            }
            0x47..=0x4B => {
                let mut offset_buf = [0; 2];
                buf_reader.read_exact(&mut offset_buf)?;
                let offset = u16::from_be_bytes(offset_buf);

                let mut type_argument_index_buf = [0; 1];
                buf_reader.read_exact(&mut type_argument_index_buf)?;
                let type_argument_index = u8::from_be_bytes(type_argument_index_buf);

                Ok(TypeAnnotationInfo::TypeArgumentTarget { target_type, offset, type_argument_index })
            }
            _ => {
                Err(anyhow!("Invalid target type for type annotation in class file"))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LocalvarTargetTableEntry {
    start_pc: u16,
    length: u16,
    index: u16,
}

impl LocalvarTargetTableEntry {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut start_pc_buf = [0; 2];
        buf_reader.read_exact(&mut start_pc_buf)?;
        let start_pc = u16::from_be_bytes(start_pc_buf);

        let mut length_buf = [0; 2];
        buf_reader.read_exact(&mut length_buf)?;
        let length = u16::from_be_bytes(length_buf);

        let mut index_buf = [0; 2];
        buf_reader.read_exact(&mut index_buf)?;
        let index = u16::from_be_bytes(index_buf);

        Ok(LocalvarTargetTableEntry { start_pc, length, index })
    }
}

#[derive(Debug)]
pub struct TypePath {
    pub path_length: u8,
    pub paths: Vec<TypePathEntry>,
}

impl TypePath {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut path_length_buf = [0; 1];
        buf_reader.read_exact(&mut path_length_buf)?;
        let path_length = u8::from_be_bytes(path_length_buf);

        let mut paths = Vec::with_capacity(path_length as usize);
        for _ in 0..path_length {
            let mut type_path_kind_buf = [0; 1];
            buf_reader.read_exact(&mut type_path_kind_buf)?;
            let type_path_kind = u8::from_be_bytes(type_path_kind_buf);

            let mut type_argument_index_buf = [0; 1];
            buf_reader.read_exact(&mut type_argument_index_buf)?;
            let type_argument_index = u8::from_be_bytes(type_argument_index_buf);

            paths.push(TypePathEntry { type_path_kind, type_argument_index });
        }

        Ok(TypePath {
            path_length,
            paths,
        })
    }
}

#[derive(Debug)]
pub struct TypePathEntry {
    pub type_path_kind: u8,
    pub type_argument_index: u8,
}

#[derive(Debug)]
pub enum ElementValue {
    ConstValueIndex(u16),
    EnumConstValue { type_name_index: u16, const_name_index: u16 },
    ClassInfoIndex(u16),
    AnnotationValue(Annotation),
    ArrayValue { num_values: u16, values: Vec<ElementValue> },
}

impl ElementValue {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {

        let mut tag_buf = [0; 1];
        buf_reader.read_exact(&mut tag_buf)?;
        let tag = tag_buf[0] as char;

        match tag {
            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' => {
                let mut const_value_index_buf = [0; 2];
                buf_reader.read_exact(&mut const_value_index_buf)?;
                let const_value_index = u16::from_be_bytes(const_value_index_buf);
                Ok(ElementValue::ConstValueIndex(const_value_index))
            }

            'e' => {
                let mut type_name_index_buf = [0; 2];
                buf_reader.read_exact(&mut type_name_index_buf)?;
                let type_name_index = u16::from_be_bytes(type_name_index_buf);

                let mut const_name_index_buf = [0; 2];
                buf_reader.read_exact(&mut const_name_index_buf)?;
                let const_name_index = u16::from_be_bytes(const_name_index_buf);

                Ok(ElementValue::EnumConstValue { type_name_index, const_name_index })
            }

            'c' => {
                let mut class_info_index_buf = [0; 2];
                buf_reader.read_exact(&mut class_info_index_buf)?;
                let class_info_index = u16::from_be_bytes(class_info_index_buf);
                Ok(ElementValue::ClassInfoIndex(class_info_index))
            }

            '@' => {
                let annotation = Annotation::from_reader(buf_reader)?;
                Ok(ElementValue::AnnotationValue(annotation))
            }

            '[' => {
                let mut num_values_buf = [0; 2];
                buf_reader.read_exact(&mut num_values_buf)?;
                let num_values = u16::from_be_bytes(num_values_buf);

                let mut values = Vec::with_capacity(num_values as usize);
                for _ in 0..num_values {
                    let value = ElementValue::from_reader(buf_reader)?;
                    values.push(value);
                }

                Ok(ElementValue::ArrayValue { num_values, values })
            }

            _ => Err(anyhow!("Invalid element value tag '{}' in class file", tag)),
        }
    }
}

#[derive(Debug)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub num_bootstrap_arguments: u16,
    pub bootstrap_arguments: Vec<u16>,
}

impl BootstrapMethod {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut bootstrap_method_ref_buf = [0; 2];
        buf_reader.read_exact(&mut bootstrap_method_ref_buf)?;
        let bootstrap_method_ref = u16::from_be_bytes(bootstrap_method_ref_buf);

        let mut num_bootstrap_arguments_buf = [0; 2];
        buf_reader.read_exact(&mut num_bootstrap_arguments_buf)?;
        let num_bootstrap_arguments = u16::from_be_bytes(num_bootstrap_arguments_buf);

        let mut bootstrap_arguments = Vec::with_capacity(num_bootstrap_arguments as usize);
        for _ in 0..num_bootstrap_arguments {
            let mut bootstrap_argument_buf = [0; 2];
            buf_reader.read_exact(&mut bootstrap_argument_buf)?;
            let bootstrap_argument = u16::from_be_bytes(bootstrap_argument_buf);
            bootstrap_arguments.push(bootstrap_argument);
        }

        Ok(BootstrapMethod {
            bootstrap_method_ref,
            num_bootstrap_arguments,
            bootstrap_arguments,
        })
    }
}

#[derive(Debug)]
pub struct MethodParameter {
    pub name_index: u16,
    pub access_flags: MethodParameterAccessFlags,
}

impl MethodParameter {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut name_index_buf = [0; 2];
        buf_reader.read_exact(&mut name_index_buf)?;
        let name_index = u16::from_be_bytes(name_index_buf);

        let mut access_flags_buf = [0; 2];
        buf_reader.read_exact(&mut access_flags_buf)?;
        let access_flags = MethodParameterAccessFlags::from_bits(u16::from_be_bytes(access_flags_buf)).expect("Invalid method parameter access flags in class file");

        Ok(MethodParameter {
            name_index,
            access_flags,
        })
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct MethodParameterAccessFlags: u16 {
        const Final = 0x0010;
        const Synthetic = 0x1000;
        const Mandated = 0x8000;
    }
}

#[derive(Debug)]
pub struct ModuleRequire {
    pub required_module_index: u16,
    pub required_flags: ModuleRequireFlags,
    pub required_version_index: u16,
}

impl ModuleRequire {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut required_module_index_buf = [0; 2];
        buf_reader.read_exact(&mut required_module_index_buf)?;
        let required_module_index = u16::from_be_bytes(required_module_index_buf);

        let mut required_flags_buf = [0; 2];
        buf_reader.read_exact(&mut required_flags_buf).expect("Failed to read required flags for module require from class file");
        let required_flags = ModuleRequireFlags::from_bits(u16::from_be_bytes(required_flags_buf));
        if required_flags.is_none() {
            return Err(anyhow!("Invalid module require flags in class file"));
        }

        let mut required_version_index_buf = [0; 2];
        buf_reader.read_exact(&mut required_version_index_buf)?;
        let required_version_index = u16::from_be_bytes(required_version_index_buf);

        Ok(ModuleRequire {
            required_module_index,
            required_flags: required_flags.unwrap(),
            required_version_index,
        })
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct ModuleRequireFlags: u16 {
        const Transitive = 0x0020;
        const StaticPhase = 0x0040;
        const Synthetic = 0x1000;
        const Mandated = 0x8000;
    }
}

#[derive(Debug)]
pub struct ModuleExport {
    pub exported_package_index: u16,
    pub export_flags: ModuleExportFlags,
    pub exported_to_count: u16,
    pub exported_to_index: Vec<u16>,
}

impl ModuleExport {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut exported_package_index_buf = [0; 2];
        buf_reader.read_exact(&mut exported_package_index_buf)?;
        let exported_package_index = u16::from_be_bytes(exported_package_index_buf);

        let mut export_flags_buf = [0; 2];
        buf_reader.read_exact(&mut export_flags_buf)?;
        let export_flags = ModuleExportFlags::from_bits(u16::from_be_bytes(export_flags_buf));
        if export_flags.is_none() {
            return Err(anyhow!("Invalid module export flags in class file"));
        }

        let mut exported_to_count_buf = [0; 2];
        buf_reader.read_exact(&mut exported_to_count_buf)?;
        let exported_to_count = u16::from_be_bytes(exported_to_count_buf);

        let mut exported_to_index = Vec::with_capacity(exported_to_count as usize);
        for _ in 0..exported_to_count {
            let mut exported_to_index_buf = [0; 2];
            buf_reader.read_exact(&mut exported_to_index_buf)?;
            let exported_to_index_entry = u16::from_be_bytes(exported_to_index_buf);
            exported_to_index.push(exported_to_index_entry);
        }

        Ok(ModuleExport {
            exported_package_index,
            export_flags: export_flags.unwrap(),
            exported_to_count,
            exported_to_index,
        })
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct ModuleExportFlags: u16 {
        const Synthetic = 0x1000;
        const Mandated = 0x8000;
    }
}

#[derive(Debug)]
pub struct ModuleOpen {
    pub opens_index: u16,
    pub opens_flags: ModuleOpensFlags,
    pub opens_to_count: u16,
    pub opens_to_index: Vec<u16>,
}

impl ModuleOpen {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut opens_index_buf = [0; 2];
        buf_reader.read_exact(&mut opens_index_buf)?;
        let opens_index = u16::from_be_bytes(opens_index_buf);

        let mut opens_flags_buf = [0; 2];
        buf_reader.read_exact(&mut opens_flags_buf)?;
        let opens_flags = ModuleOpensFlags::from_bits(u16::from_be_bytes(opens_flags_buf)).expect("Invalid module opens flags in class file");

        let mut opens_to_count_buf = [0; 2];
        buf_reader.read_exact(&mut opens_to_count_buf)?;
        let opens_to_count = u16::from_be_bytes(opens_to_count_buf);

        let mut opens_to_index = Vec::with_capacity(opens_to_count as usize);
        for _ in 0..opens_to_count {
            let mut opens_to_index_buf = [0; 2];
            buf_reader.read_exact(&mut opens_to_index_buf)?;
            let opens_to_index_entry = u16::from_be_bytes(opens_to_index_buf);
            opens_to_index.push(opens_to_index_entry);
        }

        Ok(ModuleOpen {
            opens_index,
            opens_flags,
            opens_to_count,
            opens_to_index,
        })
    }
    
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct ModuleOpensFlags: u16 {
        const Synthetic = 0x1000;
        const Mandated = 0x8000;
    }
}

#[derive(Debug)]
pub struct ModuleProvide {
    pub provided_service_index: u16,
    pub provides_with_count: u16,
    pub provides_with_index: Vec<u16>,
}

impl ModuleProvide {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Result<Self> {
        let mut provided_service_index_buf = [0; 2];
        buf_reader.read_exact(&mut provided_service_index_buf)?;
        let provided_service_index = u16::from_be_bytes(provided_service_index_buf);

        let mut provides_with_count_buf = [0; 2];
        buf_reader.read_exact(&mut provides_with_count_buf)?;
        let provides_with_count = u16::from_be_bytes(provides_with_count_buf);

        let mut provides_with_index = Vec::with_capacity(provides_with_count as usize);
        for _ in 0..provides_with_count {
            let mut provides_with_index_buf = [0; 2];
            buf_reader.read_exact(&mut provides_with_index_buf)?;
            let provides_with_index_entry = u16::from_be_bytes(provides_with_index_buf);
            provides_with_index.push(provides_with_index_entry);
        }

        Ok(ModuleProvide {
            provided_service_index,
            provides_with_count,
            provides_with_index,
        })
    }
}

#[derive(Debug)]
pub struct RecordComponentInfo {
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl RecordComponentInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R, constant_pool_info: &Vec<ConstantPoolInfo>) -> Result<Self> {
        let mut name_index_buf = [0; 2];
        buf_reader.read_exact(&mut name_index_buf)?;
        let name_index = u16::from_be_bytes(name_index_buf);

        let mut descriptor_index_buf = [0; 2];
        buf_reader.read_exact(&mut descriptor_index_buf)?;
        let descriptor_index = u16::from_be_bytes(descriptor_index_buf);

        let mut attributes_count_buf = [0; 2];
        buf_reader.read_exact(&mut attributes_count_buf)?;
        let attributes_count = u16::from_be_bytes(attributes_count_buf);

        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            let attribute_info = AttributeInfo::from_reader(buf_reader, constant_pool_info)?;
            attributes.push(attribute_info);
        }

        Ok(RecordComponentInfo {
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }
}