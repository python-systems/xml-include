use std::fs::read_to_string;
use std::path::PathBuf;
use xml_include::resolve_xml_includes;

fn is_same_except_whitespace(a: &str, b: &str) -> bool {
    let a = a.replace([' ', '\n', '\t'], "");
    let b = b.replace([' ', '\n', '\t'], "");
    a == b
}

#[test]
fn test_trading_api() {
    let input_file = PathBuf::from("tests/examples/TradingApi.xml");
    let reference_file = PathBuf::from("tests/examples/TradingApi.ref.xml");

    let resolved_content = resolve_xml_includes(&input_file).unwrap();

    assert!(is_same_except_whitespace(
        &resolved_content,
        read_to_string(reference_file).unwrap().as_str()
    ));
}
