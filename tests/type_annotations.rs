use std::fs::File;

use mjvm::class_file::class_file::ClassFile;

fn open(fixture: &str) -> ClassFile {
    let file = File::open(format!("tests/fixtures/{}", fixture))
        .unwrap_or_else(|_| panic!("fixture '{}' not found", fixture));
    ClassFile::from_reader(file)
        .unwrap_or_else(|e| panic!("failed to parse '{}': {}", fixture, e))
}

// ── TypeAnnotated.class ───────────────────────────────────────────────────────
//
// public class TypeAnnotated {
//     public @TypeAnno String getGreeting() { ... }  // METHOD_RETURN type annotation
//     public @TypeAnno int add(@TypeAnno int a, @TypeAnno int b) { ... }
// }
//
// Both methods carry RuntimeVisibleTypeAnnotations, which triggers
// TypeAnnotation::from_reader — currently implemented as todo!().
//
// These tests document the gap. The `_currently_panics` variant passes by
// catching the panic, proving the bug is real. The `_parses_successfully`
// variant is ignored until the implementation is complete.

#[test]
fn type_annotated_currently_panics() {
    // TypeAnnotation::from_reader hits todo!(), causing a panic.
    let file = File::open("tests/fixtures/TypeAnnotated.class")
        .expect("fixture not found");
    let result = std::panic::catch_unwind(|| ClassFile::from_reader(file));
    assert!(
        result.is_err(),
        "Expected a panic from the unimplemented TypeAnnotation parser"
    );
}

#[test]
#[ignore = "known bug: TypeAnnotation::from_reader is not yet implemented (todo!())"]
fn type_annotated_parses_successfully() {
    open("TypeAnnotated.class");
}

#[test]
#[ignore = "known bug: TypeAnnotation::from_reader is not yet implemented (todo!())"]
fn type_annotated_has_three_methods() {
    let cf = open("TypeAnnotated.class");
    // <init>, getGreeting, add
    assert_eq!(cf.methods_count, 3);
}

#[test]
#[ignore = "known bug: TypeAnnotation::from_reader is not yet implemented (todo!())"]
fn get_greeting_has_runtime_visible_type_annotations() {
    use mjvm::class_file::class_file::AttributeInfo;
    let cf = open("TypeAnnotated.class");
    let method = cf.methods.iter().find(|m| {
        let name_entry = &cf.constant_pool[m.name_index as usize - 1];
        if let mjvm::const_pool::constant_pool::ConstantPoolInfo::ConstantUtf8 { bytes, .. } = name_entry {
            std::str::from_utf8(bytes).unwrap_or("") == "getGreeting"
        } else {
            false
        }
    }).expect("getGreeting not found");

    // The Code attribute's inner attributes should contain RuntimeVisibleTypeAnnotations
    let has_rvta = method.attributes.iter().any(|a| {
        if let AttributeInfo::Code { attributes, .. } = a {
            attributes.iter().any(|ca| matches!(ca, AttributeInfo::RuntimeVisibleTypeAnnotations { .. }))
        } else {
            false
        }
    });
    assert!(has_rvta, "Expected RuntimeVisibleTypeAnnotations inside Code of getGreeting");
}

#[test]
#[ignore = "known bug: TypeAnnotation::from_reader is not yet implemented (todo!())"]
fn get_greeting_type_annotation_is_method_return() {
    use mjvm::class_file::class_file::AttributeInfo;
    let cf = open("TypeAnnotated.class");
    let method = cf.methods.iter().find(|m| {
        let name_entry = &cf.constant_pool[m.name_index as usize - 1];
        if let mjvm::const_pool::constant_pool::ConstantPoolInfo::ConstantUtf8 { bytes, .. } = name_entry {
            std::str::from_utf8(bytes).unwrap_or("") == "getGreeting"
        } else {
            false
        }
    }).expect("getGreeting not found");

    for attr in &method.attributes {
        if let AttributeInfo::Code { attributes, .. } = attr {
            for code_attr in attributes {
                if let AttributeInfo::RuntimeVisibleTypeAnnotations { num_annotations, annotations } = code_attr {
                    assert_eq!(*num_annotations, 1);
                    assert_eq!(annotations.len(), 1);
                    // target should be METHOD_RETURN (target_type = 0x14)
                    // Once TypeAnnotationTarget is implemented, assert the variant here.
                    return;
                }
            }
        }
    }
    panic!("RuntimeVisibleTypeAnnotations not found in getGreeting");
}

#[test]
#[ignore = "known bug: TypeAnnotation::from_reader is not yet implemented (todo!())"]
fn add_has_formal_parameter_type_annotations() {
    use mjvm::class_file::class_file::AttributeInfo;
    let cf = open("TypeAnnotated.class");
    let method = cf.methods.iter().find(|m| {
        let name_entry = &cf.constant_pool[m.name_index as usize - 1];
        if let mjvm::const_pool::constant_pool::ConstantPoolInfo::ConstantUtf8 { bytes, .. } = name_entry {
            std::str::from_utf8(bytes).unwrap_or("") == "add"
        } else {
            false
        }
    }).expect("add not found");

    for attr in &method.attributes {
        if let AttributeInfo::Code { attributes, .. } = attr {
            for code_attr in attributes {
                if let AttributeInfo::RuntimeVisibleTypeAnnotations { num_annotations, .. } = code_attr {
                    // METHOD_RETURN (1) + two METHOD_FORMAL_PARAMETER (2) = 3 annotations
                    assert_eq!(*num_annotations, 3);
                    return;
                }
            }
        }
    }
    panic!("RuntimeVisibleTypeAnnotations not found in add");
}
