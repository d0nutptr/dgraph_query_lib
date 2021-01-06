mod tests;
pub mod query;
pub mod queryblock;
pub mod condition;
pub mod predicate;
pub mod upsert;
pub mod mutation;
pub mod schema;

pub use query::QueryBuilder;
pub use queryblock::QueryBlockBuilder;
pub use predicate::EdgeBuilder;
pub use mutation::MutationBuilder;

trait ToQueryString {
    fn to_query_string(&self) -> String;
}

trait IndentedString {
    fn indent(&self) -> String;
}

impl IndentedString for String {
    fn indent(&self) -> String {
        format!("\t{}", self.replace("\n", "\n\t"))
    }
}

impl<I: ToQueryString + Clone> ToQueryString for Option<I> {
    fn to_query_string(&self) -> String {
        self.clone()
            .map(|item| item.to_query_string())
            .unwrap_or("".to_string())
    }
}