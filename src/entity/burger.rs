use super::{Burger, Ingredient};
use crate::types::db_types::BurFromDb;

#[allow(dead_code)]
impl Burger
{
    pub fn new() -> Self
    {
        Burger {
            id: 0,
            name: "".to_string(),
            description: "".to_string(),
            price: 0.0,
            recipe: "".to_string(),
            image: "".to_string(),
            quantity: 0,
            ingredients: Vec::new()
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn get_name(&self) -> &str {
        &self.name[..]
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_description(&self) -> &str {
        &self.description[..]
    }
    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    pub fn get_price(&self) -> f32 {
        self.price
    }
    pub fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    pub fn get_recipe(&self) -> &str {
        &self.recipe[..]
    }
    pub fn set_recipe(&mut self, recipe: &str) {
        self.recipe = recipe.to_string();
    }

    pub fn get_image(&self) -> &str {
        &self.image[..]
    }
    pub fn set_image(&mut self, image: &str) {
        self.image = image.to_string();
    }

    pub fn get_quantity(&self) -> u8 {
        self.quantity
    }
    pub fn set_quantity(&mut self, quantity: u8) {
        self.quantity = quantity;
    }

    pub fn get_ingredients(&self) -> &Vec<Ingredient> {
        &self.ingredients
    }
    pub fn set_ingredients(&mut self, ingredients: Vec<Ingredient>) {
        self.ingredients = ingredients;
    }
}

impl Burger
{
    pub fn feed_from_db(&mut self, datas: BurFromDb)
    {
        self.set_id(match datas.0 {
            Some(x) => x,
            None => 0
        });
        self.set_name(match &datas.1 {
            Some(x) => x,
            None => ""
        });
        self.set_description(match &datas.3 {
            Some(x) => x,
            None => ""
        });
        self.set_price(match datas.2 {
            Some(x) => x,
            None => 0.0
        });
        self.set_recipe(match &datas.4 {
            Some(x) => x,
            None => ""
        });
        self.set_image(match &datas.5 {
            Some(x) => x,
            None => ""
        });
    }
}
