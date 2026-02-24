use std::fs::File;

use mjvm::class_file::class_file::{AccessFlags, AttributeInfo, ClassFile, MethodAccessFlags};
use mjvm::const_pool::constant_pool::ConstantPoolInfo;

// ── helpers ──────────────────────────────────────────────────────────────────

fn open(fixture: &str) -> ClassFile {
    let file = File::open(format!("tests/fixtures/{}", fixture))
        .unwrap_or_else(|_| panic!("fixture '{}' not found", fixture));
    ClassFile::from_reader(file)
        .unwrap_or_else(|e| panic!("failed to parse '{}': {}", fixture, e))
}

fn cp_utf8<'a>(cp: &'a [ConstantPoolInfo], one_based: u16) -> &'a str {
    match &cp[one_based as usize - 1] {
        ConstantPoolInfo::ConstantUtf8 { bytes, .. } => {
            std::str::from_utf8(bytes).expect("CP entry is not valid UTF-8")
        }
        other => panic!("CP[{}] is not Utf8: {:?}", one_based, other),
    }
}

/// Builds a minimal valid class file whose single class-level attribute has an
/// unrecognised name. Per JVM spec §4.7.2, parsers must silently skip
/// unknown attributes by consuming `attribute_length` bytes.
fn minimal_class_with_unknown_attr() -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&[0xCA, 0xFE, 0xBA, 0xBE]); // magic
    b.extend_from_slice(&[0x00, 0x00]);               // minor
    b.extend_from_slice(&[0x00, 0x45]);               // major (69)
    // CP count = 6 in file → parser reads 5 entries (it subtracts 1)
    b.extend_from_slice(&[0x00, 0x06]);
    // CP[1] = Utf8 "TestClass"
    b.push(1); b.extend_from_slice(&[0x00, 0x09]); b.extend_from_slice(b"TestClass");
    // CP[2] = Class { name_index: 1 }
    b.push(7); b.extend_from_slice(&[0x00, 0x01]);
    // CP[3] = Utf8 "java/lang/Object"
    b.push(1); b.extend_from_slice(&[0x00, 0x10]); b.extend_from_slice(b"java/lang/Object");
    // CP[4] = Class { name_index: 3 }
    b.push(7); b.extend_from_slice(&[0x00, 0x03]);
    // CP[5] = Utf8 "UnknownAttr"
    b.push(1); b.extend_from_slice(&[0x00, 0x0B]); b.extend_from_slice(b"UnknownAttr");
    b.extend_from_slice(&[0x00, 0x21]); // access_flags (ACC_PUBLIC | ACC_SUPER)
    b.extend_from_slice(&[0x00, 0x02]); // this_class  = CP[2]
    b.extend_from_slice(&[0x00, 0x04]); // super_class = CP[4]
    b.extend_from_slice(&[0x00, 0x00]); // interfaces_count
    b.extend_from_slice(&[0x00, 0x00]); // fields_count
    b.extend_from_slice(&[0x00, 0x00]); // methods_count
    b.extend_from_slice(&[0x00, 0x01]); // attributes_count = 1
    b.extend_from_slice(&[0x00, 0x05]); // attribute_name_index = 5 → "UnknownAttr"
    b.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // attribute_length = 0
    b
}

// ── Unknown attribute ─────────────────────────────────────────────────────────
//
// JVM spec §4.7.2: "an attribute with an unrecognised name must be silently
// ignored". The parser currently panics instead.

#[test]
fn unknown_attribute_currently_panics() {
    // Catch the panic to confirm the bug is present without failing the process.
    let bytes = minimal_class_with_unknown_attr();
    let result = std::panic::catch_unwind(|| ClassFile::from_reader(bytes.as_slice()));
    assert!(result.is_err(), "Expected panic for unknown attribute (known bug)");
}

#[test]
#[ignore = "known bug: unknown attributes must be silently skipped (§4.7.2), currently panics"]
fn unknown_attribute_is_silently_skipped() {
    let bytes = minimal_class_with_unknown_attr();
    let cf = ClassFile::from_reader(bytes.as_slice())
        .expect("Unknown attributes should be silently skipped per §4.7.2");
    assert_eq!(cf.magic, 0xCAFEBABE);
    assert_eq!(cf.attributes_count, 1); // attribute was present but ignored
}

// ── WithExceptions.class ──────────────────────────────────────────────────────
//
// public void riskyMethod() throws IOException, IllegalArgumentException {}
// public void multiThrows() throws IOException, RuntimeException, ClassNotFoundException {}
//
// Exercises: Exceptions attribute

#[test]
fn with_exceptions_parses_successfully() {
    open("WithExceptions.class");
}

#[test]
fn with_exceptions_has_three_methods() {
    // <init>, riskyMethod, multiThrows
    let cf = open("WithExceptions.class");
    assert_eq!(cf.methods_count, 3);
}

#[test]
fn risky_method_has_exceptions_attribute() {
    let cf = open("WithExceptions.class");
    let method = &cf.methods[1]; // riskyMethod
    let has_exc = method.attributes.iter().any(|a| matches!(a, AttributeInfo::Exceptions { .. }));
    assert!(has_exc, "Expected Exceptions attribute on riskyMethod");
}

#[test]
fn risky_method_exceptions_count() {
    let cf = open("WithExceptions.class");
    let method = &cf.methods[1];
    let exc_attr = method.attributes.iter().find(|a| matches!(a, AttributeInfo::Exceptions { .. }));
    if let Some(AttributeInfo::Exceptions { number_of_exceptions, exception_index_table }) = exc_attr {
        assert_eq!(*number_of_exceptions, 2);
        assert_eq!(exception_index_table.len(), 2);
    } else {
        panic!("No Exceptions attribute found on riskyMethod");
    }
}

#[test]
fn risky_method_exception_class_names() {
    let cf = open("WithExceptions.class");
    let method = &cf.methods[1];
    let exc_attr = method.attributes.iter().find(|a| matches!(a, AttributeInfo::Exceptions { .. }));
    if let Some(AttributeInfo::Exceptions { exception_index_table, .. }) = exc_attr {
        // exception_index_table holds 1-based CP indices to Class entries
        let name0 = cp_class_name(&cf.constant_pool, exception_index_table[0]);
        let name1 = cp_class_name(&cf.constant_pool, exception_index_table[1]);
        assert_eq!(name0, "java/io/IOException");
        assert_eq!(name1, "java/lang/IllegalArgumentException");
    } else {
        panic!("No Exceptions attribute");
    }
}

#[test]
fn multi_throws_has_three_exceptions() {
    let cf = open("WithExceptions.class");
    let method = &cf.methods[2]; // multiThrows
    let exc_attr = method.attributes.iter().find(|a| matches!(a, AttributeInfo::Exceptions { .. }));
    if let Some(AttributeInfo::Exceptions { number_of_exceptions, exception_index_table }) = exc_attr {
        assert_eq!(*number_of_exceptions, 3);
        assert_eq!(exception_index_table.len(), 3);
        assert_eq!(cp_class_name(&cf.constant_pool, exception_index_table[0]), "java/io/IOException");
        assert_eq!(cp_class_name(&cf.constant_pool, exception_index_table[1]), "java/lang/RuntimeException");
        assert_eq!(cp_class_name(&cf.constant_pool, exception_index_table[2]), "java/lang/ClassNotFoundException");
    } else {
        panic!("No Exceptions attribute on multiThrows");
    }
}

/// Helper: resolves a 1-based Class CP index to the class name string.
fn cp_class_name<'a>(cp: &'a [ConstantPoolInfo], one_based: u16) -> &'a str {
    match &cp[one_based as usize - 1] {
        ConstantPoolInfo::ConstantClass { name_index } => cp_utf8(cp, *name_index),
        other => panic!("CP[{}] is not a Class entry: {:?}", one_based, other),
    }
}

// ── WithConstant.class ────────────────────────────────────────────────────────
//
// public static final int    MAX_SIZE = 100;
// public static final String GREETING = "Hello";
// public static final double PI       = 3.14159;
// public static final boolean FLAG    = true;
//
// Exercises: ConstantValue attribute, ConstantInteger/ConstantDouble in CP.
//
// NOTE: These tests currently fail because FieldInfoAccessFlags::try_from does
// not handle combined flags (0x0019 = Public|Static|Final). The TryFrom
// implementation needs to be replaced with a bitflags! to fix this.

#[test]
fn with_constant_parses_successfully() {
    open("WithConstant.class");
}

#[test]
fn with_constant_has_four_fields() {
    let cf = open("WithConstant.class");
    assert_eq!(cf.fields_count, 4);
    assert_eq!(cf.fields.len(), 4);
}

#[test]
fn with_constant_cp_has_integer_entry() {
    let cf = open("WithConstant.class");
    // #12 = Integer 100  (for MAX_SIZE)
    assert!(matches!(cf.constant_pool[11], ConstantPoolInfo::ConstantInteger { bytes: 100 }));
}

#[test]
fn with_constant_cp_has_double_entry() {
    let cf = open("WithConstant.class");
    // #19 = Double 3.14159
    let bits = 3.14159_f64.to_bits();
    let high = (bits >> 32) as u32;
    let low  = (bits & 0xFFFF_FFFF) as u32;
    assert!(matches!(
        cf.constant_pool[18],
        ConstantPoolInfo::ConstantDouble { high_bytes: h, low_bytes: l }
        if h == high && l == low
    ));
}

#[test]
fn max_size_field_has_constant_value_attribute() {
    let cf = open("WithConstant.class");
    let field = &cf.fields[0]; // MAX_SIZE
    assert_eq!(cp_utf8(&cf.constant_pool, field.name_index), "MAX_SIZE");
    let has_cv = field.attributes.iter().any(|a| matches!(a, AttributeInfo::ConstantValue { .. }));
    assert!(has_cv, "Expected ConstantValue attribute on MAX_SIZE");
}

#[test]
fn max_size_constant_value_index() {
    let cf = open("WithConstant.class");
    let field = &cf.fields[0];
    let cv = field.attributes.iter().find(|a| matches!(a, AttributeInfo::ConstantValue { .. }));
    if let Some(AttributeInfo::ConstantValue { constant_value_index }) = cv {
        // Should point to the Integer 100 entry (#12)
        assert!(matches!(
            cf.constant_pool[*constant_value_index as usize - 1],
            ConstantPoolInfo::ConstantInteger { bytes: 100 }
        ));
    } else {
        panic!("No ConstantValue on MAX_SIZE");
    }
}

#[test]
fn greeting_field_has_constant_value_string() {
    let cf = open("WithConstant.class");
    let field = &cf.fields[1]; // GREETING
    assert_eq!(cp_utf8(&cf.constant_pool, field.name_index), "GREETING");
    let cv = field.attributes.iter().find(|a| matches!(a, AttributeInfo::ConstantValue { .. }));
    if let Some(AttributeInfo::ConstantValue { constant_value_index }) = cv {
        // Should point to a String entry whose string_index points to "Hello"
        if let ConstantPoolInfo::ConstantString { string_index } =
            &cf.constant_pool[*constant_value_index as usize - 1]
        {
            assert_eq!(cp_utf8(&cf.constant_pool, *string_index), "Hello");
        } else {
            panic!("Expected ConstantString for GREETING");
        }
    } else {
        panic!("No ConstantValue on GREETING");
    }
}

// ── Annotated.class ───────────────────────────────────────────────────────────
//
// @Deprecated public class Annotated
//   @Deprecated public void oldMethod() {}
//
// Exercises: Deprecated attribute, RuntimeVisibleAnnotations

#[test]
fn annotated_parses_successfully() {
    open("Annotated.class");
}

#[test]
fn annotated_class_has_three_attributes() {
    let cf = open("Annotated.class");
    // SourceFile + Deprecated + RuntimeVisibleAnnotations
    assert_eq!(cf.attributes_count, 3);
}

#[test]
fn annotated_class_has_deprecated_attribute() {
    let cf = open("Annotated.class");
    let has_dep = cf.attributes.iter().any(|a| matches!(a, AttributeInfo::Deprecated));
    assert!(has_dep, "Expected Deprecated attribute on class");
}

#[test]
fn annotated_class_has_runtime_visible_annotations() {
    let cf = open("Annotated.class");
    let has_rva = cf.attributes.iter().any(|a| matches!(a, AttributeInfo::RuntimeVisibleAnnotations { .. }));
    assert!(has_rva, "Expected RuntimeVisibleAnnotations on class");
}

#[test]
fn annotated_class_runtime_annotation_is_deprecated() {
    let cf = open("Annotated.class");
    let rva = cf.attributes.iter().find(|a| matches!(a, AttributeInfo::RuntimeVisibleAnnotations { .. }));
    if let Some(AttributeInfo::RuntimeVisibleAnnotations { num_annotations, annotations }) = rva {
        assert_eq!(*num_annotations, 1);
        // annotation type_index points to Utf8 "Ljava/lang/Deprecated;"
        let type_name = cp_utf8(&cf.constant_pool, annotations[0].type_index);
        assert_eq!(type_name, "Ljava/lang/Deprecated;");
        assert_eq!(annotations[0].num_element_value_pairs, 0);
    } else {
        panic!("No RuntimeVisibleAnnotations");
    }
}

#[test]
fn old_method_has_deprecated_attribute() {
    let cf = open("Annotated.class");
    let method = &cf.methods[1]; // oldMethod
    assert_eq!(cp_utf8(&cf.constant_pool, method.name_index), "oldMethod");
    let has_dep = method.attributes.iter().any(|a| matches!(a, AttributeInfo::Deprecated));
    assert!(has_dep, "Expected Deprecated attribute on oldMethod");
}

#[test]
fn old_method_has_runtime_visible_annotations() {
    let cf = open("Annotated.class");
    let method = &cf.methods[1];
    let has_rva = method.attributes.iter().any(|a| matches!(a, AttributeInfo::RuntimeVisibleAnnotations { .. }));
    assert!(has_rva, "Expected RuntimeVisibleAnnotations on oldMethod");
}

#[test]
fn suppressed_method_has_no_annotation_attributes() {
    let cf = open("Annotated.class");
    let method = &cf.methods[2]; // suppressedMethod (@SuppressWarnings has source retention)
    assert_eq!(cp_utf8(&cf.constant_pool, method.name_index), "suppressedMethod");
    let has_anno = method.attributes.iter().any(|a| matches!(
        a,
        AttributeInfo::RuntimeVisibleAnnotations { .. }
            | AttributeInfo::RuntimeInvisibleAnnotations { .. }
            | AttributeInfo::Deprecated
    ));
    assert!(!has_anno, "suppressedMethod should have no annotation attributes");
}

// ── Generic.class ─────────────────────────────────────────────────────────────
//
// public class Generic<T>
//
// Exercises: Signature attribute, ConstantInterfaceMethodRef in CP

#[test]
fn generic_parses_successfully() {
    open("Generic.class");
}

#[test]
fn generic_cp_has_interface_methodref() {
    let cf = open("Generic.class");
    // #16 = InterfaceMethodref #17.#18  → java/util/List.add:(Ljava/lang/Object;)Z
    assert!(matches!(
        cf.constant_pool[15],
        ConstantPoolInfo::ConstantInterfaceMethodRef { class_index: 17, name_and_type_index: 18 }
    ));
}

#[test]
fn generic_class_has_signature_attribute() {
    let cf = open("Generic.class");
    let has_sig = cf.attributes.iter().any(|a| matches!(a, AttributeInfo::Signature { .. }));
    assert!(has_sig, "Expected class-level Signature attribute on Generic<T>");
}

#[test]
fn generic_class_signature_content() {
    let cf = open("Generic.class");
    let sig = cf.attributes.iter().find(|a| matches!(a, AttributeInfo::Signature { .. }));
    if let Some(AttributeInfo::Signature { signature_index }) = sig {
        // #34 = Utf8 "<T:Ljava/lang/Object;>Ljava/lang/Object;"
        let s = cp_utf8(&cf.constant_pool, *signature_index);
        assert!(s.starts_with("<T:"), "Class signature should start with type param: {}", s);
    } else {
        panic!("No class-level Signature attribute");
    }
}

#[test]
fn generic_get_value_method_has_signature() {
    let cf = open("Generic.class");
    // getValue() should have Signature "()TT;"
    let method = cf.methods.iter().find(|m| {
        cp_utf8(&cf.constant_pool, m.name_index) == "getValue"
    }).expect("getValue not found");
    let sig = method.attributes.iter().find(|a| matches!(a, AttributeInfo::Signature { .. }));
    if let Some(AttributeInfo::Signature { signature_index }) = sig {
        assert_eq!(cp_utf8(&cf.constant_pool, *signature_index), "()TT;");
    } else {
        panic!("No Signature on getValue");
    }
}

#[test]
fn generic_as_list_method_has_signature() {
    let cf = open("Generic.class");
    let method = cf.methods.iter().find(|m| {
        cp_utf8(&cf.constant_pool, m.name_index) == "asList"
    }).expect("asList not found");
    let sig = method.attributes.iter().find(|a| matches!(a, AttributeInfo::Signature { .. }));
    if let Some(AttributeInfo::Signature { signature_index }) = sig {
        let s = cp_utf8(&cf.constant_pool, *signature_index);
        assert!(s.contains("List"), "asList signature should reference List: {}", s);
    } else {
        panic!("No Signature on asList");
    }
}

// ── Nested.class / Nested$Inner.class ─────────────────────────────────────────
//
// Exercises: NestMembers, NestHost, InnerClasses, MethodParameters attributes

#[test]
fn nested_parses_successfully() {
    open("Nested.class");
}

#[test]
fn nested_class_has_nest_members_attribute() {
    let cf = open("Nested.class");
    let has_nm = cf.attributes.iter().any(|a| matches!(a, AttributeInfo::NestMembers { .. }));
    assert!(has_nm, "Expected NestMembers attribute on Nested");
}

#[test]
fn nested_nest_members_content() {
    let cf = open("Nested.class");
    let nm = cf.attributes.iter().find(|a| matches!(a, AttributeInfo::NestMembers { .. }));
    if let Some(AttributeInfo::NestMembers { number_of_classes, classes }) = nm {
        assert_eq!(*number_of_classes, 1);
        assert_eq!(classes.len(), 1);
        // classes[0] is a 1-based CP index to the Class entry for Nested$Inner
        assert_eq!(cp_class_name(&cf.constant_pool, classes[0]), "Nested$Inner");
    } else {
        panic!("No NestMembers attribute");
    }
}

#[test]
fn nested_has_inner_classes_attribute() {
    let cf = open("Nested.class");
    let has_ic = cf.attributes.iter().any(|a| matches!(a, AttributeInfo::InnerClasses { .. }));
    assert!(has_ic);
}

#[test]
fn nested_inner_parses_successfully() {
    open("Nested$Inner.class");
}

#[test]
fn nested_inner_has_nest_host_attribute() {
    let cf = open("Nested$Inner.class");
    let has_nh = cf.attributes.iter().any(|a| matches!(a, AttributeInfo::NestHost { .. }));
    assert!(has_nh, "Expected NestHost attribute on Nested$Inner");
}

#[test]
fn nested_inner_nest_host_points_to_outer_class() {
    let cf = open("Nested$Inner.class");
    let nh = cf.attributes.iter().find(|a| matches!(a, AttributeInfo::NestHost { .. }));
    if let Some(AttributeInfo::NestHost { host_class_index }) = nh {
        assert_eq!(cp_class_name(&cf.constant_pool, *host_class_index), "Nested");
    } else {
        panic!("No NestHost attribute");
    }
}

#[test]
fn nested_inner_constructor_has_method_parameters() {
    let cf = open("Nested$Inner.class");
    // The constructor (methods[0]) receives the outer class reference as a
    // synthetic/mandated parameter, which is recorded in MethodParameters.
    let constructor = &cf.methods[0];
    let has_mp = constructor.attributes.iter().any(|a| matches!(a, AttributeInfo::MethodParameters { .. }));
    assert!(has_mp, "Expected MethodParameters on Nested$Inner constructor");
}

#[test]
fn nested_inner_method_parameters_count() {
    let cf = open("Nested$Inner.class");
    let constructor = &cf.methods[0];
    let mp = constructor.attributes.iter().find(|a| matches!(a, AttributeInfo::MethodParameters { .. }));
    if let Some(AttributeInfo::MethodParameters { parameters_count, parameters }) = mp {
        assert_eq!(*parameters_count, 1);
        assert_eq!(parameters.len(), 1);
    } else {
        panic!("No MethodParameters");
    }
}

// ── AnnoWithDefault.class ─────────────────────────────────────────────────────
//
// @Retention(RUNTIME) public @interface AnnoWithDefault {
//     String value() default "hello";
//     int    count() default 0;
// }
//
// Exercises: AnnotationDefault, annotation interface access flags, interfaces,
//            RuntimeVisibleAnnotations on the annotation type itself

#[test]
fn anno_with_default_parses_successfully() {
    open("AnnoWithDefault.class");
}

#[test]
fn anno_with_default_is_annotation_interface() {
    let cf = open("AnnoWithDefault.class");
    assert!(cf.access_flags.contains(AccessFlags::Interface));
    assert!(cf.access_flags.contains(AccessFlags::Abstract));
    assert!(cf.access_flags.contains(AccessFlags::Annotation));
    assert!(cf.access_flags.contains(AccessFlags::Public));
}

#[test]
fn anno_with_default_extends_annotation() {
    let cf = open("AnnoWithDefault.class");
    // Annotation types have interfaces_count = 1: java.lang.annotation.Annotation
    assert_eq!(cf.interfaces_count, 1);
    let iface_cp_idx = cf.interfaces[0];
    assert_eq!(cp_class_name(&cf.constant_pool, iface_cp_idx), "java/lang/annotation/Annotation");
}

#[test]
fn anno_with_default_value_method_has_annotation_default() {
    let cf = open("AnnoWithDefault.class");
    let method = cf.methods.iter().find(|m| {
        cp_utf8(&cf.constant_pool, m.name_index) == "value"
    }).expect("value() not found");
    let has_ad = method.attributes.iter().any(|a| matches!(a, AttributeInfo::AnnotationDefault { .. }));
    assert!(has_ad, "Expected AnnotationDefault on value()");
}

#[test]
fn anno_with_default_count_method_has_annotation_default() {
    let cf = open("AnnoWithDefault.class");
    let method = cf.methods.iter().find(|m| {
        cp_utf8(&cf.constant_pool, m.name_index) == "count"
    }).expect("count() not found");
    let has_ad = method.attributes.iter().any(|a| matches!(a, AttributeInfo::AnnotationDefault { .. }));
    assert!(has_ad, "Expected AnnotationDefault on count()");
}

#[test]
fn anno_with_default_value_default_is_hello() {
    use mjvm::class_file::class_file::ElementValue;
    let cf = open("AnnoWithDefault.class");
    let method = cf.methods.iter().find(|m| {
        cp_utf8(&cf.constant_pool, m.name_index) == "value"
    }).unwrap();
    let ad = method.attributes.iter().find(|a| matches!(a, AttributeInfo::AnnotationDefault { .. }));
    if let Some(AttributeInfo::AnnotationDefault { default_value }) = ad {
        // Tag 's' → ConstValueIndex pointing to Utf8 "hello"
        if let ElementValue::ConstValueIndex(idx) = default_value {
            assert_eq!(cp_utf8(&cf.constant_pool, *idx), "hello");
        } else {
            panic!("Expected ConstValueIndex for string default");
        }
    } else {
        panic!("No AnnotationDefault on value()");
    }
}

#[test]
fn anno_with_default_count_default_is_zero() {
    use mjvm::class_file::class_file::ElementValue;
    let cf = open("AnnoWithDefault.class");
    let method = cf.methods.iter().find(|m| {
        cp_utf8(&cf.constant_pool, m.name_index) == "count"
    }).unwrap();
    let ad = method.attributes.iter().find(|a| matches!(a, AttributeInfo::AnnotationDefault { .. }));
    if let Some(AttributeInfo::AnnotationDefault { default_value }) = ad {
        // Tag 'I' → ConstValueIndex pointing to Integer 0
        if let ElementValue::ConstValueIndex(idx) = default_value {
            assert!(matches!(
                cf.constant_pool[*idx as usize - 1],
                ConstantPoolInfo::ConstantInteger { bytes: 0 }
            ));
        } else {
            panic!("Expected ConstValueIndex for int default");
        }
    } else {
        panic!("No AnnotationDefault on count()");
    }
}

#[test]
fn anno_with_default_class_has_runtime_visible_annotations() {
    // The annotation type itself has @Retention(RUNTIME) on it
    let cf = open("AnnoWithDefault.class");
    let has_rva = cf.attributes.iter().any(|a| matches!(a, AttributeInfo::RuntimeVisibleAnnotations { .. }));
    assert!(has_rva, "Expected RuntimeVisibleAnnotations on AnnoWithDefault");
}

#[test]
fn anno_with_default_method_access_flags_are_abstract() {
    let cf = open("AnnoWithDefault.class");
    for method in &cf.methods {
        assert!(
            method.access_flags.contains(MethodAccessFlags::Abstract),
            "All annotation interface methods must be abstract"
        );
    }
}
