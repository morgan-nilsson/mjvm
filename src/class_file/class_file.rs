use std::io::{BufRead, BufReader, Read};

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
        let constant_pool_count = u16::from_be_bytes(constant_pool_count_buf);

        // constant pool
        let mut constant_pool = Vec::with_capacity(constant_pool_count as usize);

        for _ in 0..constant_pool_count {
            let cp_info = CpInfo::from_reader(&mut buf_reader);
            constant_pool.push(cp_info);
        }

        // access flags
        let mut access_flags_buf = [0; 2];
        buf_reader.read_exact(&mut access_flags_buf).expect("Failed to read access flags from class file");
        let access_flags = AccessFlags::try_from(u16::from_be_bytes(access_flags_buf)).expect("Invalid access flags in class file");

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
            let field_info = FieldInfo::from_reader(&mut buf_reader);
            fields.push(field_info);
        }

        // methods count
        let mut methods_count_buf = [0; 2];
        buf_reader.read_exact(&mut methods_count_buf).expect("Failed to read methods count from class file");
        let methods_count = u16::from_be_bytes(methods_count_buf);

        // methods
        let mut methods = Vec::with_capacity(methods_count as usize);

        for _ in 0..methods_count {
            let method_info = MethodInfo::from_reader(&mut buf_reader);
            methods.push(method_info);
        }

        // attributes count
        let mut attributes_count_buf = [0; 2];
        buf_reader.read_exact(&mut attributes_count_buf).expect("Failed to read attributes count from class file");
        let attributes_count = u16::from_be_bytes(attributes_count_buf);

        // attributes
        let mut attributes = Vec::with_capacity(attributes_count as usize);

        for _ in 0..attributes_count {
            let attribute_info = AttributeInfo::from_reader(&mut buf_reader);
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
    pub fn from_reader<R: Read>(reader: R) -> Self {
        let mut buf_reader = BufReader::new(reader);
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

pub enum AccessFlags {
    Public = 0x0001,
    Final = 0x0010,
    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
}

impl TryFrom<u16> for AccessFlags {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0001 => Ok(AccessFlags::Public),
            0x0010 => Ok(AccessFlags::Final),
            0x0020 => Ok(AccessFlags::Super),
            0x0200 => Ok(AccessFlags::Interface),
            0x0400 => Ok(AccessFlags::Abstract),
            0x1000 => Ok(AccessFlags::Synthetic),
            0x2000 => Ok(AccessFlags::Annotation),
            0x4000 => Ok(AccessFlags::Enum),
            _ => Err(()),
        }
    }
}


pub struct FieldInfo {
    access_flags: FieldInfoAccessFlags,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

impl FieldInfo {
    pub fn from_reader<R: Read>(reader: R) -> Self {
        let mut buf_reader = BufReader::new(reader);

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
            let attribute_info = AttributeInfo::from_reader(&mut buf_reader);
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

pub struct MethodInfo {
    access_flags: MethodInfoAccessFlags,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

impl MethodInfo {
    pub fn from_reader<R: Read>(reader: R) -> Self {
        let mut buf_reader = BufReader::new(reader);

        // access flags
        let mut access_flags_buf = [0; 2];
        buf_reader.read_exact(&mut access_flags_buf).expect("Failed to read method access flags from class file");
        let access_flags = MethodInfoAccessFlags::try_from(u16::from_be_bytes(access_flags_buf)).expect("Invalid method access flags in class file");

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
            let attribute_info = AttributeInfo::from_reader(&mut buf_reader);
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

pub enum MethodInfoAccessFlags {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Synchronized = 0x0020,
    Bridge = 0x0040,
    Varargs = 0x0080,
    Native = 0x0100,
    Abstract = 0x0400,
    Strict = 0x0800,
    Synthetic = 0x1000,
}

impl TryFrom<u16> for MethodInfoAccessFlags {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0001 => Ok(MethodInfoAccessFlags::Public),
            0x0002 => Ok(MethodInfoAccessFlags::Private),
            0x0004 => Ok(MethodInfoAccessFlags::Protected),
            0x0008 => Ok(MethodInfoAccessFlags::Static),
            0x0010 => Ok(MethodInfoAccessFlags::Final),
            0x0020 => Ok(MethodInfoAccessFlags::Synchronized),
            0x0040 => Ok(MethodInfoAccessFlags::Bridge),
            0x0080 => Ok(MethodInfoAccessFlags::Varargs),
            0x0100 => Ok(MethodInfoAccessFlags::Native),
            0x0400 => Ok(MethodInfoAccessFlags::Abstract),
            0x0800 => Ok(MethodInfoAccessFlags::Strict),
            0x1000 => Ok(MethodInfoAccessFlags::Synthetic),
            _ => Err(()),
        }
    }
}

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
    // TODO: ElementValue
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum ModuleFlags {
    OPEN = 0x0020,
    SYNTHETIC = 0x1000,
    MANDATED = 0x8000,
}

impl AttributeInfo {
    pub fn from_reader<R: Read>(reader: R) -> Self {
        todo!("AttributeInfo parsing not implemented yet");

        let mut buf_reader = BufReader::new(reader);
        let mut attribute_name_index_buf = [0; 2];
        buf_reader.read_exact(&mut attribute_name_index_buf).expect("Failed to read attribute name index from class file");
        let attribute_name_index = u16::from_be_bytes(attribute_name_index_buf);

        let mut attribute_length_buf = [0; 4];
        buf_reader.read_exact(&mut attribute_length_buf).expect("Failed to read attribute length from class file");
        let attribute_length = u32::from_be_bytes(attribute_length_buf);

        match attribute_name_index {
            _ => panic!("AttributeInfo parsing not implemented yet"),
            
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

pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

pub struct StackMapFrame {

}

pub struct InnerClassInfo {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: InnerClassAccessFlags,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum InnerClassAccessFlags {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
}

pub struct LineNumberInfo {
    start_pc: u16,
    line_number: u16,
}

pub struct LocalVariableInfo {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

pub struct LocalVariableTypeInfo {
    start_pc: u16,
    length: u16,
    name_index: u16,
    signature_index: u16,
    index: u16,
}

pub struct Annotation {
    type_index: u16,
    num_element_value_pairs: u16,
    element_value_pairs: Vec<ElementValuePair>,
}

pub struct ElementValuePair {
    element_name_index: u16,
    value: ElementValue,
}

pub struct ParameterAnnotations {
    num_annotations: u16,
    annotations: Vec<Annotation>,
}

pub struct TypeAnnotation {

}

pub struct ElementValue {

}

pub struct BootstrapMethod {
    bootstrap_method_ref: u16,
    num_bootstrap_arguments: u16,
    bootstrap_arguments: Vec<u16>,
}

pub struct MethodParameter {
    name_index: u16,
    access_flags: MethodParameterAccessFlags,
}

pub enum MethodParameterAccessFlags {
    Final = 0x0010,
    Synthetic = 0x1000,
    Mandated = 0x8000,
}

pub struct ModuleRequire {
    required_module_index: u16,
    required_flags: ModuleRequireFlags,
    required_version_index: u16,
}

pub enum ModuleRequireFlags {
    Transitive = 0x0020,
    StaticPhase = 0x0040,
    Synthetic = 0x1000,
    Mandated = 0x8000,
}

pub struct ModuleExport {
    exported_package_index: u16,
    export_flags: ModuleExportFlags,
    exported_to_count: u16,
    exported_to_index: Vec<u16>,
}

pub enum ModuleExportFlags {
    Synthetic = 0x1000,
    Mandated = 0x8000,
}

pub struct ModuleOpen {
    opens_index: u16,
    opens_flags: ModuleOpensFlags,
    opens_to_count: u16,
    opens_to_index: Vec<u16>,
}

pub enum ModuleOpensFlags {
    Synthetic = 0x1000,
    Mandated = 0x8000,
}

pub struct ModuleProvide {
    provided_service_index: u16,
    provides_with_count: u16,
    provides_with_index: Vec<u16>,
}

pub struct RecordComponentInfo {
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}