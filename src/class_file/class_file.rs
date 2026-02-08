use std::io::{BufRead, BufReader, Read};
use bitflags::bitflags;

#[derive(Debug)]
pub struct ClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    constant_pool: Vec<CpInfo>,
    access_flags: AccessFlags,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: Vec<u16>,
    fields_count: u16,
    fields: Vec<FieldInfo>,
    methods_count: u16,
    methods: Vec<MethodInfo>,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    pub fn from_reader<R: Read>(reader: R) -> Self {
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

        for _ in 0..constant_pool_count {
            let cp_info = CpInfo::from_reader(&mut buf_reader);
            constant_pool.push(cp_info);
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
            let field_info = FieldInfo::from_reader(&mut buf_reader, &constant_pool);
            fields.push(field_info);
        }

        // methods count
        let mut methods_count_buf = [0; 2];
        buf_reader.read_exact(&mut methods_count_buf).expect("Failed to read methods count from class file");
        let methods_count = u16::from_be_bytes(methods_count_buf);

        // methods
        let mut methods = Vec::with_capacity(methods_count as usize);

        for _ in 0..methods_count {
            let method_info = MethodInfo::from_reader(&mut buf_reader, &constant_pool);
            methods.push(method_info);
        }

        // attributes count
        let mut attributes_count_buf = [0; 2];
        buf_reader.read_exact(&mut attributes_count_buf).expect("Failed to read attributes count from class file");
        let attributes_count = u16::from_be_bytes(attributes_count_buf);

        // attributes
        let mut attributes = Vec::with_capacity(attributes_count as usize);

        for _ in 0..attributes_count {
            let attribute_info = AttributeInfo::from_reader(&mut buf_reader, &constant_pool);
            attributes.push(attribute_info);
        }

        ClassFile {
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
        }
    }

    fn ensure_class_file_validity(&self) {
        todo!("Validity not checked")
    }
}

use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum CpInfoTag {
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

impl TryFrom<u8> for CpInfoTag {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            7 => Ok(CpInfoTag::ConstantClass),
            9 => Ok(CpInfoTag::ConstantFieldRef),
            10 => Ok(CpInfoTag::ConstantMethodRef),
            11 => Ok(CpInfoTag::ConstantInterfaceMethodRef),
            8 => Ok(CpInfoTag::ConstantString),
            3 => Ok(CpInfoTag::ConstantInteger),
            4 => Ok(CpInfoTag::ConstantFloat),
            5 => Ok(CpInfoTag::ConstantLong),
            6 => Ok(CpInfoTag::ConstantDouble),
            12 => Ok(CpInfoTag::ConstantNameAndType),
            1 => Ok(CpInfoTag::ConstantUtf8),
            15 => Ok(CpInfoTag::ConstantMethodHandle),
            16 => Ok(CpInfoTag::ConstantMethodType),
            17 => Ok(CpInfoTag::ConstantDynamic),
            18 => Ok(CpInfoTag::ConstantInvokeDynamic),
            19 => Ok(CpInfoTag::ConstantModule),
            20 => Ok(CpInfoTag::ConstantPackage),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum CpInfo {
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



impl CpInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Self {
        let mut tag_buf = [0; 1];
        buf_reader.read_exact(&mut tag_buf).expect("Failed to read constant pool tag from class file");
        let tag = CpInfoTag::try_from(u8::from_be_bytes(tag_buf)).expect("Invalid constant pool tag");

        match tag {
            CpInfoTag::ConstantClass => {
                let mut name_index_buf = [0; 2];
                buf_reader.read_exact(&mut name_index_buf).expect("Failed to read name index for ConstantClass from class file");
                let name_index = u16::from_be_bytes(name_index_buf);
                CpInfo::ConstantClass { name_index }
            }

            CpInfoTag::ConstantFieldRef => {
                let mut class_index_buf = [0; 2];
                buf_reader.read_exact(&mut class_index_buf).expect("Failed to read class index for ConstantFieldRef from class file");
                let class_index = u16::from_be_bytes(class_index_buf);

                let mut name_and_type_index_buf = [0; 2];
                buf_reader.read_exact(&mut name_and_type_index_buf).expect("Failed to read name and type index for ConstantFieldRef from class file");
                let name_and_type_index = u16::from_be_bytes(name_and_type_index_buf);

                CpInfo::ConstantFieldRef { class_index, name_and_type_index }
            }
            CpInfoTag::ConstantMethodRef => {
                let mut class_index_buf = [0; 2];
                buf_reader.read_exact(&mut class_index_buf).expect("Failed to read class index for ConstantMethodRef from class file");
                let class_index = u16::from_be_bytes(class_index_buf);

                let mut name_and_type_index_buf = [0; 2];
                buf_reader.read_exact(&mut name_and_type_index_buf).expect("Failed to read name and type index for ConstantMethodRef from class file");
                let name_and_type_index = u16::from_be_bytes(name_and_type_index_buf);

                CpInfo::ConstantMethodRef { class_index, name_and_type_index }
            }
            CpInfoTag::ConstantInterfaceMethodRef => {
                let mut class_index_buf = [0; 2];
                buf_reader.read_exact(&mut class_index_buf).expect("Failed to read class index for ConstantInterfaceMethodRef from class file");
                let class_index = u16::from_be_bytes(class_index_buf);

                let mut name_and_type_index_buf = [0; 2];
                buf_reader.read_exact(&mut name_and_type_index_buf).expect("Failed to read name and type index for ConstantInterfaceMethodRef from class file");
                let name_and_type_index = u16::from_be_bytes(name_and_type_index_buf);

                CpInfo::ConstantInterfaceMethodRef { class_index, name_and_type_index }
            }

            CpInfoTag::ConstantString => {
                let mut string_index_buf = [0; 2];
                buf_reader.read_exact(&mut string_index_buf).expect("Failed to read string index for ConstantString from class file");
                let string_index = u16::from_be_bytes(string_index_buf);
                CpInfo::ConstantString { string_index }
            }

            CpInfoTag::ConstantInteger => {
                let mut bytes_buf = [0; 4];
                buf_reader.read_exact(&mut bytes_buf).expect("Failed to read bytes for ConstantInteger from class file");
                let bytes = u32::from_be_bytes(bytes_buf);
                CpInfo::ConstantInteger { bytes }
            }

            CpInfoTag::ConstantFloat => {
                let mut bytes_buf = [0; 4];
                buf_reader.read_exact(&mut bytes_buf).expect("Failed to read bytes for ConstantFloat from class file");
                let bytes = u32::from_be_bytes(bytes_buf);
                CpInfo::ConstantFloat { bytes }
            }

            CpInfoTag::ConstantLong => {
                let mut high_bytes_buf = [0; 4];
                buf_reader.read_exact(&mut high_bytes_buf).expect("Failed to read high bytes for ConstantLong from class file");
                let high_bytes = u32::from_be_bytes(high_bytes_buf);

                let mut low_bytes_buf = [0; 4];
                buf_reader.read_exact(&mut low_bytes_buf).expect("Failed to read low bytes for ConstantLong from class file");
                let low_bytes = u32::from_be_bytes(low_bytes_buf);

                CpInfo::ConstantLong { high_bytes, low_bytes }

            }

            CpInfoTag::ConstantDouble => {
                let mut high_bytes_buf = [0; 4];
                buf_reader.read_exact(&mut high_bytes_buf).expect("Failed to read high bytes for ConstantDouble from class file");
                let high_bytes = u32::from_be_bytes(high_bytes_buf);

                let mut low_bytes_buf = [0; 4];
                buf_reader.read_exact(&mut low_bytes_buf).expect("Failed to read low bytes for ConstantDouble from class file");
                let low_bytes = u32::from_be_bytes(low_bytes_buf);

                CpInfo::ConstantDouble { high_bytes, low_bytes }
            }

            CpInfoTag::ConstantNameAndType => {
                let mut name_index_buf = [0; 2];
                buf_reader.read_exact(&mut name_index_buf).expect("Failed to read name index for ConstantNameAndType from class file");
                let name_index = u16::from_be_bytes(name_index_buf);

                let mut descriptor_index_buf = [0; 2];
                buf_reader.read_exact(&mut descriptor_index_buf).expect("Failed to read descriptor index for ConstantNameAndType from class file");
                let descriptor_index = u16::from_be_bytes(descriptor_index_buf);

                CpInfo::ConstantNameAndType { name_index, descriptor_index }
            }

            CpInfoTag::ConstantUtf8 => {
                let mut length_buf = [0; 2];
                buf_reader.read_exact(&mut length_buf).expect("Failed to read length for ConstantUtf8 from class file");
                let length = u16::from_be_bytes(length_buf);

                let mut bytes = vec![0; length as usize];
                buf_reader.read_exact(&mut bytes).expect("Failed to read bytes for ConstantUtf8 from class file");

                CpInfo::ConstantUtf8 { length, bytes }
            }

            CpInfoTag::ConstantMethodHandle => {
                let mut reference_kind_buf = [0; 1];
                buf_reader.read_exact(&mut reference_kind_buf).expect("Failed to read reference kind for ConstantMethodHandle from class file");
                let reference_kind = u8::from_be_bytes(reference_kind_buf);

                let mut reference_index_buf = [0; 2];
                buf_reader.read_exact(&mut reference_index_buf).expect("Failed to read reference index for ConstantMethodHandle from class file");
                let reference_index = u16::from_be_bytes(reference_index_buf);

                CpInfo::ConstantMethodHandle { reference_kind, reference_index }
            }

            CpInfoTag::ConstantMethodType => {
                let mut descriptor_index_buf = [0; 2];
                buf_reader.read_exact(&mut descriptor_index_buf).expect("Failed to read descriptor index for ConstantMethodType from class file");
                let descriptor_index = u16::from_be_bytes(descriptor_index_buf);

                CpInfo::ConstantMethodType { descriptor_index }
            }

            CpInfoTag::ConstantDynamic => {
                let mut bootstrap_method_attr_index_buf = [0; 2];
                buf_reader.read_exact(&mut bootstrap_method_attr_index_buf).expect("Failed to read bootstrap method attribute index for ConstantDynamic from class file");
                let bootstrap_method_attr_index = u16::from_be_bytes(bootstrap_method_attr_index_buf);

                let mut name_and_type_index_buf = [0; 2];
                buf_reader.read_exact(&mut name_and_type_index_buf).expect("Failed to read name and type index for ConstantDynamic from class file");
                let name_and_type_index = u16::from_be_bytes(name_and_type_index_buf);

                CpInfo::ConstantDynamic { bootstrap_method_attr_index, name_and_type_index }
            }

            CpInfoTag::ConstantInvokeDynamic => {
                let mut bootstrap_method_attr_index_buf = [0; 2];
                buf_reader.read_exact(&mut bootstrap_method_attr_index_buf).expect("Failed to read bootstrap method attribute index for ConstantInvokeDynamic from class file");
                let bootstrap_method_attr_index = u16::from_be_bytes(bootstrap_method_attr_index_buf);

                let mut name_and_type_index_buf = [0; 2];
                buf_reader.read_exact(&mut name_and_type_index_buf).expect("Failed to read name and type index for ConstantInvokeDynamic from class file");
                let name_and_type_index = u16::from_be_bytes(name_and_type_index_buf);

                CpInfo::ConstantInvokeDynamic { bootstrap_method_attr_index, name_and_type_index }
            }

            CpInfoTag::ConstantModule => {
                let mut name_index_buf = [0; 2];
                buf_reader.read_exact(&mut name_index_buf).expect("Failed to read name index for ConstantModule from class file");
                let name_index = u16::from_be_bytes(name_index_buf);
                CpInfo::ConstantModule { name_index }
            }

            CpInfoTag::ConstantPackage => {
                let mut name_index_buf = [0; 2];
                buf_reader.read_exact(&mut name_index_buf).expect("Failed to read name index for ConstantPackage from class file");
                let name_index = u16::from_be_bytes(name_index_buf);
                CpInfo::ConstantPackage { name_index }
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
    access_flags: FieldInfoAccessFlags,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

impl FieldInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R, constant_pool_entries: &Vec<CpInfo>) -> Self {

        // access flags
        let mut access_flags_buf = [0; 2];
        buf_reader.read_exact(&mut access_flags_buf).expect("Failed to read field access flags from class file");
        let access_flags = FieldInfoAccessFlags::try_from(u16::from_be_bytes(access_flags_buf)).expect("Invalid field access flags in class file");

        // name index
        let mut name_index_buf = [0; 2];
        buf_reader.read_exact(&mut name_index_buf).expect("Failed to read field name index from class file");
        let name_index = u16::from_be_bytes(name_index_buf);

        // descriptor index
        let mut descriptor_index_buf = [0; 2];
        buf_reader.read_exact(&mut descriptor_index_buf).expect("Failed to read field descriptor index from class file");
        let descriptor_index = u16::from_be_bytes(descriptor_index_buf);

        // attributes count
        let mut attributes_count_buf = [0; 2];
        buf_reader.read_exact(&mut attributes_count_buf).expect("Failed to read field attributes count from class file");
        let attributes_count = u16::from_be_bytes(attributes_count_buf);

        // attributes
        let mut attributes = Vec::with_capacity(attributes_count as usize);

        for _ in 0..attributes_count {
            let attribute_info = AttributeInfo::from_reader(buf_reader, constant_pool_entries);
            attributes.push(attribute_info);
        }

        FieldInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FieldInfoAccessFlags {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Volatile = 0x0040,
    Transient = 0x0080,
    Synthetic = 0x1000,
    Enum = 0x4000,
}

impl TryFrom<u16> for FieldInfoAccessFlags {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0001 => Ok(FieldInfoAccessFlags::Public),
            0x0002 => Ok(FieldInfoAccessFlags::Private),
            0x0004 => Ok(FieldInfoAccessFlags::Protected),
            0x0008 => Ok(FieldInfoAccessFlags::Static),
            0x0010 => Ok(FieldInfoAccessFlags::Final),
            0x0040 => Ok(FieldInfoAccessFlags::Volatile),
            0x0080 => Ok(FieldInfoAccessFlags::Transient),
            0x1000 => Ok(FieldInfoAccessFlags::Synthetic),
            0x4000 => Ok(FieldInfoAccessFlags::Enum),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct MethodInfo {
    access_flags: MethodAccessFlags,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

impl MethodInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R, constant_pool_info: &Vec<CpInfo>) -> Self {

        // access flags
        let mut access_flags_buf = [0; 2];
        buf_reader.read_exact(&mut access_flags_buf).expect("Failed to read method access flags from class file");
        let access_flags = MethodAccessFlags::from_bits(u16::from_be_bytes(access_flags_buf)).expect("Invalid method access flags in class file");

        // name index
        let mut name_index_buf = [0; 2];
        buf_reader.read_exact(&mut name_index_buf).expect("Failed to read method name index from class file");
        let name_index = u16::from_be_bytes(name_index_buf);

        // descriptor index
        let mut descriptor_index_buf = [0; 2];
        buf_reader.read_exact(&mut descriptor_index_buf).expect("Failed to read method descriptor index from class file");
        let descriptor_index = u16::from_be_bytes(descriptor_index_buf);

        // attributes count
        let mut attributes_count_buf = [0; 2];
        buf_reader.read_exact(&mut attributes_count_buf).expect("Failed to read method attributes count from class file");
        let attributes_count = u16::from_be_bytes(attributes_count_buf);

        // attributes
        let mut attributes = Vec::with_capacity(attributes_count as usize);

        for _ in 0..attributes_count {
            let attribute_info = AttributeInfo::from_reader(buf_reader, &constant_pool_info);
            attributes.push(attribute_info);
        }

        MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        }
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
    pub fn from_reader<R: BufRead>(buf_reader: &mut R, constant_pool_info: &Vec<CpInfo>) -> Self {
        let mut attribute_name_index_buf = [0; 2];
        buf_reader.read_exact(&mut attribute_name_index_buf).expect("Failed to read attribute name index from class file");
        let attribute_name_index = u16::from_be_bytes(attribute_name_index_buf);
        let attribute_name = match &constant_pool_info[attribute_name_index as usize - 1] {
            CpInfo::ConstantUtf8 { length: _, bytes } => String::from_utf8(bytes.clone()).expect("Invalid UTF-8 in attribute name in class file"),
            _ => panic!("Invalid constant pool entry for attribute name index in class file"),
        };

        let mut attribute_length_buf = [0; 4];
        buf_reader.read_exact(&mut attribute_length_buf).expect("Failed to read attribute length from class file");
        let attribute_length = u32::from_be_bytes(attribute_length_buf);

        match attribute_name.as_str() {
            "ConstantValue" => {
                let mut constant_value_index_buf = [0; 2];
                buf_reader.read_exact(&mut constant_value_index_buf).expect("Failed to read constant value index for ConstantValue attribute from class file");
                let constant_value_index = u16::from_be_bytes(constant_value_index_buf);
                AttributeInfo::ConstantValue { constant_value_index }
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
                    let entry = ExceptionTableEntry::from_reader(buf_reader);
                    exception_table.push(entry);
                }

                let mut attributes_count_buf = [0; 2];
                buf_reader.read_exact(&mut attributes_count_buf).expect("Failed to read attributes count for Code attribute from class file");
                let attributes_count = u16::from_be_bytes(attributes_count_buf);

                let mut attributes = Vec::with_capacity(attributes_count as usize);
                for _ in 0..attributes_count {
                    let attribute_info = AttributeInfo::from_reader(buf_reader, constant_pool_info);
                    attributes.push(attribute_info);
                }

                AttributeInfo::Code { max_stack, max_locals, code_length, code, exception_table_length, exception_table, attributes_count, attributes }
            }

            "StackMapTable" => {
                let mut number_of_entries_buf = [0; 2];
                buf_reader.read_exact(&mut number_of_entries_buf).expect("Failed to read number of entries for StackMapTable attribute from class file");
                let number_of_entries = u16::from_be_bytes(number_of_entries_buf);

                let mut entries = Vec::with_capacity(number_of_entries as usize);
                for _ in 0..number_of_entries {
                    let entry = StackMapFrame::from_reader(buf_reader);
                    entries.push(entry);
                }

                AttributeInfo::StackMapTable { number_of_entries, entries }
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

                AttributeInfo::Exceptions { number_of_exceptions, exception_index_table }
            }

            "InnerClasses" => {
                let mut number_of_classes_buf = [0; 2];
                buf_reader.read_exact(&mut number_of_classes_buf).expect("Failed to read number of classes for InnerClasses attribute from class file");
                let number_of_classes = u16::from_be_bytes(number_of_classes_buf);

                let mut classes = Vec::with_capacity(number_of_classes as usize);
                for _ in 0..number_of_classes {
                    let class_info = InnerClassInfo::from_reader(buf_reader);
                    classes.push(class_info);
                }

                AttributeInfo::InnerClasses { number_of_classes, classes }
            }

            "EnclosingMethod" => {
                let mut class_index_buf = [0; 2];
                buf_reader.read_exact(&mut class_index_buf).expect("Failed to read class index for EnclosingMethod attribute from class file");
                let class_index = u16::from_be_bytes(class_index_buf);

                let mut method_index_buf = [0; 2];
                buf_reader.read_exact(&mut method_index_buf).expect("Failed to read method index for EnclosingMethod attribute from class file");
                let method_index = u16::from_be_bytes(method_index_buf);

                AttributeInfo::EnclosingMethod { class_index, method_index }
            }

            "Synthetic" => AttributeInfo::Synthetic,

            "Signature" => {
                let mut signature_index_buf = [0; 2];
                buf_reader.read_exact(&mut signature_index_buf).expect("Failed to read signature index for Signature attribute from class file");
                let signature_index = u16::from_be_bytes(signature_index_buf);
                AttributeInfo::Signature { signature_index }
            }

            "SourceFile" => {
                let mut sourcefile_index_buf = [0; 2];
                buf_reader.read_exact(&mut sourcefile_index_buf).expect("Failed to read source file index for SourceFile attribute from class file");
                let sourcefile_index = u16::from_be_bytes(sourcefile_index_buf);
                AttributeInfo::SourceFile { sourcefile_index }
            }

            "SourceDebugExtension" => {
                let mut debug_extension = vec![0; attribute_length as usize];
                buf_reader.read_exact(&mut debug_extension).expect("Failed to read debug extension for SourceDebugExtension attribute from class file");
                AttributeInfo::SourceDebugExtension { debug_extension }
            }

            "LineNumberTable" => {
                let mut line_number_table_length_buf = [0; 2];
                buf_reader.read_exact(&mut line_number_table_length_buf).expect("Failed to read line number table length for LineNumberTable attribute from class file");
                let line_number_table_length = u16::from_be_bytes(line_number_table_length_buf);

                let mut line_number_table = Vec::with_capacity(line_number_table_length as usize);
                for _ in 0..line_number_table_length {
                    let entry = LineNumberInfo::from_reader(buf_reader);
                    line_number_table.push(entry);
                }

                AttributeInfo::LineNumberTable { line_number_table_length, line_number_table }
            }

            "LocalVariableTable" => {
                let mut local_variable_table_length_buf = [0; 2];
                buf_reader.read_exact(&mut local_variable_table_length_buf).expect("Failed to read local variable table length for LocalVariableTable attribute from class file");
                let local_variable_table_length = u16::from_be_bytes(local_variable_table_length_buf);

                let mut local_variable_table = Vec::with_capacity(local_variable_table_length as usize);
                for _ in 0..local_variable_table_length {
                    let entry = LocalVariableInfo::from_reader(buf_reader);
                    local_variable_table.push(entry);
                }

                AttributeInfo::LocalVariableTable { local_variable_table_length, local_variable_table }
            }

            "LocalVariableTypeTable" => {
                let mut local_variable_type_table_length_buf = [0; 2];
                buf_reader.read_exact(&mut local_variable_type_table_length_buf).expect("Failed to read local variable type table length for LocalVariableTypeTable attribute from class file");
                let local_variable_type_table_length = u16::from_be_bytes(local_variable_type_table_length_buf);

                let mut local_variable_type_table = Vec::with_capacity(local_variable_type_table_length as usize);
                for _ in 0..local_variable_type_table_length {
                    let entry = LocalVariableTypeInfo::from_reader(buf_reader);
                    local_variable_type_table.push(entry);
                }

                AttributeInfo::LocalVariableTypeTable { local_variable_type_table_length, local_variable_type_table }
            }

            "Deprecated" => AttributeInfo::Deprecated,

            "RuntimeVisibleAnnotations" => {
                let mut num_annotations_buf = [0; 2];
                buf_reader.read_exact(&mut num_annotations_buf).expect("Failed to read number of annotations for RuntimeVisibleAnnotations attribute from class file");
                let num_annotations = u16::from_be_bytes(num_annotations_buf);

                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    let annotation = Annotation::from_reader(buf_reader);
                    annotations.push(annotation);
                }

                AttributeInfo::RuntimeVisibleAnnotations { num_annotations, annotations }
            }

            "RuntimeInvisibleAnnotations" => {
                let mut num_annotations_buf = [0; 2];
                buf_reader.read_exact(&mut num_annotations_buf).expect("Failed to read number of annotations for RuntimeInvisibleAnnotations attribute from class file");
                let num_annotations = u16::from_be_bytes(num_annotations_buf);

                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    let annotation = Annotation::from_reader(buf_reader);
                    annotations.push(annotation);
                }

                AttributeInfo::RuntimeInvisibleAnnotations { num_annotations, annotations }
            }

            "RuntimeVisibleParameterAnnotations" => {
                let mut num_parameters_buf = [0; 1];
                buf_reader.read_exact(&mut num_parameters_buf).expect("Failed to read number of parameters for RuntimeVisibleParameterAnnotations attribute from class file");
                let num_parameters = u8::from_be_bytes(num_parameters_buf);

                let mut parameter_annotations = Vec::with_capacity(num_parameters as usize);
                for _ in 0..num_parameters {
                    let parameter_annotation = ParameterAnnotations::from_reader(buf_reader);
                    parameter_annotations.push(parameter_annotation);
                }

                AttributeInfo::RuntimeVisibleParameterAnnotations { num_parameters, parameter_annotations }
            }

            "RuntimeInvisibleParameterAnnotations" => {
                let mut num_parameters_buf = [0; 1];
                buf_reader.read_exact(&mut num_parameters_buf).expect("Failed to read number of parameters for RuntimeInvisibleParameterAnnotations attribute from class file");
                let num_parameters = u8::from_be_bytes(num_parameters_buf);

                let mut parameter_annotations = Vec::with_capacity(num_parameters as usize);
                for _ in 0..num_parameters {
                    let parameter_annotation = ParameterAnnotations::from_reader(buf_reader);
                    parameter_annotations.push(parameter_annotation);
                }

                AttributeInfo::RuntimeInvisibleParameterAnnotations { num_parameters, parameter_annotations }
            }

            "RuntimeVisibleTypeAnnotations" => {
                let mut num_annotations_buf = [0; 2];
                buf_reader.read_exact(&mut num_annotations_buf).expect("Failed to read number of annotations for RuntimeVisibleTypeAnnotations attribute from class file");
                let num_annotations = u16::from_be_bytes(num_annotations_buf);

                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    let annotation = TypeAnnotation::from_reader(buf_reader);
                    annotations.push(annotation);
                }

                AttributeInfo::RuntimeVisibleTypeAnnotations { num_annotations, annotations }
            }

            "RuntimeInvisibleTypeAnnotations" => {
                let mut num_annotations_buf = [0; 2];
                buf_reader.read_exact(&mut num_annotations_buf).expect("Failed to read number of annotations for RuntimeInvisibleTypeAnnotations attribute from class file");
                let num_annotations = u16::from_be_bytes(num_annotations_buf);

                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    let annotation = TypeAnnotation::from_reader(buf_reader);
                    annotations.push(annotation);
                }

                AttributeInfo::RuntimeInvisibleTypeAnnotations { num_annotations, annotations }
            }

            "AnnotationDefault" => {
                let default_value = ElementValue::from_reader(buf_reader);
                AttributeInfo::AnnotationDefault { default_value }
            }

            "BootstrapMethods" => {
                let mut num_bootstrap_methods_buf = [0; 2];
                buf_reader.read_exact(&mut num_bootstrap_methods_buf).expect("Failed to read number of bootstrap methods for BootstrapMethods attribute from class file");
                let num_bootstrap_methods = u16::from_be_bytes(num_bootstrap_methods_buf);

                let mut bootstrap_methods = Vec::with_capacity(num_bootstrap_methods as usize);
                for _ in 0..num_bootstrap_methods {
                    let bootstrap_method = BootstrapMethod::from_reader(buf_reader);
                    bootstrap_methods.push(bootstrap_method);
                }

                AttributeInfo::BootstrapMethods { num_bootstrap_methods, bootstrap_methods }
            }

            "MethodParameters" => {
                let mut parameters_count_buf = [0; 1];
                buf_reader.read_exact(&mut parameters_count_buf).expect("Failed to read parameters count for MethodParameters attribute from class file");
                let parameters_count = u8::from_be_bytes(parameters_count_buf);

                let mut parameters = Vec::with_capacity(parameters_count as usize);
                for _ in 0..parameters_count {
                    let parameter = MethodParameter::from_reader(buf_reader);
                    parameters.push(parameter);
                }

                AttributeInfo::MethodParameters { parameters_count, parameters }
            }

            "Module" => {
                let mut module_name_index_buf = [0; 2];
                buf_reader.read_exact(&mut module_name_index_buf).expect("Failed to read module name index for Module attribute from class file");
                let module_name_index = u16::from_be_bytes(module_name_index_buf);

                let mut module_flags_buf = [0; 2];
                buf_reader.read_exact(&mut module_flags_buf).expect("Failed to read module flags for Module attribute from class file");
                let module_flags = ModuleFlags::from_bits(u16::from_be_bytes(module_flags_buf)).expect("Invalid module flags in class file");

                let mut module_version_index_buf = [0; 2];
                buf_reader.read_exact(&mut module_version_index_buf).expect("Failed to read module version index for Module attribute from class file");
                let module_version_index = u16::from_be_bytes(module_version_index_buf);

                let mut requires_count_buf = [0; 2];
                buf_reader.read_exact(&mut requires_count_buf).expect("Failed to read requires count for Module attribute from class file");
                let requires_count = u16::from_be_bytes(requires_count_buf);

                let mut requires = Vec::with_capacity(requires_count as usize);
                for _ in 0..requires_count {
                    let require = ModuleRequire::from_reader(buf_reader);
                    requires.push(require);
                }

                let mut exports_count_buf = [0; 2];
                buf_reader.read_exact(&mut exports_count_buf).expect("Failed to read exports count for Module attribute from class file");
                let exports_count = u16::from_be_bytes(exports_count_buf);

                let mut exports = Vec::with_capacity(exports_count as usize);
                for _ in 0..exports_count {
                    let export = ModuleExport::from_reader(buf_reader);
                    exports.push(export);
                }

                let mut opens_count_buf = [0; 2];
                buf_reader.read_exact(&mut opens_count_buf).expect("Failed to read opens count for Module attribute from class file");
                let opens_count = u16::from_be_bytes(opens_count_buf);

                let mut opens = Vec::with_capacity(opens_count as usize);
                for _ in 0..opens_count {
                    let open = ModuleOpen::from_reader(buf_reader);
                    opens.push(open);
                }

                let mut uses_count_buf = [0; 2];
                buf_reader.read_exact(&mut uses_count_buf).expect("Failed to read uses count for Module attribute from class file");
                let uses_count = u16::from_be_bytes(uses_count_buf);

                let mut uses_index = Vec::with_capacity(uses_count as usize);
                for _ in 0..uses_count {
                    let mut use_index_buf = [0; 2];
                    buf_reader.read_exact(&mut use_index_buf).expect("Failed to read uses index for Module attribute from class file");
                    let use_index = u16::from_be_bytes(use_index_buf);
                    uses_index.push(use_index);
                }

                let mut provides_count_buf = [0; 2];
                buf_reader.read_exact(&mut provides_count_buf).expect("Failed to read provides count for Module attribute from class file");
                let provides_count = u16::from_be_bytes(provides_count_buf);

                let mut provides = Vec::with_capacity(provides_count as usize);
                for _ in 0..provides_count {
                    let provide = ModuleProvide::from_reader(buf_reader);
                    provides.push(provide);
                }

                AttributeInfo::Module { module_name_index, module_flags, module_version_index, requires_count, requires, exports_count, exports, opens_count, opens, uses_count, uses_index, provides_count, provides }
            }

            "ModulePackages" => {
                let mut package_count_buf = [0; 2];
                buf_reader.read_exact(&mut package_count_buf).expect("Failed to read package count for ModulePackages attribute from class file");
                let package_count = u16::from_be_bytes(package_count_buf);

                let mut package_index = Vec::with_capacity(package_count as usize);
                for _ in 0..package_count {
                    let mut package_index_buf = [0; 2];
                    buf_reader.read_exact(&mut package_index_buf).expect("Failed to read package index for ModulePackages attribute from class file");
                    let package_index_entry = u16::from_be_bytes(package_index_buf);
                    package_index.push(package_index_entry);
                }

                AttributeInfo::ModulePackages { package_count, package_index }
            }

            "ModuleMainClass" => {
                let mut main_class_index_buf = [0; 2];
                buf_reader.read_exact(&mut main_class_index_buf).expect("Failed to read main class index for ModuleMainClass attribute from class file");
                let main_class_index = u16::from_be_bytes(main_class_index_buf);
                AttributeInfo::ModuleMainClass { main_class_index }
            }

            "NestHost" => {
                let mut host_class_index_buf = [0; 2];
                buf_reader.read_exact(&mut host_class_index_buf).expect("Failed to read host class index for NestHost attribute from class file");
                let host_class_index = u16::from_be_bytes(host_class_index_buf);
                AttributeInfo::NestHost { host_class_index }
            }

            "NestMembers" => {
                let mut number_of_classes_buf = [0; 2];
                buf_reader.read_exact(&mut number_of_classes_buf).expect("Failed to read number of classes for NestMembers attribute from class file");
                let number_of_classes = u16::from_be_bytes(number_of_classes_buf);

                let mut classes = Vec::with_capacity(number_of_classes as usize);
                for _ in 0..number_of_classes {
                    let mut class_index_buf = [0; 2];
                    buf_reader.read_exact(&mut class_index_buf).expect("Failed to read class index for NestMembers attribute from class file");
                    let class_index = u16::from_be_bytes(class_index_buf);
                    classes.push(class_index);
                }

                AttributeInfo::NestMembers { number_of_classes, classes }
            }

            "Record" => {
                let mut number_of_components_buf = [0; 2];
                buf_reader.read_exact(&mut number_of_components_buf).expect("Failed to read number of components for Record attribute from class file");
                let number_of_components = u16::from_be_bytes(number_of_components_buf);

                let mut components = Vec::with_capacity(number_of_components as usize);
                for _ in 0..number_of_components {
                    let component = RecordComponentInfo::from_reader(buf_reader, constant_pool_info);
                    components.push(component);
                }

                AttributeInfo::Record { number_of_components, components }
            }

            "PermittedSubclasses" => {
                let mut number_of_classes_buf = [0; 2];
                buf_reader.read_exact(&mut number_of_classes_buf).expect("Failed to read number of classes for PermittedSubclasses attribute from class file");
                let number_of_classes = u16::from_be_bytes(number_of_classes_buf);

                let mut classes = Vec::with_capacity(number_of_classes as usize);
                for _ in 0..number_of_classes {
                    let mut class_index_buf = [0; 2];
                    buf_reader.read_exact(&mut class_index_buf).expect("Failed to read class index for PermittedSubclasses attribute from class file");
                    let class_index = u16::from_be_bytes(class_index_buf);
                    classes.push(class_index);
                }

                AttributeInfo::PermittedSubclasses { number_of_classes, classes }
            }


            // todo address this. Should not be a panic
            _ => panic!("Unsupported attribute name '{}' in class file this shouldnt be a panic see 4.7.2", attribute_name),   
        }

    }
}

pub enum AttributeInfoType {
    ConstantValue,
    Code,
    StackMapTable,
    Exceptions,
    InnerClasses,
    EnclosingMethod,
    Synthetic,
    Signature,
    SourceFile,
    SourceDebugExtension,
    LineNumberTable,
    LocalVariableTable,
    LocalVariableTypeTable,
    Deprecated,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    AnnotationDefault,
    BootstrapMethods,
}

impl TryFrom<&str> for AttributeInfoType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ConstantValue" => Ok(AttributeInfoType::ConstantValue),
            "Code" => Ok(AttributeInfoType::Code),
            "StackMapTable" => Ok(AttributeInfoType::StackMapTable),
            "Exceptions" => Ok(AttributeInfoType::Exceptions),
            "InnerClasses" => Ok(AttributeInfoType::InnerClasses),
            "EnclosingMethod" => Ok(AttributeInfoType::EnclosingMethod),
            "Synthetic" => Ok(AttributeInfoType::Synthetic),
            "Signature" => Ok(AttributeInfoType::Signature),
            "SourceFile" => Ok(AttributeInfoType::SourceFile),
            "SourceDebugExtension" => Ok(AttributeInfoType::SourceDebugExtension),
            "LineNumberTable" => Ok(AttributeInfoType::LineNumberTable),
            "LocalVariableTable" => Ok(AttributeInfoType::LocalVariableTable),
            "LocalVariableTypeTable" => Ok(AttributeInfoType::LocalVariableTypeTable),
            "Deprecated" => Ok(AttributeInfoType::Deprecated),
            "RuntimeVisibleAnnotations" => Ok(AttributeInfoType::RuntimeVisibleAnnotations),
            "RuntimeInvisibleAnnotations" => Ok(AttributeInfoType::RuntimeInvisibleAnnotations),
            "RuntimeVisibleParameterAnnotations" => Ok(AttributeInfoType::RuntimeVisibleParameterAnnotations),
            "RuntimeInvisibleParameterAnnotations" => Ok(AttributeInfoType::RuntimeInvisibleParameterAnnotations),
            "RuntimeVisibleTypeAnnotations" => Ok(AttributeInfoType::RuntimeVisibleTypeAnnotations),
            "RuntimeInvisibleTypeAnnotations" => Ok(AttributeInfoType::RuntimeInvisibleTypeAnnotations),
            "AnnotationDefault" => Ok(AttributeInfoType::AnnotationDefault),
            "BootstrapMethods" => Ok(AttributeInfoType::BootstrapMethods),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl ExceptionTableEntry {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Self {
        let mut start_pc_buf = [0; 2];
        buf_reader.read_exact(&mut start_pc_buf).expect("Failed to read start pc for exception table entry in Code attribute from class file");
        let start_pc = u16::from_be_bytes(start_pc_buf);

        let mut end_pc_buf = [0; 2];
        buf_reader.read_exact(&mut end_pc_buf).expect("Failed to read end pc for exception table entry in Code attribute from class file");
        let end_pc = u16::from_be_bytes(end_pc_buf);

        let mut handler_pc_buf = [0; 2];
        buf_reader.read_exact(&mut handler_pc_buf).expect("Failed to read handler pc for exception table entry in Code attribute from class file");
        let handler_pc = u16::from_be_bytes(handler_pc_buf);

        let mut catch_type_buf = [0; 2];
        buf_reader.read_exact(&mut catch_type_buf).expect("Failed to read catch type for exception table entry in Code attribute from class file");
        let catch_type = u16::from_be_bytes(catch_type_buf);

        ExceptionTableEntry {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        }
    }
}

#[derive(Debug)]
pub struct StackMapFrame {

}

impl StackMapFrame {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Self {
        todo!("StackMapFrame parsing not implemented yet");
    }
}

#[derive(Debug)]
pub struct InnerClassInfo {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: InnerClassAccessFlags,
}

impl InnerClassInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Self {
        let mut inner_class_info_index_buf = [0; 2];
        buf_reader.read_exact(&mut inner_class_info_index_buf).expect("Failed to read inner class info index for InnerClasses attribute from class file");
        let inner_class_info_index = u16::from_be_bytes(inner_class_info_index_buf);

        let mut outer_class_info_index_buf = [0; 2];
        buf_reader.read_exact(&mut outer_class_info_index_buf).expect("Failed to read outer class info index for InnerClasses attribute from class file");
        let outer_class_info_index = u16::from_be_bytes(outer_class_info_index_buf);

        let mut inner_name_index_buf = [0; 2];
        buf_reader.read_exact(&mut inner_name_index_buf).expect("Failed to read inner name index for InnerClasses attribute from class file");
        let inner_name_index = u16::from_be_bytes(inner_name_index_buf);

        let mut inner_class_access_flags_buf = [0; 2];
        buf_reader.read_exact(&mut inner_class_access_flags_buf).expect("Failed to read inner class access flags for InnerClasses attribute from class file");
        let inner_class_access_flags = InnerClassAccessFlags::from_bits(u16::from_be_bytes(inner_class_access_flags_buf)).expect("Invalid inner class access flags in class file");

        InnerClassInfo {
            inner_class_info_index,
            outer_class_info_index,
            inner_name_index,
            inner_class_access_flags,
        }
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
    start_pc: u16,
    line_number: u16,
}

impl LineNumberInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Self {
        let mut start_pc_buf = [0; 2];
        buf_reader.read_exact(&mut start_pc_buf).expect("Failed to read start pc for line number info in LineNumberTable attribute from class file");
        let start_pc = u16::from_be_bytes(start_pc_buf);

        let mut line_number_buf = [0; 2];
        buf_reader.read_exact(&mut line_number_buf).expect("Failed to read line number for line number info in LineNumberTable attribute from class file");
        let line_number = u16::from_be_bytes(line_number_buf);

        LineNumberInfo {
            start_pc,
            line_number,
        }
    }
}

#[derive(Debug)]
pub struct LocalVariableInfo {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

impl LocalVariableInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Self {
        let mut start_pc_buf = [0; 2];
        buf_reader.read_exact(&mut start_pc_buf).expect("Failed to read start pc for local variable info in LocalVariableTable attribute from class file");
        let start_pc = u16::from_be_bytes(start_pc_buf);

        let mut length_buf = [0; 2];
        buf_reader.read_exact(&mut length_buf).expect("Failed to read length for local variable info in LocalVariableTable attribute from class file");
        let length = u16::from_be_bytes(length_buf);

        let mut name_index_buf = [0; 2];
        buf_reader.read_exact(&mut name_index_buf).expect("Failed to read name index for local variable info in LocalVariableTable attribute from class file");
        let name_index = u16::from_be_bytes(name_index_buf);

        let mut descriptor_index_buf = [0; 2];
        buf_reader.read_exact(&mut descriptor_index_buf).expect("Failed to read descriptor index for local variable info in LocalVariableTable attribute from class file");
        let descriptor_index = u16::from_be_bytes(descriptor_index_buf);

        let mut index_buf = [0; 2];
        buf_reader.read_exact(&mut index_buf).expect("Failed to read index for local variable info in LocalVariableTable attribute from class file");
        let index = u16::from_be_bytes(index_buf);

        LocalVariableInfo {
            start_pc,
            length,
            name_index,
            descriptor_index,
            index,
        }
    }
}

#[derive(Debug)]
pub struct LocalVariableTypeInfo {
    start_pc: u16,
    length: u16,
    name_index: u16,
    signature_index: u16,
    index: u16,
}

impl LocalVariableTypeInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Self {
        let mut start_pc_buf = [0; 2];
        buf_reader.read_exact(&mut start_pc_buf).expect("Failed to read start pc for local variable type info in LocalVariableTypeTable attribute from class file");
        let start_pc = u16::from_be_bytes(start_pc_buf);

        let mut length_buf = [0; 2];
        buf_reader.read_exact(&mut length_buf).expect("Failed to read length for local variable type info in LocalVariableTypeTable attribute from class file");
        let length = u16::from_be_bytes(length_buf);

        let mut name_index_buf = [0; 2];
        buf_reader.read_exact(&mut name_index_buf).expect("Failed to read name index for local variable type info in LocalVariableTypeTable attribute from class file");
        let name_index = u16::from_be_bytes(name_index_buf);

        let mut signature_index_buf = [0; 2];
        buf_reader.read_exact(&mut signature_index_buf).expect("Failed to read signature index for local variable type info in LocalVariableTypeTable attribute from class file");
        let signature_index = u16::from_be_bytes(signature_index_buf);

        let mut index_buf = [0; 2];
        buf_reader.read_exact(&mut index_buf).expect("Failed to read index for local variable type info in LocalVariableTypeTable attribute from class file");
        let index = u16::from_be_bytes(index_buf);

        LocalVariableTypeInfo {
            start_pc,
            length,
            name_index,
            signature_index,
            index,
        }
    }
}

#[derive(Debug)]
pub struct Annotation {
    type_index: u16,
    num_element_value_pairs: u16,
    element_value_pairs: Vec<ElementValuePair>,
}

impl Annotation {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Self {
        let mut type_index_buf = [0; 2];
        buf_reader.read_exact(&mut type_index_buf).expect("Failed to read type index for annotation from class file");
        let type_index = u16::from_be_bytes(type_index_buf);

        let mut num_element_value_pairs_buf = [0; 2];
        buf_reader.read_exact(&mut num_element_value_pairs_buf).expect("Failed to read number of element value pairs for annotation from class file");
        let num_element_value_pairs = u16::from_be_bytes(num_element_value_pairs_buf);

        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
        for _ in 0..num_element_value_pairs {
            let mut element_name_index_buf = [0; 2];
            buf_reader.read_exact(&mut element_name_index_buf).expect("Failed to read element name index for element value pair in annotation from class file");
            let element_name_index = u16::from_be_bytes(element_name_index_buf);

            let value = ElementValue::from_reader(buf_reader);

            element_value_pairs.push(ElementValuePair { element_name_index, value });
        }

        Annotation {
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        }
    }
}

#[derive(Debug)]
pub struct ElementValuePair {
    element_name_index: u16,
    value: ElementValue,
}

#[derive(Debug)]
pub struct ParameterAnnotations {
    num_annotations: u16,
    annotations: Vec<Annotation>,
}

impl ParameterAnnotations {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Self {
        let mut num_annotations_buf = [0; 2];
        buf_reader.read_exact(&mut num_annotations_buf).expect("Failed to read number of annotations for parameter annotations from class file");
        let num_annotations = u16::from_be_bytes(num_annotations_buf);

        let mut annotations = Vec::with_capacity(num_annotations as usize);
        for _ in 0..num_annotations {
            let annotation = Annotation::from_reader(buf_reader);
            annotations.push(annotation);
        }

        ParameterAnnotations {
            num_annotations,
            annotations,
        }
    }
}

#[derive(Debug)]
pub struct TypeAnnotation {

}

impl TypeAnnotation {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Self {
        todo!("TypeAnnotation parsing not implemented yet");
    }
    
}

#[derive(Debug)]
pub struct ElementValue {

}

impl ElementValue {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Self {
        todo!("ElementValue parsing not implemented yet");
    }
}

#[derive(Debug)]
pub struct BootstrapMethod {
    bootstrap_method_ref: u16,
    num_bootstrap_arguments: u16,
    bootstrap_arguments: Vec<u16>,
}

impl BootstrapMethod {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Self {
        let mut bootstrap_method_ref_buf = [0; 2];
        buf_reader.read_exact(&mut bootstrap_method_ref_buf).expect("Failed to read bootstrap method reference for bootstrap method from class file");
        let bootstrap_method_ref = u16::from_be_bytes(bootstrap_method_ref_buf);

        let mut num_bootstrap_arguments_buf = [0; 2];
        buf_reader.read_exact(&mut num_bootstrap_arguments_buf).expect("Failed to read number of bootstrap arguments for bootstrap method from class file");
        let num_bootstrap_arguments = u16::from_be_bytes(num_bootstrap_arguments_buf);

        let mut bootstrap_arguments = Vec::with_capacity(num_bootstrap_arguments as usize);
        for _ in 0..num_bootstrap_arguments {
            let mut bootstrap_argument_buf = [0; 2];
            buf_reader.read_exact(&mut bootstrap_argument_buf).expect("Failed to read bootstrap argument for bootstrap method from class file");
            let bootstrap_argument = u16::from_be_bytes(bootstrap_argument_buf);
            bootstrap_arguments.push(bootstrap_argument);
        }

        BootstrapMethod {
            bootstrap_method_ref,
            num_bootstrap_arguments,
            bootstrap_arguments,
        }
    }
}

#[derive(Debug)]
pub struct MethodParameter {
    name_index: u16,
    access_flags: MethodParameterAccessFlags,
}

impl MethodParameter {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Self {
        let mut name_index_buf = [0; 2];
        buf_reader.read_exact(&mut name_index_buf).expect("Failed to read name index for method parameter from class file");
        let name_index = u16::from_be_bytes(name_index_buf);

        let mut access_flags_buf = [0; 2];
        buf_reader.read_exact(&mut access_flags_buf).expect("Failed to read access flags for method parameter from class file");
        let access_flags = MethodParameterAccessFlags::from_bits(u16::from_be_bytes(access_flags_buf)).expect("Invalid method parameter access flags in class file");

        MethodParameter {
            name_index,
            access_flags,
        }
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
    required_module_index: u16,
    required_flags: ModuleRequireFlags,
    required_version_index: u16,
}

impl ModuleRequire {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Self {
        let mut required_module_index_buf = [0; 2];
        buf_reader.read_exact(&mut required_module_index_buf).expect("Failed to read required module index for module require from class file");
        let required_module_index = u16::from_be_bytes(required_module_index_buf);

        let mut required_flags_buf = [0; 2];
        buf_reader.read_exact(&mut required_flags_buf).expect("Failed to read required flags for module require from class file");
        let required_flags = ModuleRequireFlags::from_bits(u16::from_be_bytes(required_flags_buf)).expect("Invalid module require flags in class file");

        let mut required_version_index_buf = [0; 2];
        buf_reader.read_exact(&mut required_version_index_buf).expect("Failed to read required version index for module require from class file");
        let required_version_index = u16::from_be_bytes(required_version_index_buf);

        ModuleRequire {
            required_module_index,
            required_flags,
            required_version_index,
        }
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
    exported_package_index: u16,
    export_flags: ModuleExportFlags,
    exported_to_count: u16,
    exported_to_index: Vec<u16>,
}

impl ModuleExport {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Self {
        let mut exported_package_index_buf = [0; 2];
        buf_reader.read_exact(&mut exported_package_index_buf).expect("Failed to read exported package index for module export from class file");
        let exported_package_index = u16::from_be_bytes(exported_package_index_buf);

        let mut export_flags_buf = [0; 2];
        buf_reader.read_exact(&mut export_flags_buf).expect("Failed to read export flags for module export from class file");
        let export_flags = ModuleExportFlags::from_bits(u16::from_be_bytes(export_flags_buf)).expect("Invalid module export flags in class file");

        let mut exported_to_count_buf = [0; 2];
        buf_reader.read_exact(&mut exported_to_count_buf).expect("Failed to read exported to count for module export from class file");
        let exported_to_count = u16::from_be_bytes(exported_to_count_buf);

        let mut exported_to_index = Vec::with_capacity(exported_to_count as usize);
        for _ in 0..exported_to_count {
            let mut exported_to_index_buf = [0; 2];
            buf_reader.read_exact(&mut exported_to_index_buf).expect("Failed to read exported to index for module export from class file");
            let exported_to_index_entry = u16::from_be_bytes(exported_to_index_buf);
            exported_to_index.push(exported_to_index_entry);
        }

        ModuleExport {
            exported_package_index,
            export_flags,
            exported_to_count,
            exported_to_index,
        }
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
    opens_index: u16,
    opens_flags: ModuleOpensFlags,
    opens_to_count: u16,
    opens_to_index: Vec<u16>,
}

impl ModuleOpen {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Self {
        let mut opens_index_buf = [0; 2];
        buf_reader.read_exact(&mut opens_index_buf).expect("Failed to read opens index for module open from class file");
        let opens_index = u16::from_be_bytes(opens_index_buf);

        let mut opens_flags_buf = [0; 2];
        buf_reader.read_exact(&mut opens_flags_buf).expect("Failed to read opens flags for module open from class file");
        let opens_flags = ModuleOpensFlags::from_bits(u16::from_be_bytes(opens_flags_buf)).expect("Invalid module opens flags in class file");

        let mut opens_to_count_buf = [0; 2];
        buf_reader.read_exact(&mut opens_to_count_buf).expect("Failed to read opens to count for module open from class file");
        let opens_to_count = u16::from_be_bytes(opens_to_count_buf);

        let mut opens_to_index = Vec::with_capacity(opens_to_count as usize);
        for _ in 0..opens_to_count {
            let mut opens_to_index_buf = [0; 2];
            buf_reader.read_exact(&mut opens_to_index_buf).expect("Failed to read opens to index for module open from class file");
            let opens_to_index_entry = u16::from_be_bytes(opens_to_index_buf);
            opens_to_index.push(opens_to_index_entry);
        }

        ModuleOpen {
            opens_index,
            opens_flags,
            opens_to_count,
            opens_to_index,
        }
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
    provided_service_index: u16,
    provides_with_count: u16,
    provides_with_index: Vec<u16>,
}

impl ModuleProvide {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R) -> Self {
        let mut provided_service_index_buf = [0; 2];
        buf_reader.read_exact(&mut provided_service_index_buf).expect("Failed to read provided service index for module provide from class file");
        let provided_service_index = u16::from_be_bytes(provided_service_index_buf);

        let mut provides_with_count_buf = [0; 2];
        buf_reader.read_exact(&mut provides_with_count_buf).expect("Failed to read provides with count for module provide from class file");
        let provides_with_count = u16::from_be_bytes(provides_with_count_buf);

        let mut provides_with_index = Vec::with_capacity(provides_with_count as usize);
        for _ in 0..provides_with_count {
            let mut provides_with_index_buf = [0; 2];
            buf_reader.read_exact(&mut provides_with_index_buf).expect("Failed to read provides with index for module provide from class file");
            let provides_with_index_entry = u16::from_be_bytes(provides_with_index_buf);
            provides_with_index.push(provides_with_index_entry);
        }

        ModuleProvide {
            provided_service_index,
            provides_with_count,
            provides_with_index,
        }
    }
}

#[derive(Debug)]
pub struct RecordComponentInfo {
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

impl RecordComponentInfo {
    pub fn from_reader<R: BufRead>(buf_reader: &mut R, constant_pool_info: &Vec<CpInfo>) -> Self {
        let mut name_index_buf = [0; 2];
        buf_reader.read_exact(&mut name_index_buf).expect("Failed to read name index for record component from class file");
        let name_index = u16::from_be_bytes(name_index_buf);

        let mut descriptor_index_buf = [0; 2];
        buf_reader.read_exact(&mut descriptor_index_buf).expect("Failed to read descriptor index for record component from class file");
        let descriptor_index = u16::from_be_bytes(descriptor_index_buf);

        let mut attributes_count_buf = [0; 2];
        buf_reader.read_exact(&mut attributes_count_buf).expect("Failed to read attributes count for record component from class file");
        let attributes_count = u16::from_be_bytes(attributes_count_buf);

        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            let attribute_info = AttributeInfo::from_reader(buf_reader, constant_pool_info);
            attributes.push(attribute_info);
        }

        RecordComponentInfo {
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        }
    }
}