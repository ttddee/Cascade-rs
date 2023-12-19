pub enum NodeProperty {
    Float(PropertyData<f32>),
    Int(PropertyData<i32>),
}

impl NodeProperty {
    pub fn new_float(
        property_name: String,
        minimum: f32,
        maximum: f32,
        step_size: f32,
        initial_value: f32,
    ) -> Self {
        NodeProperty::Float(PropertyData {
            name: property_name,
            min: minimum,
            max: maximum,
            step: step_size,
            value: initial_value,
        })
    }

    pub fn new_int(
        property_name: String,
        minimum: i32,
        maximum: i32,
        step_size: i32,
        initial_value: i32,
    ) -> Self {
        NodeProperty::Int(PropertyData {
            name: property_name,
            min: minimum,
            max: maximum,
            step: step_size,
            value: initial_value,
        })
    }
}

pub struct PropertyData<T> {
    name: String,
    min: T,
    max: T,
    step: T,
    value: T,
}

impl<T> PropertyData<T> {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn min(&self) -> &T {
        &self.min
    }

    pub fn max(&self) -> &T {
        &self.max
    }

    pub fn step(&self) -> &T {
        &self.step
    }

    pub fn value(&mut self) -> &mut T {
        &mut self.value
    }
}
