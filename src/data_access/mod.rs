use mysql::{Pool, PooledConn};

pub struct DbConnection<'a>
{
    ip: &'a str,
    user: &'a str,
    password: &'a str,
    database: &'a str,
    port: u16,
    pool: Option<Pool>
}
mod db_connection;

pub trait DAO<T>
{
    fn create(&self, _obj: T) -> bool;
    fn update(&self, _obj: T) -> bool;
    fn delete(&self, _obj: T) -> bool;
    fn find_by_id(&mut self, id: u32) -> T;
    fn find_all(&mut self) -> Vec<T>;
}


pub struct DAOFactory {}
mod dao_factory;

pub struct DAOIngredient {
    conn: PooledConn
} mod dao_ingredient;
pub struct DAODrink {
    conn: PooledConn
} mod dao_drink;
pub struct DAOBurger {
    conn: PooledConn
} mod dao_burger;
pub struct DAOCommand {
    conn: PooledConn
} mod dao_command;

pub mod command_details_handler;
