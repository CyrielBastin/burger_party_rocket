mod data_access;
mod entity;

use data_access::DAOFactory;
use crate::data_access::DAO;

fn main()
{
    println!("Hello, world!");
    let mut repo = DAOFactory::create_dao_boisson();
    let b = repo.find_by_id(10);
    println!("{}", b);
    let bb = repo.find_all();
    for i in &bb {
        println!("{}", i);
    }
}
