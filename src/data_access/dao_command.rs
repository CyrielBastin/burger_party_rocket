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
        if _obj.get_burgers().is_empty() && _obj.get_drinks().is_empty() {
            return false;
        }

        let first_query = "INSERT INTO `commande` (n_table, n_serveur, paye, heure) \
                                 VALUES (1, ?, ?, ?)";

        let _: () = self.conn.exec_drop(first_query, (1, 1, _obj.get_date_time())).unwrap();

        let command_id = self.get_id_from_datetime(_obj.get_date_time());
        let command_burgers = _obj.get_burgers();
        let command_drinks = _obj.get_drinks();

        push_cmd_burgers_to_db(self, command_id, command_burgers);
        push_cmd_drinks_to_db(self, command_id, command_drinks);

        true
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

fn push_cmd_burgers_to_db(dao: &mut DAOCommand, id: u32, vec_burgers: &Vec<Burger>)
{
    if vec_burgers.is_empty() { return; }

    let query = "INSERT INTO `commande_burger` VALUES (?, ?, ?)";
    for burger in vec_burgers {
        let _ = dao.conn.exec_drop(query, (id, burger.get_id(), burger.get_quantity())).unwrap();
    }
}

fn push_cmd_drinks_to_db(dao: &mut DAOCommand, id: u32, vec_drinks: &Vec<Drink>)
{
    if vec_drinks.is_empty() { return; }

    let query = "INSERT INTO `commande_boisson` VALUES (?, ?, ?)";
    for drink in vec_drinks {
        let _ = dao.conn.exec_drop(query, (id, drink.get_id(), drink.get_quantity())).unwrap();
    }
}
