use crate::data_access::{DAO, DAOIngredient};
use crate::entity::Ingredient;

impl DAO<Ingredient> for DAOIngredient
{
    fn create(&self, _obj: Ingredient) -> bool {
        false
    }

    fn update(&self, _obj: Ingredient) -> bool {
        false
    }

    fn delete(&self, _obj: Ingredient) -> bool {
        false
    }

    fn find_by_id(&mut self, id: u32) -> Ingredient
    {
        unimplemented!()
    }

    fn find_all(&mut self) -> Vec<Ingredient>
    {
        unimplemented!()
    }
}
