use insta::assert_yaml_snapshot;

use cfsp::parse::{class_signature, field_signature, method_signature, ParseResult};

#[test]
fn test_class_signature_with_generic() -> ParseResult<()> {
    let class_signature =
        class_signature("<T:Ljava/lang/Object;>Ljava/lang/Object;Ljava/lang/Runnable;")?;

    assert_yaml_snapshot!(class_signature);

    Ok(())
}

#[test]
fn test_field_signature_object() -> ParseResult<()> {
    let field_signature = field_signature("Ljava/lang/Object;")?;

    assert_yaml_snapshot!(field_signature);

    Ok(())
}

#[test]
fn test_field_signature_type_variable() -> ParseResult<()> {
    let field_signature = field_signature("TT;")?;

    assert_yaml_snapshot!(field_signature);

    Ok(())
}

#[test]
fn test_method_signature_with_generic() -> ParseResult<()> {
    let method_signature = method_signature(
        "<T:Ljava/lang/Object;>(Z[[ZTT;)Ljava/lang/Object;^Ljava/lang/Exception;",
    )?;

    assert_yaml_snapshot!(method_signature);

    Ok(())
}

#[test]
fn test_class_signature() {
    assert!(class_signature(
        "<T::Ljava/lang/Runnable;R:Ljava/lang/Object;>Ljava/lang/Object;Ljava/lang/Runnable;"
    )
    .is_ok());
    assert!(class_signature(
        "<T::Ljava/lang/Runnable;R:Ljava/lang/Object;>Ljava/lang/Object;Ljava/lang/Runnable;I"
    )
    .is_err());
    assert!(field_signature("Lclass.class").is_err());
    assert!(field_signature("R").is_err());
    assert!(field_signature("").is_err());
    assert!(field_signature("Lclass/").is_err());
    assert!(field_signature("Lclass<*>;").is_ok());
    assert!(field_signature("Lclass<+Ljava/lang/Runnable;>;").is_ok());
    assert!(field_signature("Lclass<+>;").is_err());
    assert!(field_signature("Lclass<*>.inner<*>;").is_ok());
    assert!(method_signature("(Z[[ZTT;)V^",).is_err());
    assert!(method_signature("<>(Z[[ZTT;)V^Ljava/lang/Exception;").is_ok());
    assert!(method_signature("<>(Z[[ZTT;)V^TT;",).is_ok());
    assert!(method_signature("<>(Z[[ZTT;)",).is_err());
    assert!(method_signature("<>(Z[[ZTT;)V^",).is_err());
    assert!(method_signature("<>(Z[[ZTT;)V^",).is_err());
}
