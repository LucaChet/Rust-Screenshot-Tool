use druid::{Data, Lens, ImageBuf, widget::{Image, Container}};
use im::Vector;
use image::*;
use serde::{Serialize, Deserialize};
use screenshots::Screen;
use druid::widget::SvgData;

use self::screenshot_derived_lenses::new_name;

#[derive(Clone, Data, PartialEq, Debug, Serialize, Deserialize)]
pub enum Format{
    MainFormat,
    Png,
    Jpg,
    Gif,
}

impl Format { 
    pub fn to_string(&self) -> String { 
        match self { 
            Format::MainFormat => "".to_string(),
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
    pub new_name: String,
}

impl Screenshot{
    pub fn new(name: String, format: Format, newname: String)->Self{
        Self { 
            name,
            format,
            new_name: newname,
        }
    }
    pub fn set_label_text(&mut self, text: String) {
        self.name = text;
    }
}

// fn load_png_image(file_path: &str) -> Result<DynamicImage, image::error::ImageError> {
//     // Carica l'immagine dal percorso specificato
//     let image = image::open(file_path)?;

//     Ok(image)
// }

// pub fn prendi_screen()->DynamicImage{
//     let file_path = "target/screens/2023-09-09_15-51-15-610481200_UTC.png";

//     // Carica l'immagine dal percorso specificato
//     match load_png_image(file_path) {
//         Ok(image) => {
//             // Puoi ora utilizzare l'immagine come variabile
//             let width = image.width();
//             let height = image.height();
//             println!("Dimensioni dell'immagine: {} x {}", width, height);
//             return image;
//         }
//         Err(err) => panic!("Errore durante il caricamento dell'immagine: {:?}", err),
//     }
// }






