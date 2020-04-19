use crate::data_access::{DAO, DAOBoisson};
use crate::entity::Boisson;
use mysql::{Row, from_row};
use mysql::prelude::Queryable;
use crate::types::db_types::DrkFromDb;


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
        let query = "SELECT *, 0 AS quantite FROM `boisson` WHERE `id` = ?";
        let result: Row = self.conn.exec_first(query, (id,)).unwrap().unwrap();
        let datas = from_row::<DrkFromDb>(result);
        let mut boisson = Boisson::new();
        boisson.feed_from_db(datas);

        boisson
    }

    fn find_all(&mut self) -> Vec<Boisson>
    {
        let query = "SELECT *, 0 AS quantite FROM `boisson`";
        let result: Vec<Row> = self.conn.exec(query, ()).unwrap();
        let mut list_boissons = Vec::new();
        for row in result {
            let datas = from_row::<DrkFromDb>(row);
            let mut boisson = Boisson::new();
            boisson.feed_from_db(datas);
            list_boissons.push(boisson);
        }

        list_boissons
    }
}
