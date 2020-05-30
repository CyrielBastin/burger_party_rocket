use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ingredient
{
    id: u32,
    name: String,
    description: String,
    price: f32,
    calories: u16,
    image: String,
    quantity: u8
}
mod ingredient;

#[derive(Serialize, Deserialize, Debug)]
pub struct Drink
{
    id: u32,
    name: String,
    description: String,
    price: f32,
    calories: u16,
    image: String,
    quantity: u8
}
mod drink;

#[derive(Serialize, Deserialize, Debug)]
pub struct Burger
{
    id: u32,
    name: String,
    description: String,
    price: f32,
    recipe: String,
    image: String,
    quantity: u8,
    ingredients: Vec<Ingredient>
}
mod burger;

#[derive(Serialize, Deserialize, Debug)]
pub struct Command
{
    id: u32,
    terminal: u8,
    date_time: String,
    payed: bool,
    burgers: Vec<Burger>,
    drinks: Vec<Drink>
}
mod command;
