use crate::data_access::{DAO, DAODrink};
use crate::entity::Drink;
use mysql::{Row, from_row};
use mysql::prelude::Queryable;
use crate::types::db_types::DrkFromDb;


impl DAO<Drink> for DAODrink
{
    fn create(&self, _obj: Drink) -> bool {
        false
    }

    fn update(&self, _obj: Drink) -> bool {
        false
    }

    fn delete(&self, _obj: Drink) -> bool {
        false
    }

    fn find_by_id(&mut self, id: u32) -> Drink
    {
        let query = "SELECT *, 0 AS quantite FROM `boisson` WHERE `id` = ?";
        let result: Row = self.conn.exec_first(query, (id,)).unwrap().unwrap();
        let datas = from_row::<DrkFromDb>(result);
        let mut drink = Drink::new();
        drink.feed_from_db(datas);

        drink
    }

    fn find_all(&mut self) -> Vec<Drink>
    {
        let query = "SELECT *, 0 AS quantite FROM `boisson`";
        let result: Vec<Row> = self.conn.exec(query, ()).unwrap();
        let mut list_drinks = Vec::new();
        for row in result {
            let datas = from_row::<DrkFromDb>(row);
            let mut drink = Drink::new();
            drink.feed_from_db(datas);
            list_drinks.push(drink);
        }

        list_drinks
    }
}

impl DAODrink
{
    pub fn find_id_all(&mut self) -> Vec<u32>
    {
        let query = "SELECT `id` FROM `boisson`";

        self.conn.exec(query, ()).unwrap()
    }
}
