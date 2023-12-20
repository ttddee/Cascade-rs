pub enum NodeProperty {
    Float(NumberData<f32>),
    Int(NumberData<i32>),
    Choice(ChoiceData),
}

impl NodeProperty {
    pub fn new_float(
        property_name: String,
        minimum: f32,
        maximum: f32,
        step_size: f32,
        initial_value: f32,
    ) -> Self {
        NodeProperty::Float(NumberData {
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
        NodeProperty::Int(NumberData {
            name: property_name,
            min: minimum,
            max: maximum,
            step: step_size,
            value: initial_value,
        })
    }

    pub fn new_choice(
        property_name: String,
        choices_list: Vec<String>,
        initial_index: usize,
    ) -> Self {
        NodeProperty::Choice(ChoiceData {
            name: property_name,
            choices: choices_list,
            index: initial_index,
        })
    }
}

pub struct NumberData<T> {
    name: String,
    min: T,
    max: T,
    step: T,
    value: T,
}

impl<T> NumberData<T> {
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

pub struct ChoiceData {
    name: String,
    choices: Vec<String>,
    index: usize,
}

impl ChoiceData {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn choices(&self) -> &Vec<String> {
        &self.choices
    }

    pub fn index(&mut self) -> &mut usize {
        &mut self.index
    }
}
