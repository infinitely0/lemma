use std::fmt::Write;

use serde_json::Value;

pub fn pretty_print_ast(json: Value) -> String {
    let mut output = String::new();
    format_tree(&json, &mut output, "");
    output
}

fn format_tree(value: &Value, output: &mut String, prefix: &str) {
    let node_empty = |value: &Value| match value {
        Value::Object(map) => map.is_empty() || map.keys().all(|k| k == "line"),
        Value::Array(arr) => arr.is_empty(),
        _ => false,
    };
    match value {
        Value::Object(map) => {
            let filtered_map: Vec<_> = map.iter().filter(|(k, _)| *k != "line").collect();
            let len = filtered_map.len();
            for (i, (k, v)) in filtered_map.iter().enumerate() {
                if i < len - 1 {
                    writeln!(output, "{}├──{}", prefix, k).unwrap();
                    format_tree(v, output, &(prefix.to_string() + "│   "));
                } else {
                    writeln!(output, "{}└──{}", prefix, k).unwrap();
                    format_tree(v, output, &(prefix.to_string() + "    "));
                }
            }
        }
        Value::Array(arr) => {
            let len = arr.len();
            for (i, v) in arr.iter().enumerate() {
                if !node_empty(v) {
                    if i < len - 1 {
                        writeln!(output, "{}├──[{}]", prefix, i).unwrap();
                        format_tree(v, output, &(prefix.to_string() + "│   "));
                    } else {
                        writeln!(output, "{}└──[{}]", prefix, i).unwrap();
                        format_tree(v, output, &(prefix.to_string() + "    "));
                    }
                }
            }
        }
        _ => writeln!(output, "{}└──{}", prefix, value).unwrap(),
    }
}
