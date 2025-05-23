use std::path::PathBuf;

pub enum NodeProperty {
    Float(NumberData<f32>),
    Int(NumberData<i32>),
    Choice(ChoiceData),
    PathList(PathListData),
}

impl NodeProperty {
    pub fn new_float(name: String, min: f32, max: f32, step: f32, value: f32) -> Self {
        NodeProperty::Float(NumberData {
            name,
            min,
            max,
            step,
            value,
        })
    }

    pub fn new_int(name: String, min: i32, max: i32, step: i32, value: i32) -> Self {
        NodeProperty::Int(NumberData {
            name,
            min,
            max,
            step,
            value,
        })
    }

    pub fn new_choice(name: String, choices: Vec<String>, index: usize) -> Self {
        NodeProperty::Choice(ChoiceData {
            name,
            choices,
            index,
        })
    }

    pub fn new_path_list() -> Self {
        NodeProperty::PathList(PathListData {
            index: 0,
            list: Vec::<PathBuf>::new(),
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

pub struct PathListData {
    index: usize,
    list: Vec<PathBuf>,
}

impl PathListData {
    pub fn index(&self) -> &usize {
        &self.index
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn current_entry(&self) -> &PathBuf {
        &self.list[self.index]
    }

    pub fn add(&mut self, entry: PathBuf) {
        self.list.push(entry);
    }

    pub fn list(&self) -> &Vec<PathBuf> {
        &self.list
    }
}
