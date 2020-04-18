mod data_access;
mod entity;

use data_access::{DAOFactory, DAO};

fn main()
{
    println!("Hello, world!");
    let mut repo = DAOFactory::create_dao_burger();
    let bur = repo.find_by_id(3);
    println!("{:#?}", bur);
}
