use builder::bonus::Value;



#[derive(Debug, Clone)]
pub struct ValueSelector {

}

impl ValueSelector {
    pub fn new<V>(value: V) -> Self where V: Into<Option<Value>> {
        Self {}
    }
}
