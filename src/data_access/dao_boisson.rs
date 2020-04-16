use crate::data_access::{DAO, DAOBoisson};
use crate::entity::Boisson;

impl DAO<Boisson> for DAOBoisson
{
    fn create(&self, _obj: Boisson) -> bool {
        false
    }

    fn update(&self, _obj: Boisson) -> bool {
        false
    }

    fn delete(&self, _obj: Boisson) -> bool {
        false
    }

    fn find_by_id(&mut self, id: u32) -> Boisson
    {
        unimplemented!()
    }

    fn find_all(&mut self) -> Vec<Boisson>
    {
        unimplemented!()
    }
}
