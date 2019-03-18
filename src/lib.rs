extern crate failure;
extern crate serde_json;

use failure::Fail;
use std::collections::{BTreeMap, HashMap};

type MyResult<T> = Result<T, failure::Error>;
type JsonValue = serde_json::Value;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Value error")]
    ValueError,
}

pub fn sort_list(value: JsonValue) -> MyResult<JsonValue> {
    let list: Vec<JsonValue> = value
        .as_array()
        .map(|x| x.to_owned())
        .ok_or(ErrorKind::ValueError)?;
    let mut new_list: Vec<JsonValue> = vec![];
    for json_value in list.into_iter() {
        new_list.push(sort_json(json_value)?)
    }
    serde_json::to_value(new_list).map_err(|err| err.context("Not able to Serialize").into())
}

pub fn sort_map(value: JsonValue) -> MyResult<JsonValue> {
    let map = value
        .as_object()
        .map(|x| x.to_owned())
        .ok_or(ErrorKind::ValueError)?;
    let mut new_map = HashMap::new();
    for (key, value) in map.into_iter() {
        new_map.insert(key, sort_json(value)?);
    }
    let btree_map: BTreeMap<_, _> = new_map.iter().collect();
    serde_json::to_value(btree_map).map_err(|err| err.context("Not able to Serialize").into())
}

pub fn sort_json(value: JsonValue) -> MyResult<JsonValue> {
    if value.is_object() {
        return sort_map(value);
    }
    if value.is_array() {
        return sort_list(value);
    }
    Ok(value)
}