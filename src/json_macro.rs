
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String,Json>>)
}

macro_rules! json {
    (null) => {Json::Null};
    ([...]) => {Json::Array(...)};
    (false) => {Json::Boolean(false)};
    (true) => {Json::Boolean(true)};
    ($data:expr) => {
        {log_syntax!($data); 
        $data}
    };
}

#[test]
fn json_null() {
    assert_eq!(json!(null), Json::Null);
}

#[test]
fn json_bool() {
    assert_eq!(json!(true), Json::Boolean(true));
    assert_eq!(json!(false), Json::Boolean(false));
}

#[test]
fn json_array() {
    assert_eq!(json!([1,2,3]), Json::Array(vec![Json::Number(1),Json::Number(2),Json::Number(3)]));
}

pub fn main() {
    println!("json_macro");
    // trace_macros!(true);
    // println!("{:?}",json![
    //     {
    //         "name": "Garrett",
    //         "occupation": "Programmer"
    //     },
    //     {
    //         "name": "Olya",
    //         "occupation": "Artist"
    //     },
    // ]);

    // trace_macros!(false);
}

// const JSON_GLOSSERY_SAMPLE: String = String::from(
//     r#"{
//     "glossary": {
//         "title": "example glossary",
//         "GlossDiv": {
//             "title": "S",
//             "GlossList": {
//                 "GlossEntry": {
//                     "ID": "SGML",
//                     "SortAs": "SGML",
//                     "GlossTerm": "Standard Generalized Markup Language",
//                     "Acronym": "SGML",
//                     "Abbrev": "ISO 8879:1986",
//                     "GlossDef": {
//                         "para": "A meta-markup language, used to create markup languages such as DocBook.",
//                         "GlossSeeAlso": ["GML", "XML"]
//                     },
//                     "GlossSee": "markup"
//                 }
//             }
//         }
//     }
// }"#,
// );
