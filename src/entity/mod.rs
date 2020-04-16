pub struct Ingredient
{
    id: u8,
    nom: String,
    description: String,
    prix: f32,
    calories: u16,
    image: String
}
mod ingredient;

pub struct Boisson
{
    id: u8,
    nom: String,
    description: String,
    prix: f32,
    calories: u16,
    image: String,
    quantite: u8
}
mod boisson;

pub struct Burger
{
    id: u8,
    nom: String,
    description: String,
    prix: f32,
    recette: String,
    image: String,
    quantite: u8,
    ingredients: Vec<Ingredient>
}
mod burger;
