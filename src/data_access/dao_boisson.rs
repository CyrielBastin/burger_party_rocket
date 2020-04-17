use crate::data_access::{DAO, DAOBoisson};
use crate::entity::Boisson;
use mysql::{Row, from_row};
use mysql::prelude::Queryable;

type DataFromDb = (Option<u8>, Option<String>, Option<f32>, Option<String>,
                   Option<u16>, Option<u8>, Option<u8>, Option<String>);

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
        let query = "SELECT * FROM `ingredient` WHERE `id` = ?";
        let result: Row = self.conn.exec_first(query, (id,)).unwrap().unwrap();
        let datas = from_row::<DataFromDb>(result);
        let mut ingredient = Ingredient::new();
        ingredient.feed_from_db(datas);

        ingredient
    }

    fn find_all(&mut self) -> Vec<Boisson>
    {
    }
}
