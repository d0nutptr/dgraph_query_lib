use derive_builder::Builder;
use crate::ToQueryString;
use itertools::Itertools;
use crate::condition::{Condition, ConditionValue};
use rand::random;
use std::collections::HashMap;
use serde::Serialize;

#[derive(Builder, Serialize, Clone)]
pub struct Mutation {
    #[builder(default)]
    pub set: Vec<MutationUnit>,
    #[builder(default)]
    pub delete: Vec<MutationUnit>
}

#[derive(Clone, Serialize)]
pub struct MutationUnit {
    #[serde(flatten)]
    uid: MutationUID,
    #[serde(flatten)]
    predicates: HashMap<String, MutationPredicateValue>,
}

impl MutationUnit {
    pub fn new(uid: MutationUID) -> Self {
        Self {
            uid,
            predicates: Default::default()
        }
    }

    pub fn predicate(mut self, name: &str, value: MutationPredicateValue) -> Self {
        self.predicates.insert(name.to_string(), value);
        self
    }

    pub fn predicate_ref(&mut self, name: &str, value: MutationPredicateValue) {
        self.predicates.insert(name.to_string(), value);
    }
}

#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum MutationPredicateValue {
    // todo: technically should support arrays
    String(String),
    Number(i64),
    Float(f64),
    Bool(bool),
    Edge(MutationUID),
    Edges(Vec<MutationUID>),
    Null
}

impl MutationPredicateValue {
    pub fn string(value: &str) -> MutationPredicateValue {
        MutationPredicateValue::String(value.to_string())
    }
}

#[derive(Clone, Serialize)]
pub struct MutationUID {
    uid: String
}

impl MutationUID {
    pub fn placeholder() -> MutationUID {
        Self {
            uid: MutationUID::generate_placeholder()
        }
    }

    pub fn variable(name: &str) -> MutationUID {
        Self {
            uid: MutationUID::generate_variable(name)
        }
    }

    pub fn uid(value: &str) -> MutationUID {
        Self {
            uid: value.to_string()
        }
    }

    fn generate_placeholder() -> String {
        format!("_:uid_placeholder_{}", random::<u128>())
    }

    fn generate_variable(name: &str) -> String {
        format!("uid({})", name)
    }
}

#[derive(Clone)]
pub enum MutationType {
    SET
}

#[derive(Clone)]
pub enum MutationLiteral {
    Literal(String),
    String(String)
}

impl ToQueryString for Mutation {
    fn to_query_string(&self) -> String {
        unimplemented!()
    }
}
