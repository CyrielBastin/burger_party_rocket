use mysql::{Row, from_row};
use mysql::prelude::Queryable;
use crate::data_access::{DAO, DAOCommand, DAOFactory};
use crate::entity::{Command, Drink, Burger};
use crate::data_access::dao_burger::fetch_ingredients;
use crate::types::db_types::{CmdFromDb, DrkFromDb, BurFromDb};


impl DAO<Command> for DAOCommand
{
    fn create(&mut self, _obj: Command) -> bool
    {
        let first_query = "INSERT INTO `commande` (n_table, n_serveur, paye, heure) \
                                 VALUES (1, ?, ?, ?)";

        let result: () = self.conn.exec_drop(first_query, (1, 1, _obj.get_date_time())).unwrap();
        /////////////////////////
        false
    }

    fn update(&mut self, _obj: Command) -> bool {
        false
    }

    fn delete(&mut self, _obj: Command) -> bool {
        false
    }

    fn find_by_id(&mut self, id: u32) -> Command
    {
        let query = "SELECT id, n_table, CAST(heure AS CHAR), n_serveur, paye \
                          FROM `commande` WHERE `id` = ?";
        let result: Row = self.conn.exec_first(query, (id,)).unwrap().unwrap();
        let datas = from_row::<CmdFromDb>(result);
        let mut command = Command::new();
        self.get_all_details(&mut command, datas, id);

        command
    }

    fn find_all(&mut self) -> Vec<Command>
    {
        let query = "SELECT id, n_table, CAST(heure AS CHAR), n_serveur, paye FROM `commande`";
        let result: Vec<Row> = self.conn.exec(query, ()).unwrap();
        let mut list_commands = Vec::new();
        for row in result
        {
            let datas = from_row::<CmdFromDb>(row);
            let mut command = Command::new();
            let cmd_id = datas.0.unwrap();
            self.get_all_details(&mut command, datas, cmd_id);
            list_commands.push(command);
        }

        list_commands
    }
}

#[allow(dead_code)]
impl DAOCommand
{
    pub fn get_id_from_datetime(&mut self, datetime: &str) -> u32
    {
        let query = "SELECT `id` FROM `commande` WHERE `heure` = ?";
        let result: Row = self.conn.exec_first(query, (datetime,)).unwrap().unwrap();
        let data = from_row::<u32>(result);

        data
    }

    fn get_all_details(&mut self, command: &mut Command, datas: CmdFromDb, id: u32)
    {
        command.feed_from_db(datas);
        // Once the command has been retrieved, we fetch drinks
        command.set_drinks(fetch_drinks(self, id));
        // Finally, we fetch burgers and their ingredients
        command.set_burgers(fetch_burgers(self, id));
    }
}

fn fetch_drinks(dao: &mut DAOCommand, command_id: u32) -> Vec<Drink>
{
    let query = "SELECT boisson.*, commande_boisson.quantite \
                      FROM commande_boisson \
                      INNER JOIN boisson \
                      ON commande_boisson.boisson_id = boisson.id \
                      WHERE commande_boisson.commande_id = ?";

    let result: Vec<Row> = dao.conn.exec(query, (command_id,)).unwrap();
    let mut list_drinks = Vec::new();
    push_row_to_vec_drk(result, &mut list_drinks);

    list_drinks
}
fn push_row_to_vec_drk(result: Vec<Row>, list_drk: &mut Vec<Drink>)
{
    for row in result
    {
        let datas = from_row::<DrkFromDb>(row);
        let mut drink = Drink::new();
        // adding the quantity of each drinks to the drink.quantity field
        drink.set_quantity(datas.8.unwrap());
        drink.feed_from_db(datas);
        list_drk.push(drink);
    }
}

fn fetch_burgers(dao: &mut DAOCommand, command_id: u32) -> Vec<Burger>
{
    let query = "SELECT burger.*, commande_burger.quantite \
                      FROM commande_burger \
                      INNER JOIN burger \
                      ON commande_burger.burger_id = burger.id \
                      WHERE commande_burger.commande_id = ?";

    let result: Vec<Row> = dao.conn.exec(query, (command_id,)).unwrap();
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
        // adding the quantity of each burgers to the burger.quantity field
        burger.set_quantity(datas.6.unwrap());
        burger.feed_from_db(datas);
        burger.set_ingredients(fetch_ingredients(&mut DAOFactory::create_dao_burger(), burger.get_id()));
        list_bur.push(burger);
    }
}
