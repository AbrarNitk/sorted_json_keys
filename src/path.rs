use serde_json::Value;

pub fn get(path: &str, value: &Value) -> Option<Value> {
    match value {
        Value::Array(vec) => get_vec(path, vec),
        Value::Object(map) => get_map(path, map),
        Value::Null => None,
        _ => {
            if path.is_empty() {
                // Get the whole value.
                // We know it's not null - checked above
                Some(value.to_owned())
            } else {
                // Path continues, but we can't traverse into a scalar
                None
            }
        }
    }
}

fn get_vec(_path: &str, _value: &Vec<Value>) -> Option<Value> {
    // todo: need to fix it later for array indexed values, for now it is not needed
    None
}

fn get_map(path: &str, value: &serde_json::Map<String, Value>) -> Option<Value> {
    let (leading, trailing) = path_split(path);

    if let Some(remaining) = trailing {
        match value.get(&leading) {
            None => None,
            Some(v) => get(&remaining, v),
        }
    } else {
        match value.get(&leading) {
            Some(v) => Some(v.to_owned()),
            None => None,
        }
    }
}

/// Split the path string by dot, if present.
///
/// Returns a tuple of (before_dot, after_dot), removing escapes
fn path_split(path: &str) -> (String, Option<&str>) {
    let mut buf = String::new();
    let mut escaped = false;
    for (n, c) in path.char_indices() {
        match c {
            _ if escaped => {
                buf.push(c);
                escaped = false;
            }
            '\\' => {
                escaped = true;
            }
            '.' => {
                return (buf, Some(&path[n + 1..]));
            }
            _ => {
                buf.push(c);
            }
        }
    }

    // trailing slash is discarded
    (buf, None)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_simple() {
        let json_value = serde_json::json!({
            "foo": "foo-v",
            "foo-1": "foo-1-v",
            "foo-2": "foo-2-v",
            "foo-3": "foo-3-v"
        });

        let output = super::get("foo", &json_value);
        assert_eq!(output, Some(serde_json::Value::String("foo-v".to_string())));
        let output = super::get("foo-temp", &json_value);
        assert_eq!(output, None)
    }

    #[test]
    fn test_nested_primitive() {
        let json_value = serde_json::json!({
            "foo": "foo-v",
            "foo-1": "foo-1-v",
            "foo-2": "foo-2-v",
            "foo-3": "foo-3-v",
            "nested": {
                "foo": "foo-v",
                "nested": {
                    "foo": "foo-v-nested",
                }
            }
        });

        let output = super::get("nested.nested.foo", &json_value);
        assert_eq!(
            output,
            Some(serde_json::Value::String("foo-v-nested".to_string()))
        );
    }

    #[test]
    fn test_nested_json() {
        let json_value = serde_json::json!({
            "foo": "foo-v",
            "foo-1": "foo-1-v",
            "foo-2": "foo-2-v",
            "foo-3": "foo-3-v",
            "nested": {
                "foo": "foo-v",
                "nested": {
                    "foo": "foo-v-nested",
                }
            }
        });

        let output = super::get("nested.nested", &json_value);
        assert_eq!(
            output,
            Some(serde_json::json!({
                "foo": "foo-v-nested",
            }))
        );
    }
}
