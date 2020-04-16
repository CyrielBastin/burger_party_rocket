use super::DAOFactory;
use crate::data_access::{DbConnection, DAOIngredient, DAOBoisson, DAOBurger, DAOCommande};

impl DAOFactory
{
    pub fn create_dao_ingredient() -> DAOIngredient {
        DAOIngredient {
            conn: DbConnection::new()
        }
    }
    pub fn create_dao_boisson() -> DAOBoisson {
        DAOBoisson {
            conn: DbConnection::new()
        }
    }
    pub fn create_dao_burger() -> DAOBurger {
        DAOBurger {
            conn: DbConnection::new()
        }
    }
    pub fn create_dao_commande() -> DAOCommande {
        DAOCommande {
            conn: DbConnection::new()
        }
    }
}
