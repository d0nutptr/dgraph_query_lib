use crate::queryblock::QueryBlockType;
use crate::condition::{Condition, ConditionValue};
use crate::predicate::{Predicate, Field, Variable, Edge};
use crate::{QueryBuilder, QueryBlockBuilder, EdgeBuilder};
use crate::ToQueryString;
use crate::MutationBuilder;
use serde_json;
use crate::mutation::{MutationUnit, MutationUID, MutationPredicateValue, Mutation};
use crate::upsert::{Upsert, UpsertBlock};
use crate::schema::{Schema, SchemaDefinition, PredicateDefinition, PredicateType, Indexing};

#[test]
fn create_query() {
    let query = QueryBuilder::default()
        .query_blocks(vec![
            QueryBlockBuilder::default()
                .query_type(QueryBlockType::var())
                .root_filter(Condition::HAS("last_seen".to_string()))
                .predicates(vec![
                    Predicate::Field(Field::new("uid")),
                    Predicate::ScalarVariable("TIME".to_string(), Field::new("last_seen")),
                    Predicate::EdgeVariable("IMPORTANT_EDGES".to_string(), EdgeBuilder::default()
                        .name("connected_elements".to_string())
                        .filter(Condition::REGEXP("a_predicate".to_string(), ConditionValue::Regexp("[aA].+".to_string())))
                        .alias("some_alias".to_string())
                        .predicates(vec![
                            Predicate::Field(Field::new("uid")),
                            Predicate::Field(Field::new("dgraph.type").alias("dgraph_type"))
                        ]).build().unwrap())
                ])
                .build().unwrap(),
            QueryBlockBuilder::default()
                .query_type(QueryBlockType::query())
                .filter(Condition::GT("last_seen".to_string(), ConditionValue::Literal("1234".to_string())))
                .root_filter(Condition::EQ("node_key".to_string(), ConditionValue::String("fake_node_key".to_string())))
                .predicates(vec![
                    Predicate::Field(Field::new("uid")),
                    Predicate::Field(Field::new("node_key")),
                    Predicate::Field(Field::new("first_seen")),
                    Predicate::Field(Field::new("last_seen")),
                    Predicate::Edge(EdgeBuilder::default()
                        .name("org".to_string())
                        .predicates(vec![
                            Predicate::Field(Field::new("uid")),
                            Predicate::Field(Field::new("org_name")),
                            Predicate::Edge(EdgeBuilder::default()
                                .name("nested_edge".to_string())
                                .filter(Condition::EQ("inner_predicate".to_string(), ConditionValue::String("A string".to_string())))
                                .predicates(vec![
                                    Predicate::Field(Field::new("uid")),
                                    Predicate::Field(Field::new("dgraph.type").alias("dgraph_type")),
                                    Predicate::Count(Field::new("widgets"))
                                ]).build().unwrap()
                            )
                        ]).build().unwrap())
                ])
                .build().unwrap(),
            QueryBlockBuilder::default()
                .query_type(QueryBlockType::query())
                .root_filter(Condition::EQ("last_seen".to_string(), ConditionValue::Val(Variable::new("TIME"))))
                .predicates(vec![
                    Predicate::Field(Field::new("uid")),
                    Predicate::Field(Field::new("last_seen")),
                    Predicate::Field(Field::new("node_key")),
                    Predicate::Val(Variable::new("TIME").alias("foo"))
                ])
                .build().unwrap()
        ])
        .build().unwrap();

    println!("{}", query.to_query_string());
}

#[test]
fn create_mutation() {
    let new_node_1 = MutationUID::placeholder();
    let new_node_2 = MutationUID::placeholder();
    let new_node_3 = MutationUID::placeholder();

    let mutation = MutationBuilder::default()
        .set(vec![
            MutationUnit::new(new_node_1.clone())
                .predicate("pred", MutationPredicateValue::String("Some Value".to_string()))
                .predicate("edge", MutationPredicateValue::Edges(vec![
                    new_node_2.clone(),
                    new_node_3.clone()
                ])),
            MutationUnit::new(new_node_2.clone())
                .predicate("pred", MutationPredicateValue::String("Some Value".to_string()))
                .predicate("edge", MutationPredicateValue::Edges(vec![new_node_3.clone()])),
            MutationUnit::new(new_node_3.clone())
                .predicate("pred", MutationPredicateValue::String("Some Value".to_string()))
        ]).build().unwrap();

    println!("{}", serde_json::to_string_pretty(&mutation).unwrap());
}

#[test]
fn create_upsert() {
    let new_node_1 = MutationUID::placeholder();

    let query = QueryBuilder::default()
        .query_blocks(vec![
            QueryBlockBuilder::default()
                .query_type(QueryBlockType::query())
                .root_filter(Condition::has("pred"))
                .filter(Condition::EQ("pred".to_string(), ConditionValue::string("Some Value2")))
                .predicates(vec![
                    Predicate::ScalarVariable("SUBJECT_NODE".to_string(), Field::new("uid"))
                ])
                .build().unwrap(),
        ]).build().unwrap();

    let mutation = MutationBuilder::default()
        .set(vec![
            MutationUnit::new(new_node_1)
                .predicate("pred", MutationPredicateValue::string("Some Value2"))
                .predicate("upserted", MutationPredicateValue::Bool(true))
        ]).build().unwrap();

    let upsert = Upsert::new(query).upsert_block(UpsertBlock::new(mutation).cond(Condition::EQ(Condition::len("SUBJECT_NODE"), ConditionValue::literal_int(0))));

    println!("{}", serde_json::to_string_pretty(&upsert).unwrap());
}

#[test]
fn create_edge_upsert() {
    let node_key_1 = format!("node-key-{}", rand::random::<u32>());
    let node_key_2 = format!("node-key-{}", rand::random::<u32>());

    let query = QueryBuilder::default()
        .query_blocks(vec![
            QueryBlockBuilder::default()
                .query_type(QueryBlockType::Var)
                .root_filter(Condition::EQ(format!("node_key"), ConditionValue::String(node_key_1)))
                .predicates(vec![
                    Predicate::ScalarVariable("SUBJECT_NODE".to_string(), Field::new("uid"))
                ])
                .build().unwrap(),
            QueryBlockBuilder::default()
                .query_type(QueryBlockType::Var)
                .root_filter(Condition::EQ(format!("node_key"), ConditionValue::String(node_key_2)))
                .predicates(vec![
                    Predicate::ScalarVariable("SUBJECT_NODE_2".to_string(), Field::new("uid"))
                ])
                .build().unwrap(),
        ]).build().unwrap();

    let mutation = MutationBuilder::default()
        .set(vec![
            MutationUnit::new(MutationUID::variable("SUBJECT_NODE"))
                .predicate("edge_1", MutationPredicateValue::Edges(vec![
                    MutationUID::variable("SUBJECT_NODE_2")
                ])),
            MutationUnit::new(MutationUID::variable("SUBJECT_NODE_2"))
                .predicate("edge_2", MutationPredicateValue::Edge(MutationUID::variable("SUBJECT_NODE")))
        ]).build().unwrap();

    let upsert = Upsert::new(query).upsert_block(UpsertBlock::new(mutation));

    println!("{}", serde_json::to_string_pretty(&upsert).unwrap());
}

#[test]
fn generate_schema() {
    let mut schema = Schema::new()
        .add_definition(SchemaDefinition::new("Organization")
            .add_predicate(PredicateDefinition::new("first_seen", PredicateType::INT)
                .add_index(Indexing::INT))
            .add_predicate(PredicateDefinition::new("last_seen", PredicateType::INT)
                .add_index(Indexing::INT))
            .add_predicate(PredicateDefinition::new("node_key", PredicateType::String)
                .add_index(Indexing::EXACT)
                .upsert())
            .add_predicate(PredicateDefinition::new("org_name", PredicateType::String)
                .add_index(Indexing::TRIGRAM)));

    println!("{}", schema.to_string());
}