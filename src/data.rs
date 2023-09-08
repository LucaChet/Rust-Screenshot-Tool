use druid::{Data, Lens};
use im::Vector;
use serde::{Serialize, Deserialize};

#[derive(Clone, Data, PartialEq, Debug)]
pub enum Format{
    Png,
    Jpg,
    Gif,
}

#[derive(Clone, Data, Lens, Default)]
pub struct TodoState {
    pub todos: Vector<TodoItem>,
    pub new_text: String,
}

#[derive(Clone, Data, Lens, PartialEq)]
pub struct TodoItem {
    pub checked: bool,
    pub text: String,
    pub format: Format,
}






