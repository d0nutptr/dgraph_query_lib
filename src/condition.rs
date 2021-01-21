use crate::ToQueryString;
use serde_json::Value;
use crate::predicate::Variable;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub enum ConditionValue {
    String(String),
    StringArr(Vec<String>),
    Regexp(String),
    Literal(String),
    NumArr(Vec<String>),
    Val(Variable)
}

impl ConditionValue {
    pub fn string(value: &str) -> ConditionValue {
        ConditionValue::String(value.to_string())
    }

    pub fn regexp(value: &str) -> ConditionValue {
        ConditionValue::Regexp(value.to_string())
    }

    pub fn literal(value: &str) -> ConditionValue {
        ConditionValue::Literal(value.to_string())
    }

    pub fn literal_int(value: i64) -> ConditionValue {
        ConditionValue::Literal(format!("{}", value))
    }

    pub fn string_arr(value: Vec<&str>) -> ConditionValue {
        ConditionValue::StringArr(value.into_iter().map(|item| item.to_string()).collect())
    }

    pub fn num_arr(value: Vec<i64>) -> ConditionValue {
        ConditionValue::NumArr(value.into_iter().map(|value| format!("{}", value)).collect())
    }
}

impl ToQueryString for ConditionValue {
    fn to_query_string(&self) -> String {
        match self {
            ConditionValue::Regexp(value) | ConditionValue::Literal(value) => format!("{}", value),
            ConditionValue::String(value) => Value::String(value.clone()).to_string(),
            ConditionValue::Val(value) => format!("val({})", value.get_name()),
            ConditionValue::StringArr(value) => format!("[{}]", value.iter().map(|item| Value::String(item.to_string()).to_string()).join(", ")),
            ConditionValue::NumArr(value) => format!("[{}]", value.iter().join(", "))
        }
    }
}

#[derive(Clone, Debug)]
pub enum Condition {
    UID(String),
    EQ(String, ConditionValue),
    GE(String, ConditionValue),
    GT(String, ConditionValue),
    LE(String, ConditionValue),
    LT(String, ConditionValue),
    HAS(String),
    REGEXP(String, ConditionValue),
    AND(Box<Condition>, Box<Condition>),
    OR(Box<Condition>, Box<Condition>),
    NOT(Box<Condition>),
}

impl Condition {
    pub fn len(value: &str) -> String {
        format!("len({})", value)
    }

    pub fn has(value: &str) -> Condition {
        Condition::HAS(value.to_string())
    }

    pub fn uid(value: &str) -> Condition {
        Condition::UID(value.to_string())
    }
}

impl ToQueryString for Condition {
    fn to_query_string(&self) -> String {
        match self {
            Condition::UID(id) => format!("uid({id})", id = id),
            Condition::EQ(predicate, value) => format!("eq({predicate}, {value})", predicate = predicate, value = value.to_query_string()),
            Condition::GE(predicate, value) => format!("ge({predicate}, {value})", predicate = predicate, value = value.to_query_string()),
            Condition::GT(predicate, value) => format!("gt({predicate}, {value})", predicate = predicate, value = value.to_query_string()),
            Condition::LE(predicate, value) => format!("le({predicate}, {value})", predicate = predicate, value = value.to_query_string()),
            Condition::LT(predicate, value) => format!("lt({predicate}, {value})", predicate = predicate, value = value.to_query_string()),
            Condition::HAS(predicate) => format!("has({predicate})", predicate = predicate),
            Condition::REGEXP(predicate, ConditionValue::Regexp(regex_val)) => format!("regexp({predicate}, /{regex}/)", predicate = predicate, regex = regex_val),
            Condition::AND(left, right) => format!("({left} AND {right})", left = left.to_query_string(), right = right.to_query_string()),
            Condition::OR(left, right) => format!("({left} OR {right})", left = left.to_query_string(), right = right.to_query_string()),
            Condition::NOT(condition) => format!("(not {condition})", condition = condition.to_query_string()),
            _ => panic!("Invalid condition: {:?}", self)
        }
    }
}