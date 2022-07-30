use crate::{filter::*, sort::*};
use serde_json::{json, Value as JsonValue};

#[test]
fn sort_test() {
    let j = json!({"d": 1, "e": 2, "f": 3, "a": 5, "b": 4, "c": 5});
    let k = sort_json(j)
        .unwrap()
        .as_object()
        .map(|x| x.to_owned())
        .unwrap();
    let sorted_keys = vec!["a", "b", "c", "d", "e", "f"];
    let mut after_sort = vec![];
    for k1 in k.keys() {
        after_sort.push(k1);
    }
    assert_eq!(after_sort, sorted_keys)
}

#[test]
fn filter_test() {
    let input = json!([1,null,3,null,{"a": [1,2,3,4, "Abrar" ]}]);
    let expected = json!([1,3,{"a": [1,2,3,4]}]);
    let result = filter(input, &|value| match value {
        JsonValue::Number(number) => {
            if number.is_i64() && number.as_i64().unwrap() < 5 {
                true
            } else {
                false
            }
        }
        JsonValue::String(string) => {
            if string == "Abrar" {
                false
            } else {
                true
            }
        }
        JsonValue::Null => false,
        _ => true,
    });

    assert_eq!(result, expected);
}

#[test]
fn filter_map_keys() {
    let data = json!({"d": 1, "e": 2, "f": 3, "a": 5, "b": 4, "c": 5});
    let expected = json!({"e": 2, "f": 3, "a": 5, "b": 4, "c": 5});
    let result = filter_map_with_keys(data, &|key| {
        if key.contains("d") {
            false
        } else {
            true
        }
    });
    assert_eq!(result, expected)
}
