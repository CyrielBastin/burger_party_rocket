use mysql::{Row, from_row};
use mysql::prelude::Queryable;
use crate::data_access::{DAO, DAOCommande, DAOFactory};
use crate::entity::{Commande, Boisson, Burger};
use crate::data_access::dao_burger::fetch_ingredients;
use crate::types::db_types::{CmdFromDb, DrkFromDb, BurFromDb};


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
        let query = "SELECT id, n_table, CAST(heure AS CHAR), n_serveur, paye \
                          FROM `commande` WHERE `id` = ?";
        let result: Row = self.conn.exec_first(query, (id,)).unwrap().unwrap();
        let datas = from_row::<CmdFromDb>(result);
        let mut commande = Commande::new();
        self.get_all_details(&mut commande, datas, id);

        commande
    }

    fn find_all(&mut self) -> Vec<Commande>
    {
        let query = "SELECT id, n_table, CAST(heure AS CHAR), n_serveur, paye FROM `commande`";
        let result: Vec<Row> = self.conn.exec(query, ()).unwrap();
        let mut list_commandes = Vec::new();
        for row in result
        {
            let datas = from_row::<CmdFromDb>(row);
            let mut commande = Commande::new();
            let cmd_id = datas.0.unwrap();
            self.get_all_details(&mut commande, datas, cmd_id);
            list_commandes.push(commande);
        }

        list_commandes
    }
}

#[allow(dead_code)]
impl DAOCommande
{
    pub fn get_id_from_datetime(&mut self, datetime: &str) -> u32
    {
        let query = "SELECT `id` FROM `commande` WHERE `heure` = ?";
        let result: Row = self.conn.exec_first(query, (datetime,)).unwrap().unwrap();
        let data = from_row::<u32>(result);

        data
    }

    fn get_all_details(&mut self, commande: &mut Commande, datas: CmdFromDb, id: u32)
    {
        commande.feed_from_db(datas);
        // Once the command has been retrieved, we fetch drinks
        commande.set_boissons(fetch_boissons(self, id));
        // Finally, we fetch burgers and their ingredients
        commande.set_burgers(fetch_burgers(self, id));
    }
}

fn fetch_boissons(dao: &mut DAOCommande, commande_id: u32) -> Vec<Boisson>
{
    let query = "SELECT boisson.*, commande_boisson.quantite \
                      FROM commande_boisson \
                      INNER JOIN boisson \
                      ON commande_boisson.boisson_id = boisson.id \
                      WHERE commande_boisson.commande_id = ?";

    let result: Vec<Row> = dao.conn.exec(query, (commande_id,)).unwrap();
    let mut list_boissons = Vec::new();
    push_row_to_vec_boi(result, &mut list_boissons);

    list_boissons
}
fn push_row_to_vec_boi(result: Vec<Row>, list_boi: &mut Vec<Boisson>)
{
    for row in result
    {
        let datas = from_row::<DrkFromDb>(row);
        let mut boisson = Boisson::new();
        // adding the quantity of each drinks to the boisson.quantite field
        boisson.set_quantite(datas.8.unwrap());
        boisson.feed_from_db(datas);
        list_boi.push(boisson);
    }
}

fn fetch_burgers(dao: &mut DAOCommande, commande_id: u32) -> Vec<Burger>
{
    let query = "SELECT burger.*, commande_burger.quantite \
                      FROM commande_burger \
                      INNER JOIN burger \
                      ON commande_burger.burger_id = burger.id \
                      WHERE commande_burger.commande_id = ?";

    let result: Vec<Row> = dao.conn.exec(query, (commande_id,)).unwrap();
    let mut list_burgers= Vec::new();
    push_row_to_vec_bur(result, &mut list_burgers);

    list_burgers
}
fn push_row_to_vec_bur(result: Vec<Row>, list_bur: &mut Vec<Burger>)
{
    for row in result
    {
        let datas = from_row::<BurFromDb>(row);
        let mut burger = Burger::new();
        // adding the quantity of each burgers to the burger.quantite field
        burger.set_quantite(datas.6.unwrap());
        burger.feed_from_db(datas);
        burger.set_ingredients(fetch_ingredients(&mut DAOFactory::create_dao_burger(), burger.get_id()));
        list_bur.push(burger);
    }
}
