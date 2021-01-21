use derive_builder::Builder;
use crate::predicate::Predicate;
use crate::condition::Condition;
use crate::{ToQueryString, IndentedString};
use rand::random;
use itertools::Itertools;

#[derive(Builder, Clone)]
pub struct QueryBlock {
    query_type: QueryBlockType,
    predicates: Vec<Predicate>,
    root_filter: Condition,
    #[builder(setter(strip_option), default)]
    filter: Option<Condition>,
    #[builder(setter(strip_option), default)]
    first: Option<i64>,
    #[builder(setter(strip_option), default)]
    variable: Option<String>,
    #[builder(default)]
    order: QueryOrder,
    #[builder(default)]
    cascade: bool,
}

#[derive(Clone)]
pub enum QueryOrder {
    None,
    ASC(Predicate),
    DESC(Predicate)
}

impl Default for QueryOrder {
    fn default() -> Self {
        QueryOrder::None
    }
}

impl ToQueryString for QueryOrder {
    fn to_query_string(&self) -> String {
        match self {
            QueryOrder::None => format!(""),
            QueryOrder::ASC(pred) => format!(", orderasc: {}", pred.to_query_string()),
            QueryOrder::DESC(pred) => format!(", orderdesc: {}", pred.to_query_string())
        }
    }
}

#[derive(Clone)]
pub enum QueryBlockType {
    Query(String),
    Var
}

impl QueryBlockType {
    pub fn query() -> Self {
        QueryBlockType::Query(Self::generate_name())
    }

    pub fn var() -> Self {
        QueryBlockType::Var
    }

    fn generate_name() -> String {
        format!("query_{}", random::<u128>())
    }
}

impl ToQueryString for QueryBlockType {
    fn to_query_string(&self) -> String {
        match self {
            QueryBlockType::Query(name) => name.clone(),
            QueryBlockType::Var => "var".to_string()
        }
    }
}

impl ToQueryString for QueryBlock {
    fn to_query_string(&self) -> String {
        let query_block_inner = self.predicates.iter()
            .map(|predicate| predicate.to_query_string())
            .join("\n");

        let first = self.first
            .as_ref()
            .map(|first| format!(", first: {}", first))
            .unwrap_or(format!(""));

        let filter = self.filter.clone()
            .map(|filter| format!("@filter({})", filter.to_query_string()))
            .unwrap_or("".to_string());

        let cascade = if self.cascade { format!("@cascade") } else { format!("") };

        let variable = self.variable.clone()
            .map(|variable| format!("{} as ", variable))
            .unwrap_or("".to_string());

        format!("\
        {variable}{name}(func: {root_filter}{order}{first}) {filter} {cascade} {{\
        \n{query_block_inner}\
        \n}}",
                variable = variable,
                name = self.query_type.to_query_string(),
                root_filter = self.root_filter.to_query_string(),
                order = self.order.to_query_string(),
                first = first,
                filter = filter,
                cascade = cascade,
                query_block_inner = query_block_inner.indent()
        )
    }
}
