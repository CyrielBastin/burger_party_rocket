use crate::data_access::{DAO, DAOBurger};
use crate::entity::Burger;

impl DAO<Burger> for DAOBurger
{
    fn create(&self, _obj: Burger) -> bool {
        false
    }

    fn update(&self, _obj: Burger) -> bool {
        false
    }

    fn delete(&self, _obj: Burger) -> bool {
        false
    }

    fn find_by_id(&mut self, id: u32) -> Burger
    {
        unimplemented!()
    }

    fn find_all(&mut self) -> Vec<Burger>
    {
        unimplemented!()
    }
}
