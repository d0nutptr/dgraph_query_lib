use itertools::Itertools;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Clone)]
pub struct Schema {
    pub definitions: Vec<SchemaDefinition>
}

impl Schema {
    pub fn new() -> Self {
        Self {
            definitions: vec![]
        }
    }

    pub fn add_definition(mut self, definition: SchemaDefinition) -> Self {
        self.definitions.push(definition);
        self
    }

    pub fn add_definition_ref(&mut self, definition: SchemaDefinition) {
        self.definitions.push(definition);
    }
}

impl ToString for Schema {
    fn to_string(&self) -> String {
        let mut representation = self.definitions.iter()
            .map(|def| def.to_string())
            .join("\n");
        representation += "\n\n";

        let predicate_map: HashMap<&String, &PredicateDefinition> = HashMap::from_iter(self.definitions.iter()
            .map(|def| &def.predicates)
            .flatten()
            .map(|pred| (&pred.name, pred)));

        representation += &predicate_map.iter().map(|(name, pred)| pred.to_string()).join("\n");

        representation
    }
}

#[derive(Clone)]
pub struct SchemaDefinition {
    name: String,
    predicates: Vec<PredicateDefinition>
}

impl SchemaDefinition {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            predicates: vec![]
        }
    }

    pub fn add_predicate(mut self, predicate: PredicateDefinition) -> Self {
        self.predicates.push(predicate);
        self
    }

    pub fn add_predicate_ref(&mut self, predicate: PredicateDefinition) {
        self.predicates.push(predicate);
    }

}

impl ToString for SchemaDefinition {
    fn to_string(&self) -> String {
        format!(
            "type {type_name} {{\n\
                \t{predicates}\n\
            }}",
        type_name = &self.name,
        predicates = &self.predicates.iter().map(|pred| &pred.name).join("\n\t"))
    }
}

#[derive(Clone)]
pub struct PredicateDefinition {
    name: String,
    predicate_type: PredicateType,
    indexing: Vec<Indexing>,
    upsert: bool
}

impl PredicateDefinition {
    pub fn new(name: &str, predicate_type: PredicateType) -> Self {
        Self {
            name: name.to_string(),
            predicate_type,
            indexing: vec![],
            upsert: false
        }
    }

    pub fn add_index(mut self, index: Indexing) -> Self {
        self.indexing.push(index);
        self
    }

    pub fn add_index_ref(&mut self, index: Indexing) {
        self.indexing.push(index);
    }

    pub fn upsert(mut self) -> Self {
        self.upsert = true;
        self
    }
}

impl ToString for PredicateDefinition {
    fn to_string(&self) -> String {
        let mut index = format!("");

        if !self.indexing.is_empty() {
            index = format!(
                " @index({})",
                &self.indexing.iter().map(|pred| pred.to_string()).join(", ")
            );
        }

        let upsert = if self.upsert {
            format!(" @upsert")
        } else {
            format!("")
        };

        format!(
            "{name}: {ptype}{index}{upsert} .",
            name = &self.name,
            ptype = self.predicate_type.to_string(),
            index = index,
            upsert = upsert
        )
    }
}

#[derive(Clone)]
pub enum PredicateType {
    String,
    StringArray,
    UID,
    UIDArray,
    INT,
    INTArray
}

impl ToString for PredicateType {
    fn to_string(&self) -> String {
        match self {
            PredicateType::String => format!("string"),
            PredicateType::StringArray => format!("[string]"),
            PredicateType::UID => format!("uid"),
            PredicateType::UIDArray => format!("[uid]"),
            PredicateType::INT => format!("int"),
            PredicateType::INTArray => format!("[int]")
        }
    }
}

#[derive(Clone)]
pub enum Indexing {
    TERM,
    TRIGRAM,
    INT,
    EXACT
}

impl ToString for Indexing {
    fn to_string(&self) -> String {
        match self {
            Indexing::TERM => format!("term"),
            Indexing::TRIGRAM => format!("trigram"),
            Indexing::INT => format!("int"),
            Indexing::EXACT => format!("exact")
        }
    }
}