use std::fs::File;

use mjvm::class_file::{AttributeInfo, ClassFile, TypeAnnotation, TypeAnnotationInfo};
use mjvm::constant_pool::ConstantPoolInfo;

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

/// Extracts the RuntimeVisibleTypeAnnotations from a method's attributes.
fn method_type_annotations<'a>(cf: &'a ClassFile, method_name: &str) -> &'a Vec<TypeAnnotation> {
    let method = cf.methods.iter().find(|m| {
        cp_utf8(&cf.constant_pool, m.name_index) == method_name
    }).unwrap_or_else(|| panic!("{} not found", method_name));

    for attr in &method.attributes {
        if let AttributeInfo::RuntimeVisibleTypeAnnotations { annotations, .. } = attr {
            return annotations;
        }
    }
    panic!("RuntimeVisibleTypeAnnotations not found on {}", method_name);
}

// ── TypeAnnotated.class ───────────────────────────────────────────────────────
//
// public class TypeAnnotated {
//     public @TypeAnno String getGreeting() { ... }  // METHOD_RETURN type annotation
//     public @TypeAnno int add(@TypeAnno int a, @TypeAnno int b) { ... }
// }

#[test]
fn type_annotated_parses_successfully() {
    open("TypeAnnotated.class");
}

#[test]
fn type_annotated_has_three_methods() {
    let cf = open("TypeAnnotated.class");
    // <init>, getGreeting, add
    assert_eq!(cf.methods_count, 3);
}

#[test]
fn init_has_no_type_annotations() {
    let cf = open("TypeAnnotated.class");
    let method = cf.methods.iter().find(|m| {
        cp_utf8(&cf.constant_pool, m.name_index) == "<init>"
    }).expect("<init> not found");
    let has = method.attributes.iter().any(|a|
        matches!(a, AttributeInfo::RuntimeVisibleTypeAnnotations { .. })
    );
    assert!(!has, "<init> should have no type annotations");
}

// ── getGreeting: single METHOD_RETURN annotation ────────────────────────────

#[test]
fn get_greeting_has_runtime_visible_type_annotations() {
    let cf = open("TypeAnnotated.class");
    let annotations = method_type_annotations(&cf, "getGreeting");
    assert_eq!(annotations.len(), 1);
}

#[test]
fn get_greeting_annotation_target_is_method_return() {
    let cf = open("TypeAnnotated.class");
    let ann = &method_type_annotations(&cf, "getGreeting")[0];
    assert_eq!(ann.type_annotation_info, TypeAnnotationInfo::EmptyTarget { target_type: 0x14 });
}

#[test]
fn get_greeting_annotation_type_is_type_anno() {
    let cf = open("TypeAnnotated.class");
    let ann = &method_type_annotations(&cf, "getGreeting")[0];
    // type_index → CP #16 = Utf8 "LTypeAnno;"
    assert_eq!(cp_utf8(&cf.constant_pool, ann.type_index), "LTypeAnno;");
}

#[test]
fn get_greeting_annotation_has_empty_type_path() {
    let cf = open("TypeAnnotated.class");
    let ann = &method_type_annotations(&cf, "getGreeting")[0];
    assert_eq!(ann.type_path.path_length, 0);
    assert!(ann.type_path.paths.is_empty());
}

#[test]
fn get_greeting_annotation_has_no_element_value_pairs() {
    let cf = open("TypeAnnotated.class");
    let ann = &method_type_annotations(&cf, "getGreeting")[0];
    assert_eq!(ann.num_element_value_pairs, 0);
    assert!(ann.element_value_pairs.is_empty());
}

// ── add: METHOD_RETURN + two METHOD_FORMAL_PARAMETER annotations ────────────

#[test]
fn add_has_three_type_annotations() {
    let cf = open("TypeAnnotated.class");
    let annotations = method_type_annotations(&cf, "add");
    assert_eq!(annotations.len(), 3);
}

#[test]
fn add_first_annotation_is_method_return() {
    let cf = open("TypeAnnotated.class");
    let ann = &method_type_annotations(&cf, "add")[0];
    assert_eq!(ann.type_annotation_info, TypeAnnotationInfo::EmptyTarget { target_type: 0x14 });
    assert_eq!(cp_utf8(&cf.constant_pool, ann.type_index), "LTypeAnno;");
    assert_eq!(ann.num_element_value_pairs, 0);
}

#[test]
fn add_second_annotation_is_formal_parameter_0() {
    let cf = open("TypeAnnotated.class");
    let ann = &method_type_annotations(&cf, "add")[1];
    assert_eq!(
        ann.type_annotation_info,
        TypeAnnotationInfo::FormalParameterTarget { target_type: 0x16, formal_parameter_index: 0 }
    );
    assert_eq!(cp_utf8(&cf.constant_pool, ann.type_index), "LTypeAnno;");
    assert_eq!(ann.num_element_value_pairs, 0);
}

#[test]
fn add_third_annotation_is_formal_parameter_1() {
    let cf = open("TypeAnnotated.class");
    let ann = &method_type_annotations(&cf, "add")[2];
    assert_eq!(
        ann.type_annotation_info,
        TypeAnnotationInfo::FormalParameterTarget { target_type: 0x16, formal_parameter_index: 1 }
    );
    assert_eq!(cp_utf8(&cf.constant_pool, ann.type_index), "LTypeAnno;");
    assert_eq!(ann.num_element_value_pairs, 0);
}

#[test]
fn add_all_annotations_have_empty_type_path() {
    let cf = open("TypeAnnotated.class");
    for ann in method_type_annotations(&cf, "add") {
        assert_eq!(ann.type_path.path_length, 0);
        assert!(ann.type_path.paths.is_empty());
    }
}
