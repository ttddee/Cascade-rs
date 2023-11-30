pub enum NodeProperty {
    Float (PropertyData<f32>),
    Int (PropertyData<i32>),
}

pub struct PropertyData<T> {
    min: T,
    max: T,
    step: T,
    value: T,
}

impl<T> PropertyData<T> {
    fn value(&self) -> &T {
        &self.value
    }
}


