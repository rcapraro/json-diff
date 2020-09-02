use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum JsonNode {
    Nil,
    Value(Value, Value),
    Node(HashMap<String, JsonNode>)
}

#[derive(Debug, PartialEq)]
pub struct Mismatch {
    pub left_only_keys: JsonNode,
    pub right_only_keys: JsonNode,
    pub keys_in_both: JsonNode,
}

impl Mismatch {
    pub fn new(l: JsonNode, r: JsonNode, u: JsonNode) -> Mismatch {
        Mismatch {
            left_only_keys: l,
            right_only_keys: r,
            keys_in_both: u,
        }
    }
}

pub fn match_json(value1: &Value, value2: &Value) -> Mismatch {
    //TODO
    Mismatch::new(JsonNode::Nil, JsonNode::Nil, JsonNode::Nil)
}