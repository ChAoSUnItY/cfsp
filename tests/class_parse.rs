use std::io::Cursor;

use insta::assert_yaml_snapshot;

use cfsp::parse::{to_class, ParsingOption};

#[test]
fn test_main() {
    let mut cursor = Cursor::new(include_bytes!(
        "../compiled_source/out/production/compiled_source/Main.class"
    ));

    assert_yaml_snapshot!(to_class(&mut cursor, ParsingOption::default()).unwrap());
}

#[test]
fn test_main_skip_attribute() {
    let mut cursor = Cursor::new(include_bytes!(
        "../compiled_source/out/production/compiled_source/Main.class"
    ));

    assert_yaml_snapshot!(
        to_class(&mut cursor, ParsingOption::default().skip_attribute()).unwrap()
    );
}

#[test]
fn test_main_skip_instruction() {
    let mut cursor = Cursor::new(include_bytes!(
        "../compiled_source/out/production/compiled_source/Main.class"
    ));

    assert_yaml_snapshot!(
        to_class(&mut cursor, ParsingOption::default().skip_instruction()).unwrap()
    );
}

#[test]
fn test_enum() {
    let mut cursor = Cursor::new(include_bytes!(
        "../compiled_source/out/production/compiled_source/Enum.class"
    ));

    assert_yaml_snapshot!(to_class(&mut cursor, ParsingOption::default()).unwrap());
}

#[test]
fn test_record() {
    let mut cursor = Cursor::new(include_bytes!(
        "../compiled_source/out/production/compiled_source/Record.class"
    ));

    assert_yaml_snapshot!(to_class(&mut cursor, ParsingOption::default()).unwrap());
}

#[test]
fn test_visible_annotation() {
    let mut cursor = Cursor::new(include_bytes!(
        "../compiled_source/out/production/compiled_source/VisibleAnnotation.class"
    ));

    assert_yaml_snapshot!(to_class(&mut cursor, ParsingOption::default()).unwrap());
}

#[test]
fn test_invisible_annotation() {
    let mut cursor = Cursor::new(include_bytes!(
        "../compiled_source/out/production/compiled_source/InvisibleAnnotation.class"
    ));

    assert_yaml_snapshot!(to_class(&mut cursor, ParsingOption::default()).unwrap());
}

#[test]
fn test_annotation_target() {
    let mut cursor = Cursor::new(include_bytes!(
        "../compiled_source/out/production/compiled_source/AnnotationTarget.class"
    ));

    assert_yaml_snapshot!(to_class(&mut cursor, ParsingOption::default()).unwrap());
}
