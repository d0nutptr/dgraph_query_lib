use derive_builder::Builder;
use crate::{ToQueryString, IndentedString};
use itertools::Itertools;
use crate::condition::Condition;

#[derive(Clone)]
pub enum Predicate {
    Count(Field),
    Field(Field),
    Edge(Edge),
    Val(Variable),
    ScalarVariable(String, Field),
    EdgeVariable(String, Edge)
}

#[derive(Clone, Debug)]
pub struct Variable {
    name: String,
    alias: Option<String>
}

impl Variable {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            alias: None
        }
    }

    pub fn alias(mut self, alias: &str) -> Self {
        self.alias = Some(alias.to_string());
        self
    }

    pub fn random() -> Self {
        Self::new(&format!("var_{}", rand::random::<u128>()))
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_alias(&self) -> String {
        self.alias.clone()
            .map(|alias| format!("{} : ", alias))
            .unwrap_or("".to_string())
    }
}

#[derive(Clone)]
pub struct Field {
    name: String,
    alias: Option<String>
}

impl Field {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            alias: None
        }
    }

    pub fn alias(mut self, alias: &str) -> Self {
        self.alias = Some(alias.to_string());
        self
    }

    fn get_alias(&self) -> String {
        self.alias.clone()
            .map(|alias| format!("{} : ", alias))
            .unwrap_or("".to_string())
    }
}

#[derive(Builder, Clone)]
pub struct Edge {
    name: String,
    predicates: Vec<Predicate>,
    #[builder(setter(strip_option), default)]
    filter: Option<Condition>,
    #[builder(setter(strip_option), default)]
    alias: Option<String>
}

impl Edge {
    fn get_alias(&self) -> String {
        self.alias.clone()
            .map(|alias| format!("{} : ", alias))
            .unwrap_or("".to_string())
    }
}

impl ToQueryString for Edge {
    fn to_query_string(&self) -> String {
        let predicates = self.predicates.iter()
            .map(|predicate| predicate.to_query_string())
            .join("\n");

        let filter = self.filter.clone()
            .map(|filter| format!(" @filter({})", filter.to_query_string()))
            .unwrap_or("".to_string());

        format!("\
        {alias}{name}{filter} {{\n\
        {predicates}\n\
        }}", alias = self.get_alias(), name = self.name, filter = filter, predicates = predicates.indent())
    }
}

impl ToQueryString for Predicate {
    fn to_query_string(&self) -> String {
        match self {
            Predicate::Field(field) => {
                format!("{alias}{name}", alias = field.get_alias(), name = field.name)
            },
            Predicate::Val(variable) => {
                format!("{alias}val({name})", alias = variable.get_alias(), name = variable.name)
            },
            Predicate::EdgeVariable(name, edge) => {
                format!("{name} as {edge}", name = name, edge = edge.to_query_string())
            },
            Predicate::ScalarVariable(name, field) => {
                format!("{name} as {field}", name = name, field = field.name)
            },
            Predicate::Count(field) => {
                format!("{alias}count({name})", alias = field.get_alias(), name = field.name)
            },
            Predicate::Edge(edge) => {
                format!("{edge}", edge = edge.to_query_string())
            }
        }
    }
}