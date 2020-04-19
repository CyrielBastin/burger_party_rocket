mod data_access;
mod entity;
mod types;

use data_access::{DAOFactory, DAO};

fn main()
{
    println!("Hello, world!");
    let mut repo = DAOFactory::create_dao_commande();
    let a = repo.find_by_id(2);
    println!("{:#?}", a);
}
