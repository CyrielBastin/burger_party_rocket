use super::DAOFactory;
use crate::data_access::{DbConnection, DAOIngredient, DAODrink, DAOBurger, DAOCommand};

#[allow(dead_code)]
impl DAOFactory
{
    pub fn create_dao_ingredient() -> DAOIngredient {
        DAOIngredient {
            conn: DbConnection::new()
        }
    }
    pub fn create_dao_drink() -> DAODrink {
        DAODrink {
            conn: DbConnection::new()
        }
    }
    pub fn create_dao_burger() -> DAOBurger {
        DAOBurger {
            conn: DbConnection::new()
        }
    }
    pub fn create_dao_command() -> DAOCommand {
        DAOCommand {
            conn: DbConnection::new()
        }
    }
}
