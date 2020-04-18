use std::collections::HashMap;

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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
