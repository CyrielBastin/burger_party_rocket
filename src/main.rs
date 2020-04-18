use crate::entity::Commande;

mod data_access;
mod entity;

fn main()
{
    println!("Hello, world!");
    let commande = Commande::get_local_to_string();
    println!("{}", commande);
}
