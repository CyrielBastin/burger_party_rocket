use crate::data_access::{DAO, DAOIngredient};
use crate::entity::Ingredient;
use mysql::{Row, from_row};
use mysql::prelude::Queryable;
use crate::types::db_types::IngrFromDb;
use crate::data_access::dao_burger::push_row_to_vec_ingr;


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
        let query = "SELECT *, 0 AS quantite FROM `ingredient` WHERE `id` = ?";
        let result: Row = self.conn.exec_first(query, (id,)).unwrap().unwrap();
        let datas = from_row::<IngrFromDb>(result);
        let mut ingredient = Ingredient::new();
        ingredient.feed_from_db(datas);

        ingredient
    }

    fn find_all(&mut self) -> Vec<Ingredient>
    {
        let query = "SELECT *, 0 AS quantite FROM `ingredient`";
        let result: Vec<Row> = self.conn.exec(query, ()).unwrap();
        let mut list_ingredients = Vec::new();
        push_row_to_vec_ingr(result, &mut list_ingredients);

        list_ingredients
    }
}
