use super::{Burger, Ingredient};
use std::fmt::{Display, Formatter, Result};

impl Burger
{
    pub fn new() -> Self
    {
        Burger {
            id: 0,
            nom: "".to_string(),
            description: "".to_string(),
            prix: 0.0,
            recette: "".to_string(),
            image: "".to_string(),
            quantite: 0,
            ingredients: Vec::new()
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

    pub fn get_recette(&self) -> &str {
        &self.recette[..]
    }
    pub fn set_recette(&mut self, recette: &str) {
        self.recette = recette.to_string();
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

    pub fn get_ingredients(&self) -> &Vec<Ingredient> {
        &self.ingredients
    }
    pub fn set_ingredients(&mut self, ingredients: Vec<Ingredient>) {
        self.ingredients = ingredients;
    }
}

impl Display for Burger
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        write!(f, "Burger {{\n\tid: {},\n\tnom: {},\n\tdescription: {},\n\t\
                prix: {},\n\trecette: {},\n\timage: {},\n\tquantite: {}\n}}",
               self.get_id(), self.get_nom(), self.get_description(), self.get_prix(),
               self.get_recette(), self.get_image(), self.get_quantite())
    }
}