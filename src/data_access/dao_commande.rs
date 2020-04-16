use crate::data_access::{DAO, DAOCommande};
use crate::entity::Commande;

impl DAO<Commande> for DAOCommande
{
    fn create(&self, _obj: Commande) -> bool {
        false
    }

    fn update(&self, _obj: Commande) -> bool {
        false
    }

    fn delete(&self, _obj: Commande) -> bool {
        false
    }

    fn find_by_id(&mut self, id: u32) -> Commande
    {
        unimplemented!()
    }

    fn find_all(&mut self) -> Vec<Commande>
    {
        unimplemented!()
    }
}
