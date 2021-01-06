use derive_builder::Builder;
use crate::{ToQueryString, IndentedString};
use crate::queryblock::QueryBlock;
use itertools::Itertools;

#[derive(Builder, Clone)]
pub struct Query {
    query_blocks: Vec<QueryBlock>
}

impl ToQueryString for Query {
    fn to_query_string(&self) -> String {
        let query_blocks = self.query_blocks.iter()
            .map(|query_block| query_block.to_query_string().indent())
            .join("\n");

        format!("{{\n{query_blocks}\n}}", query_blocks = query_blocks)
    }
}
