use super::Ingredient;
use std::fmt::{Display, Formatter, Result};

impl Ingredient
{
    pub fn new() -> Self
    {
        Ingredient {
            id: 0,
            nom: "".to_string(),
            description: "".to_string(),
            prix: 0.0,
            calories: 0,
            image: "".to_string()
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
}

impl Display for Ingredient
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        write!(f, "Ingredient {{\n\tid: {},\n\tnom: {},\n\tdescription: {},\n\t\
                prix: {},\n\tcalories: {},\n\timage: {}\n}}",
        self.get_id(), self.get_nom(), self.get_description(), self.get_prix(),
        self.get_calories(), self.get_image())
    }
}
