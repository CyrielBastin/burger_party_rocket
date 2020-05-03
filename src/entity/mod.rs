use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ingredient
{
    id: u32,
    nom: String,
    description: String,
    prix: f32,
    calories: u16,
    image: String,
    quantite: u8
}
mod ingredient;

#[derive(Serialize, Deserialize, Debug)]
pub struct Boisson
{
    id: u32,
    nom: String,
    description: String,
    prix: f32,
    calories: u16,
    image: String,
    quantite: u8
}
mod boisson;

#[derive(Serialize, Deserialize, Debug)]
pub struct Burger
{
    id: u32,
    nom: String,
    description: String,
    prix: f32,
    recette: String,
    image: String,
    quantite: u8,
    ingredients: Vec<Ingredient>
}
mod burger;

#[derive(Serialize, Deserialize, Debug)]
pub struct Commande
{
    id: u32,
    terminal: u8,
    heure_complete: String,
    paye: bool,
    burgers: Vec<Burger>,
    boissons: Vec<Boisson>
}
mod commande;
