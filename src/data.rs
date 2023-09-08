use druid::{Data, Lens, ImageBuf, widget::{Image, Container}};
use im::Vector;
use image::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Data, PartialEq, Debug, Serialize, Deserialize)]
pub enum Format{
    Empty,
    Png,
    Jpg,
    Gif,
}

impl Format { 
    pub fn to_string(&self) -> String { 
        match self { 
            Format::Empty => "".to_string(),
            Format::Jpg => ".jpg".to_string(), 
            Format::Png => ".png".to_string(), 
            Format::Gif => ".gif".to_string(),  
        } 
    } 
}

#[derive(Clone, Data, Lens, PartialEq, Serialize, Deserialize)]
pub struct Screenshot {
    pub name: String,
    pub format: Format,
}

impl Screenshot{
    pub fn new(name: String, format: Format)->Self{
        Self { 
            name,
            format,
        }
    }
}

#[derive(Clone, Lens, PartialEq)]
pub struct ScreenImage{
    image: ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

impl ScreenImage{
    pub fn new(image: ImageBuffer<image::Rgba<u8>, Vec<u8>>)->Self{
        Self { image, }
    }
}






