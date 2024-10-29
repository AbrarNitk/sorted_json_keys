use serde_json::{Map, Value as JsonValue, Value};

fn join(s: &str, j: &str) -> String {
    if s.is_empty() {
        return String::from(j);
    }

    if j.is_empty() {
        return String::from(s);
    }

    format!("{}.{}", s, j)
}

pub fn filter_array<F>(value: Vec<JsonValue>, parent_key: &str, f: &F) -> JsonValue
where
    F: Fn(&str) -> bool,
{
    let mut array = vec![];
    for value in value.into_iter() {
        match value {
            Value::Array(arr) => {
                let filtered = filter_array(arr, join(parent_key, "[]").as_str(), f);
                if !filtered.is_null() {
                    array.push(filtered);
                }
            }
            Value::Object(map) => {
                let filtered = filter_map(map, join(parent_key, "[]").as_str(), f);
                if !filtered.is_null() {
                    array.push(filtered);
                }
            }

            // can not filter array literal values based on the keys
            Value::Null => {}
            Value::Bool(_) => {}
            Value::Number(_) => {}
            Value::String(_) => {}
        }
    }

    if array.is_empty() {
        JsonValue::Null
    } else {
        JsonValue::Array(array)
    }
}

pub fn filter_map<F>(value: Map<String, JsonValue>, parent_key: &str, f: &F) -> JsonValue
where
    F: Fn(&str) -> bool,
{
    let mut map = Map::new();
    for (key, value) in value.into_iter() {
        if f(join(parent_key, &key).as_str()) {
            map.insert(key, value);
        } else {
            let value = filter_util(value, join(parent_key, &key).as_str(), f);
            if !value.is_null() {
                map.insert(key, value);
            }
        }
    }

    if map.is_empty() {
        JsonValue::Null
    } else {
        JsonValue::Object(map)
    }
}

pub fn filter_util<F>(value: JsonValue, parent_key: &str, f: &F) -> JsonValue
where
    F: Fn(&str) -> bool,
{
    match value {
        Value::Array(arr) => filter_array(arr, parent_key, f),
        Value::Object(map) => filter_map(map, parent_key, f),

        // can not filter array literal values based on the keys
        Value::Null => JsonValue::Null,
        Value::Bool(_) => JsonValue::Null,
        Value::Number(_) => JsonValue::Null,
        Value::String(_) => JsonValue::Null,
    }
}

pub fn filter<F>(value: JsonValue, f: &F) -> JsonValue
where
    F: Fn(&str) -> bool,
{
    filter_util(value, "", f)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map_filter() {
        let data: serde_json::Value = serde_json::json!({
            "foo": "bar",
            "config": {
                "foo-config": "bar",
                "temp": "temppp",
                "hector": "tractor"
            }
        });

        let filtered = filter(data, &|key| {
            vec!["foo", "config.temp", "config.hector"].contains(&key)
        });

        let expected = serde_json::json!({
          "config": {
            "hector": "tractor",
            "temp": "temppp"
          },
          "foo": "bar"
        });

        assert_eq!(expected, filtered);
        // println!("value: {}", serde_json::to_string_pretty(&value).unwrap());
    }

    #[test]
    fn test_list_filter() {
        let data: serde_json::Value = serde_json::json!({
            "foo": "bar",
            "config": [
                {
                    "foo-config": "bar",
                    "temp": "temppp",
                    "hector": "tractor"
                },
                {
                    "foo-config": "bar1",
                    "temp": "temppp1",
                    "hector": "tractor1"
                }
            ]
        });

        let expected = serde_json::json!({
            "foo": "bar",
            "config": [
            {
              "hector": "tractor",
              "temp": "temppp"
            },
            {
              "hector": "tractor1",
              "temp": "temppp1"
            }
          ]
        });

        let filtered = filter(data, &|key| {
            vec!["foo", "config.[].temp", "config.[].hector"].contains(&key)
        });
        assert_eq!(expected, filtered);
    }

    #[test]
    fn filter_inner_list() {
        let data: serde_json::Value = serde_json::json!({
            "foo": "bar",
            "config": [
                {
                    "foo-config": "bar",
                    "temp": [{ "a": 1 }],
                    "hector": "tractor"
                },
                {
                    "foo-config": "bar1",
                    "temp": [{ "a": 2 }],
                    "hector": "tractor1"
                }
            ]
        });

        let filtered = filter(data, &|key| key == "config.[].temp.[].a");

        let expected: serde_json::Value = serde_json::json!({
            "config": [
                {
                    "temp": [{ "a": 1 }],
                },
                {
                    "temp": [{ "a": 2 }],
                }
            ]
        });

        assert_eq!(expected, filtered);

        // println!("value: {}", serde_json::to_string_pretty(&filtered).unwrap());
    }
}
