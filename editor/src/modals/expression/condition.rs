use builder::bonus::Condition;



#[derive(Clone, Debug)]
pub struct ConditionSelector {

}

impl ConditionSelector {
    pub fn new<C>(condition: C) -> Self where C: Into<Option<Condition>> {
        Self {}
    }
}
