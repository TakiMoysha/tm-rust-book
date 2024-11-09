// Abstract type
pub trait TraitBasedType {
    fn method_d(&self);
}

impl<T: TraitBasedType + ?Sized> TraitBasedType for Box<T> {
    fn method_d(&self) {
        todo!()
    }
}
pub struct SpecificType {
    _private_field: String,
}

impl SpecificType {
    pub fn new() -> Self {
        Self {
            _private_field: String::new(),
        }
    }
}

impl TraitBasedType for SpecificType {
    fn method_d(&self) {
        todo!()
    }
}

pub struct AnotherSpecificType {
    _private_field: i32,
}

impl AnotherSpecificType {
    pub fn new(arg: Option<i32>) -> Self {
        Self {
            _private_field: arg.unwrap_or_default(),
        }
    }
}

impl TraitBasedType for AnotherSpecificType {
    fn method_d(&self) {
        todo!()
    }
}
