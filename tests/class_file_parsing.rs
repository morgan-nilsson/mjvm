use std::fs::File;

use mjvm::class_file::{AccessFlags, AttributeInfo, ClassFile, MethodAccessFlags};
use mjvm::constant_pool::ConstantPoolInfo;

// ── helpers ──────────────────────────────────────────────────────────────────

fn open(fixture: &str) -> ClassFile {
    let file = File::open(format!("tests/fixtures/{}", fixture))
        .unwrap_or_else(|_| panic!("fixture '{}' not found", fixture));
    ClassFile::from_reader(file)
        .unwrap_or_else(|e| panic!("failed to parse '{}': {}", fixture, e))
}

/// Returns the UTF-8 string at a 1-based constant pool index.
fn cp_utf8<'a>(cp: &'a [ConstantPoolInfo], one_based: u16) -> &'a str {
    match &cp[one_based as usize - 1] {
        ConstantPoolInfo::ConstantUtf8 { bytes, .. } => {
            std::str::from_utf8(bytes).expect("CP entry is not valid UTF-8")
        }
        other => panic!("CP[{}] is not Utf8: {:?}", one_based, other),
    }
}

// ── Basic.class ───────────────────────────────────────────────────────────────
//
// public class Basic {
//     public static void main(String[] args) { return; }
// }
//
// javac: minor=0, major=69 (Java 25), 14 CP entries, 0 interfaces,
//        0 fields, 2 methods (<init> + main), 1 class attribute (SourceFile)

#[test]
fn basic_magic_number() {
    assert_eq!(open("Basic.class").magic, 0xCAFEBABE);
}

#[test]
fn basic_version_numbers() {
    let cf = open("Basic.class");
    assert_eq!(cf.minor_version, 0);
    assert_eq!(cf.major_version, 69); // Java 25
}

#[test]
fn basic_constant_pool_count() {
    let cf = open("Basic.class");
    assert_eq!(cf.constant_pool_count, 14);
    assert_eq!(cf.constant_pool.len(), 14);
}

#[test]
fn basic_access_flags() {
    let cf = open("Basic.class");
    assert!(cf.access_flags.contains(AccessFlags::Public));
    assert!(cf.access_flags.contains(AccessFlags::Super));
    assert!(!cf.access_flags.contains(AccessFlags::Interface));
    assert!(!cf.access_flags.contains(AccessFlags::Abstract));
    assert!(!cf.access_flags.contains(AccessFlags::Enum));
}

#[test]
fn basic_this_and_super_class_indices() {
    let cf = open("Basic.class");
    // #7 = Class "Basic",  #2 = Class "java/lang/Object"
    assert_eq!(cf.this_class, 7);
    assert_eq!(cf.super_class, 2);
}

#[test]
fn basic_no_interfaces() {
    let cf = open("Basic.class");
    assert_eq!(cf.interfaces_count, 0);
    assert!(cf.interfaces.is_empty());
}

#[test]
fn basic_no_fields() {
    let cf = open("Basic.class");
    assert_eq!(cf.fields_count, 0);
    assert!(cf.fields.is_empty());
}

#[test]
fn basic_method_count() {
    let cf = open("Basic.class");
    assert_eq!(cf.methods_count, 2);
    assert_eq!(cf.methods.len(), 2);
}

// CP structure tests

#[test]
fn basic_cp_methodref_entry() {
    let cf = open("Basic.class");
    // #1 = Methodref #2.#3  →  java/lang/Object."<init>":()V
    assert!(matches!(
        cf.constant_pool[0],
        ConstantPoolInfo::ConstantMethodRef { class_index: 2, name_and_type_index: 3 }
    ));
}

#[test]
fn basic_cp_class_entries() {
    let cf = open("Basic.class");
    // #2 = Class #4 (java/lang/Object),  #7 = Class #8 (Basic)
    assert!(matches!(cf.constant_pool[1], ConstantPoolInfo::ConstantClass { name_index: 4 }));
    assert!(matches!(cf.constant_pool[6], ConstantPoolInfo::ConstantClass { name_index: 8 }));
}

#[test]
fn basic_cp_class_names() {
    let cf = open("Basic.class");
    assert_eq!(cp_utf8(&cf.constant_pool, 4), "java/lang/Object");
    assert_eq!(cp_utf8(&cf.constant_pool, 8), "Basic");
}

#[test]
fn basic_cp_name_and_type_entry() {
    let cf = open("Basic.class");
    // #3 = NameAndType #5:#6  →  "<init>":()V
    assert!(matches!(
        cf.constant_pool[2],
        ConstantPoolInfo::ConstantNameAndType { name_index: 5, descriptor_index: 6 }
    ));
}

#[test]
fn basic_cp_method_names() {
    let cf = open("Basic.class");
    // #5 = Utf8 "<init>",  #11 = Utf8 "main"
    assert_eq!(cp_utf8(&cf.constant_pool, 5), "<init>");
    assert_eq!(cp_utf8(&cf.constant_pool, 11), "main");
}

#[test]
fn basic_cp_method_descriptors() {
    let cf = open("Basic.class");
    // #6 = Utf8 "()V",  #12 = Utf8 "([Ljava/lang/String;)V"
    assert_eq!(cp_utf8(&cf.constant_pool, 6), "()V");
    assert_eq!(cp_utf8(&cf.constant_pool, 12), "([Ljava/lang/String;)V");
}

// Method tests

#[test]
fn basic_constructor_access_flags() {
    let cf = open("Basic.class");
    let init = &cf.methods[0];
    assert!(init.access_flags.contains(MethodAccessFlags::Public));
    assert!(!init.access_flags.contains(MethodAccessFlags::Static));
}

#[test]
fn basic_main_access_flags() {
    let cf = open("Basic.class");
    let main = &cf.methods[1];
    assert!(main.access_flags.contains(MethodAccessFlags::Public));
    assert!(main.access_flags.contains(MethodAccessFlags::Static));
}

#[test]
fn basic_constructor_has_code_attribute() {
    let cf = open("Basic.class");
    let init = &cf.methods[0];
    assert_eq!(init.attributes_count, 1);
    assert!(matches!(init.attributes[0], AttributeInfo::Code { .. }));
}

#[test]
fn basic_constructor_code_details() {
    let cf = open("Basic.class");
    let init = &cf.methods[0];
    if let AttributeInfo::Code { max_stack, max_locals, code_length, code, exception_table_length, .. } =
        &init.attributes[0]
    {
        assert_eq!(*max_stack, 1);
        assert_eq!(*max_locals, 1);
        assert_eq!(*code_length, 5);
        assert_eq!(code.len(), 5);
        assert_eq!(*exception_table_length, 0);
        assert_eq!(code[0], 0x2a); // aload_0
        assert_eq!(code[1], 0xb7); // invokespecial
        assert_eq!(code[4], 0xb1); // return
    } else {
        panic!("Expected Code attribute on constructor");
    }
}

#[test]
fn basic_main_code_details() {
    let cf = open("Basic.class");
    let main = &cf.methods[1];
    if let AttributeInfo::Code { max_stack, max_locals, code_length, code, exception_table_length, .. } =
        &main.attributes[0]
    {
        assert_eq!(*max_stack, 0);
        assert_eq!(*max_locals, 1);
        assert_eq!(*code_length, 1);
        assert_eq!(code.len(), 1);
        assert_eq!(*exception_table_length, 0);
        assert_eq!(code[0], 0xb1); // return
    } else {
        panic!("Expected Code attribute on main");
    }
}

#[test]
fn basic_code_attribute_contains_line_number_table() {
    let cf = open("Basic.class");
    let main = &cf.methods[1];
    if let AttributeInfo::Code { attributes, attributes_count, .. } = &main.attributes[0] {
        assert_eq!(*attributes_count, 1);
        assert!(matches!(attributes[0], AttributeInfo::LineNumberTable { .. }));
    } else {
        panic!("Expected Code attribute on main");
    }
}

#[test]
fn basic_line_number_table_content() {
    let cf = open("Basic.class");
    let main = &cf.methods[1];
    if let AttributeInfo::Code { attributes, .. } = &main.attributes[0] {
        if let AttributeInfo::LineNumberTable { line_number_table_length, line_number_table } =
            &attributes[0]
        {
            assert_eq!(*line_number_table_length, 1);
            assert_eq!(line_number_table[0].start_pc, 0);
            assert_eq!(line_number_table[0].line_number, 3); // `return;` is line 3
        } else {
            panic!("Expected LineNumberTable");
        }
    } else {
        panic!("Expected Code attribute");
    }
}

// Class-level attribute tests

#[test]
fn basic_class_attribute_count() {
    let cf = open("Basic.class");
    assert_eq!(cf.attributes_count, 1);
}

#[test]
fn basic_sourcefile_attribute() {
    let cf = open("Basic.class");
    // #13 = Utf8 "SourceFile",  #14 = Utf8 "Basic.java"
    assert!(matches!(
        cf.attributes[0],
        AttributeInfo::SourceFile { sourcefile_index: 14 }
    ));
}

#[test]
fn basic_sourcefile_name() {
    let cf = open("Basic.class");
    if let AttributeInfo::SourceFile { sourcefile_index } = cf.attributes[0] {
        assert_eq!(cp_utf8(&cf.constant_pool, sourcefile_index), "Basic.java");
    } else {
        panic!("Expected SourceFile attribute");
    }
}

// ── Print.class ───────────────────────────────────────────────────────────────
//
// public class Print {
//     public static void main(String[] args) { System.out.println("Hello, world!"); }
// }
//
// Exercises: ConstantString, ConstantFieldRef (System.out), ConstantMethodRef (println)

#[test]
fn print_parses_successfully() {
    open("Print.class");
}

#[test]
fn print_magic_number() {
    assert_eq!(open("Print.class").magic, 0xCAFEBABE);
}

#[test]
fn print_constant_pool_count() {
    let cf = open("Print.class");
    assert_eq!(cf.constant_pool_count, 28);
    assert_eq!(cf.constant_pool.len(), 28);
}

#[test]
fn print_cp_has_string_constant() {
    let cf = open("Print.class");
    // #13 = String #14  →  "Hello, world!"
    assert!(matches!(
        cf.constant_pool[12],
        ConstantPoolInfo::ConstantString { string_index: 14 }
    ));
}

#[test]
fn print_cp_string_content() {
    let cf = open("Print.class");
    // #14 = Utf8 "Hello, world!"
    assert_eq!(cp_utf8(&cf.constant_pool, 14), "Hello, world!");
}

#[test]
fn print_cp_fieldref_for_system_out() {
    let cf = open("Print.class");
    // #7 = Fieldref #8.#9  →  java/lang/System.out:Ljava/io/PrintStream;
    assert!(matches!(
        cf.constant_pool[6],
        ConstantPoolInfo::ConstantFieldRef { class_index: 8, name_and_type_index: 9 }
    ));
}

#[test]
fn print_cp_system_class_name() {
    let cf = open("Print.class");
    // #8 = Class #10,  #10 = Utf8 "java/lang/System"
    assert_eq!(cp_utf8(&cf.constant_pool, 10), "java/lang/System");
}

#[test]
fn print_cp_methodref_for_println() {
    let cf = open("Print.class");
    // #15 = Methodref #16.#17  →  java/io/PrintStream.println:(Ljava/lang/String;)V
    assert!(matches!(
        cf.constant_pool[14],
        ConstantPoolInfo::ConstantMethodRef { class_index: 16, name_and_type_index: 17 }
    ));
    assert_eq!(cp_utf8(&cf.constant_pool, 19), "println");
    assert_eq!(cp_utf8(&cf.constant_pool, 20), "(Ljava/lang/String;)V");
}

#[test]
fn print_this_class_name() {
    let cf = open("Print.class");
    // this_class = #21 = Class #22,  #22 = Utf8 "Print"
    assert_eq!(cf.this_class, 21);
    if let ConstantPoolInfo::ConstantClass { name_index } = &cf.constant_pool[20] {
        assert_eq!(cp_utf8(&cf.constant_pool, *name_index), "Print");
    } else {
        panic!("Expected ConstantClass at CP index 21");
    }
}

#[test]
fn print_sourcefile_attribute() {
    let cf = open("Print.class");
    // #27 = Utf8 "SourceFile",  #28 = Utf8 "Print.java"
    assert!(matches!(
        cf.attributes[0],
        AttributeInfo::SourceFile { sourcefile_index: 28 }
    ));
}

#[test]
fn print_main_bytecode() {
    let cf = open("Print.class");
    let main = &cf.methods[1];
    if let AttributeInfo::Code { max_stack, code, .. } = &main.attributes[0] {
        assert_eq!(*max_stack, 2);
        assert_eq!(code[0], 0xb2); // getstatic
        assert_eq!(code[3], 0x12); // ldc
        assert_eq!(code[5], 0xb6); // invokevirtual
        assert_eq!(code[8], 0xb1); // return
    } else {
        panic!("Expected Code attribute on main");
    }
}

// ── Sum.class ─────────────────────────────────────────────────────────────────
//
// Exercises: ConstantInvokeDynamic, ConstantMethodHandle, BootstrapMethods
//            attribute, InnerClasses attribute

#[test]
fn sum_parses_successfully() {
    open("Sum.class");
}

#[test]
fn sum_constant_pool_count() {
    let cf = open("Sum.class");
    assert_eq!(cf.constant_pool_count, 45);
    assert_eq!(cf.constant_pool.len(), 45);
}

#[test]
fn sum_cp_has_invokedynamic() {
    let cf = open("Sum.class");
    // #13 = InvokeDynamic #0:#14  →  makeConcatWithConstants:(III)Ljava/lang/String;
    assert!(matches!(
        cf.constant_pool[12],
        ConstantPoolInfo::ConstantInvokeDynamic { bootstrap_method_attr_index: 0, name_and_type_index: 14 }
    ));
}

#[test]
fn sum_cp_has_methodhandle() {
    let cf = open("Sum.class");
    // #34 = MethodHandle 6:#35  →  REF_invokeStatic StringConcatFactory.makeConcatWithConstants
    assert!(matches!(
        cf.constant_pool[33],
        ConstantPoolInfo::ConstantMethodHandle { reference_kind: 6, reference_index: 35 }
    ));
}

#[test]
fn sum_cp_string_template() {
    let cf = open("Sum.class");
    // #33 = Utf8 "The sum of \u0001 and \u0001 is: \u0001"
    // \u0001 is the indy concat placeholder character
    let s = cp_utf8(&cf.constant_pool, 33);
    assert!(s.starts_with("The sum of "));
    assert!(s.ends_with("is: \u{0001}"));
}

#[test]
fn sum_class_attribute_count() {
    let cf = open("Sum.class");
    assert_eq!(cf.attributes_count, 3); // SourceFile + BootstrapMethods + InnerClasses
}

#[test]
fn sum_has_sourcefile_attribute() {
    let cf = open("Sum.class");
    let has_sf = cf.attributes.iter().any(|a| matches!(a, AttributeInfo::SourceFile { .. }));
    assert!(has_sf);
}

#[test]
fn sum_has_bootstrap_methods_attribute() {
    let cf = open("Sum.class");
    let has_bm = cf.attributes.iter().any(|a| matches!(a, AttributeInfo::BootstrapMethods { .. }));
    assert!(has_bm, "Expected BootstrapMethods attribute on class");
}

#[test]
fn sum_bootstrap_methods_content() {
    let cf = open("Sum.class");
    let bm_attr = cf
        .attributes
        .iter()
        .find(|a| matches!(a, AttributeInfo::BootstrapMethods { .. }));
    if let Some(AttributeInfo::BootstrapMethods { num_bootstrap_methods, bootstrap_methods }) = bm_attr {
        assert_eq!(*num_bootstrap_methods, 1);
        assert_eq!(bootstrap_methods.len(), 1);
        assert_eq!(bootstrap_methods[0].bootstrap_method_ref, 34); // #34 MethodHandle
        assert_eq!(bootstrap_methods[0].num_bootstrap_arguments, 1);
        assert_eq!(bootstrap_methods[0].bootstrap_arguments[0], 32); // #32 String template
    } else {
        panic!("No BootstrapMethods attribute found");
    }
}

#[test]
fn sum_has_inner_classes_attribute() {
    let cf = open("Sum.class");
    let has_ic = cf.attributes.iter().any(|a| matches!(a, AttributeInfo::InnerClasses { .. }));
    assert!(has_ic, "Expected InnerClasses attribute on class");
}

#[test]
fn sum_inner_classes_content() {
    let cf = open("Sum.class");
    let ic_attr = cf
        .attributes
        .iter()
        .find(|a| matches!(a, AttributeInfo::InnerClasses { .. }));
    if let Some(AttributeInfo::InnerClasses { number_of_classes, classes }) = ic_attr {
        // MethodHandles$Lookup inner class reference
        assert_eq!(*number_of_classes, 1);
        assert_eq!(classes.len(), 1);
    } else {
        panic!("No InnerClasses attribute found");
    }
}

// ── Chess.class ───────────────────────────────────────────────────────────────
//
// Exercises: instance field, StackMapTable inside a Code attribute
//
// NOTE: Chess.board has access flags 0x0000 (package-private / default access).
// FieldInfoAccessFlags::try_from currently does not handle 0x0000, so these
// tests will fail until that case is added to the TryFrom implementation.

#[test]
fn chess_parses_successfully() {
    open("Chess.class");
}

#[test]
fn chess_has_one_field() {
    let cf = open("Chess.class");
    assert_eq!(cf.fields_count, 1);
    assert_eq!(cf.fields.len(), 1);
}

#[test]
fn chess_field_name() {
    let cf = open("Chess.class");
    let field = &cf.fields[0];
    assert_eq!(cp_utf8(&cf.constant_pool, field.name_index), "board");
}

#[test]
fn chess_field_descriptor() {
    let cf = open("Chess.class");
    let field = &cf.fields[0];
    // [[C = 2-dimensional char array
    assert_eq!(cp_utf8(&cf.constant_pool, field.descriptor_index), "[[C");
}

#[test]
fn chess_has_two_methods() {
    let cf = open("Chess.class");
    assert_eq!(cf.methods_count, 2);
    assert_eq!(cf.methods.len(), 2);
}

#[test]
fn chess_constant_pool_count() {
    let cf = open("Chess.class");
    assert_eq!(cf.constant_pool_count, 39);
}

#[test]
fn chess_code_attribute_has_stack_map_table() {
    let cf = open("Chess.class");
    // The initializeBoard method has a loop → StackMapTable required
    let has_smt = cf.methods.iter().any(|m| {
        m.attributes.iter().any(|a| {
            if let AttributeInfo::Code { attributes, .. } = a {
                attributes.iter().any(|ca| matches!(ca, AttributeInfo::StackMapTable { .. }))
            } else {
                false
            }
        })
    });
    assert!(has_smt, "Expected a StackMapTable inside a Code attribute");
}

// ── Error / edge-case tests ───────────────────────────────────────────────────

#[test]
fn rejects_extra_trailing_bytes() {
    let mut bytes = std::fs::read("tests/fixtures/Basic.class").unwrap();
    bytes.push(0xFF);
    let result = ClassFile::from_reader(bytes.as_slice());
    assert!(result.is_err(), "Expected error for trailing bytes after class file");
}
