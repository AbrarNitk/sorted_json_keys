use crate::filter::utils::key_join;
use serde_json::{Map, Value};

fn get_array(arr: &Vec<Value>, parent_key: &str, search_key: &str) -> Vec<Value> {
    let mut collected_values = vec![];

    for value in arr {
        let object_key = key_join(parent_key, "[]");
        match value {
            Value::Object(nested_map) => {
                let values_from_nested = get_map(nested_map, &object_key, search_key);
                collected_values.extend(values_from_nested);
            }
            Value::Array(nested_array) => {
                let values_from_nested = get_array(nested_array, &object_key, search_key);
                collected_values.extend(values_from_nested);
            }
            _ => {}
        }
    }
    collected_values
}

fn get_map(map: &Map<String, Value>, parent_key: &str, search_key: &str) -> Vec<Value> {
    let mut collected_values = vec![];
    for (k, v) in map {
        let object_key = key_join(parent_key, k);
        if object_key.eq(search_key) {
            if !v.is_null() {
                collected_values.push(v.clone());
            }
        } else {
            match v {
                Value::Array(nested_array) => {
                    let values_from_nested = get_array(nested_array, &object_key, search_key);
                    collected_values.extend(values_from_nested);
                }
                Value::Object(nested_map) => {
                    let values_from_nested = get_map(nested_map, &object_key, search_key);
                    collected_values.extend(values_from_nested);
                }
                _ => {}
            }
        }
    }

    collected_values
}

fn get_util(value: &Value, parent_key: &str, key: &str) -> Vec<Value> {
    match value {
        Value::Array(arr) => get_array(arr, parent_key, key),
        Value::Object(map) => get_map(map, parent_key, key),
        _ => vec![],
    }
}

pub fn get(value: &Value, key: &str) -> Vec<Value> {
    get_util(value, "", key)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_js(value: &str) -> Value {
        Value::String(value.to_string())
    }

    #[test]
    fn empty_test() {
        let json = serde_json::json!({"foo": "b"});
        let expected: Vec<Value> = vec![];
        assert_eq!(get(&json, "bar"), expected);
    }

    #[test]
    fn simple_test() {
        let json = serde_json::json!({"foo": "b"});
        let expected = vec![to_js("b")];
        assert_eq!(get(&json, "foo"), expected);
    }

    #[test]
    fn nested_object_test() {
        let json = serde_json::json!({"foo": { "bar": { "baz": "qux" } } });
        let expected = vec![to_js("qux")];
        assert_eq!(get(&json, "foo.bar.baz"), expected);
    }

    #[test]
    fn nested_array_test() {
        let json = serde_json::json!({
            "foo": {
                    "bar": [
                        {
                            "baz": "qux1"
                        },
                        {
                            "baz": "qux2"
                        },
                        {
                            "baz": "qux3"
                        },
                        {
                            "temp": "qux"
                        }
                    ]
            }
        });
        let expected = vec![to_js("qux1"), to_js("qux2"), to_js("qux3")];
        assert_eq!(get(&json, "foo.bar.[].baz"), expected);
    }

    #[test]
    fn nested_nested_array_test() {
        let json = serde_json::json!({
            "foo": {
                    "bar": [
                    [
                        {
                            "baz": "qux1"
                        },
                        {
                            "baz": "qux2"
                        },
                        {
                            "baz": "qux3"
                        },
                        {
                            "temp": "qux"
                        }
                    ],
                    [
                        {
                            "baz": "qux11"
                        },
                        {
                            "baz": "qux12"
                        },
                        {
                            "baz": "qux13"
                        },
                        {
                            "temp": "qux"
                        }
                    ]
                ]
            }
        });
        let expected = vec![
            to_js("qux1"),
            to_js("qux2"),
            to_js("qux3"),
            to_js("qux11"),
            to_js("qux12"),
            to_js("qux13"),
        ];
        assert_eq!(get(&json, "foo.bar.[].[].baz"), expected);
    }
}
