# sorted_json_keys

Rust lib to sort JSON based on string keys and filter json values

Note: 

- filter is working based on the keys
- sorted is buggy, need improvements


```rust

#[cfg(test)]
mod tests {
    use crate::filter::keys::filter;
    
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

```