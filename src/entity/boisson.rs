use super::Boisson;
use std::fmt::{Display, Formatter, Result};

impl Boisson
{
    pub fn new() -> Self
    {
        Boisson {
            id: 0,
            nom: "".to_string(),
            description: "".to_string(),
            prix: 0.0,
            calories: 0,
            image: "".to_string(),
            quantite: 0
        }
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }
    pub fn set_id(&mut self, id: u8) {
        self.id = id;
    }

    pub fn get_nom(&self) -> &str {
        &self.nom[..]
    }
    pub fn set_nom(&mut self, nom: &str) {
        self.nom = nom.to_string();
    }

    pub fn get_description(&self) -> &str {
        &self.description[..]
    }
    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    pub fn get_prix(&self) -> f32 {
        self.prix
    }
    pub fn set_prix(&mut self, prix: f32) {
        self.prix = prix;
    }

    pub fn get_calories(&self) -> u16 {
        self.calories
    }
    pub fn set_calories(&mut self, calories: u16) {
        self.calories = calories;
    }

    pub fn get_image(&self) -> &str {
        &self.image[..]
    }
    pub fn set_image(&mut self, image: &str) {
        self.image = image.to_string();
    }

    pub fn get_quantite(&self) -> u8 {
        self.quantite
    }
    pub fn set_quantite(&mut self, quantite: u8) {
        self.quantite = quantite;
    }
}

impl Display for Boisson
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        write!(f, "Boisson {{\n\tid: {},\n\tnom: {},\n\tdescription: {},\n\t\
                prix: {},\n\tcalories: {},\n\timage: {},\n\tquantite: {}\n}}",
               self.get_id(), self.get_nom(), self.get_description(), self.get_prix(),
               self.get_calories(), self.get_image(), self.get_quantite())
    }
}
