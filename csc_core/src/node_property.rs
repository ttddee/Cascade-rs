pub enum NodeProperty {
    Float (PropertyData<f32>),
    Int (PropertyData<i32>),
}

impl NodeProperty {
    pub fn new_float(minimum: f32, maximum: f32, step_size: f32, initial_value: f32) -> Self {
        NodeProperty::Float(PropertyData { min: minimum, max: maximum, step: step_size, value: initial_value })
    }
}

pub struct PropertyData<T> {
    min: T,
    max: T,
    step: T,
    value: T,
}

impl<T> PropertyData<T> {
    // pub fn new(minimum: T, maximum: T, stepsize: T, current_value:T) -> PropertyData<T> {
    //     min
    // }

    pub fn value(&self) -> &T {
        &self.value
    }
}


