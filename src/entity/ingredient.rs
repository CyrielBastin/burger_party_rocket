use super::Ingredient;
use crate::types::db_types::IngrFromDb;

#[allow(dead_code)]
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
            image: "".to_string(),
            quantite: 0
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn set_id(&mut self, id: u32) {
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

impl Ingredient
{
    pub fn feed_from_db(&mut self, datas: IngrFromDb)
    {
        self.set_id(match datas.0 {
            Some(x) => x,
            None => 0
        });
        self.set_nom(match &datas.1 {
            Some(x) => x,
            None => ""
        });
        self.set_description(match &datas.2 {
            Some(x) => x,
            None => ""
        });
        self.set_prix(match datas.3 {
            Some(x) => x,
            None => 0.0
        });
        self.set_calories(match datas.4 {
            Some(x) => x,
            None => 0
        });
        self.set_image(match &datas.7 {
            Some(x) => x,
            None => ""
        });
        self.set_quantite(match datas.8 {
            Some(x) => x,
            None => 0
        })
    }
}
