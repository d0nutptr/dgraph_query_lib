use crate::query::Query;
use crate::ToQueryString;
use crate::mutation::Mutation;
use crate::condition::Condition;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Upsert {
    pub query: String,
    pub mutations: Vec<UpsertBlock>
}

impl Upsert {
    pub fn new(query: Query) -> Self {
        Self {
            query: query.to_query_string(),
            mutations: vec![]
        }
    }

    pub fn upsert_block(mut self, block: UpsertBlock) -> Self {
        self.mutations.push(block);
        self
    }

    pub fn upsert_block_ref(&mut self, block: UpsertBlock) {
        self.mutations.push(block);
    }
}


#[derive(Serialize, Clone)]
pub struct UpsertBlock {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cond: Option<String>,
    #[serde(flatten)]
    pub mutation: Mutation
}

impl UpsertBlock {
    pub fn new(mutation: Mutation) -> Self {
        Self {
            cond: None,
            mutation
        }
    }

    pub fn cond(mut self, condition: Condition) -> Self {
        self.cond = Some(format!("@if({})", condition.to_query_string()));
        self
    }
}

/*

{
  "query": "{q1(func: regexp(ip_value, /^185.+$/)) {u1 as uid} }",
  "mutations": [
    {
      "cond": "@if(eq(len(u1), 0))",
      "set": [
        {
          "uid": "uid(u1)",
          "name": "FAKE"
        }
      ]
    }
  ]
}


Example ^

 */