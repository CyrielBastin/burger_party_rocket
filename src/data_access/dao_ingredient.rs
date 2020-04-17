use crate::data_access::{DAO, DAOIngredient};
use crate::entity::Ingredient;
use mysql::{Row, from_row};
use mysql::prelude::Queryable;

type DataFromDb = (Option<u32>, Option<String>, Option<String>, Option<f32>,
                   Option<u16>, Option<u8>, Option<u8>, Option<String>);

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
        let query = "SELECT * FROM `ingredient` WHERE `id` = ?";
        let result: Row = self.conn.exec_first(query, (id,)).unwrap().unwrap();
        let datas = from_row::<DataFromDb>(result);
        let mut ingredient = Ingredient::new();
        ingredient.feed_from_db(datas);

        ingredient
    }

    fn find_all(&mut self) -> Vec<Ingredient>
    {
        let query = "SELECT * FROM `ingredient`";
        let result: Vec<Row> = self.conn.exec(query, ()).unwrap();
        let mut list_ingredients = Vec::new();
        for row in result {
            let datas = from_row::<DataFromDb>(row);
            let mut ingredient = Ingredient::new();
            ingredient.feed_from_db(datas);
            list_ingredients.push(ingredient);
        }

        list_ingredients
    }
}