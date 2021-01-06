use crate::ToQueryString;
use crate::predicate::Predicate;

pub struct InnerBlock {
    predicates: Vec<Predicate>,
}

impl ToQueryString for InnerBlock {
    fn to_query_string(&self) -> String {

    }
}