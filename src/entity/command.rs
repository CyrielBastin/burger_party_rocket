use super::{Command, Burger, Drink};
use crate::types::db_types::CmdFromDb;
use chrono::Local;
use regex::Regex;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
impl Command
{
    pub fn new() -> Self
    {
        Command {
            id: 0,
            terminal: 0,
            date_time: "".to_string(),
            payed: false,
            burgers: Vec::new(),
            drinks: Vec::new()
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn get_terminal(&self) -> u8 {
        self.terminal
    }
    pub fn set_terminal(&mut self, terminal: u8) {
        self.terminal = terminal;
    }

    pub fn get_date_time(&self) -> &str {
        &self.date_time[..]
    }
    pub fn set_date_time(&mut self, date_time: &str) {
        self.date_time = date_time.to_string();
    }

    pub fn get_payed(&self) -> bool {
        self.payed
    }
    pub fn set_payed(&mut self, payed: bool) {
        self.payed = payed;
    }

    pub fn get_burgers(&self) -> &Vec<Burger> {
        &self.burgers
    }
    pub fn set_burgers(&mut self, burgers: Vec<Burger>) {
        self.burgers = burgers;
    }

    pub fn get_drinks(&self) -> &Vec<Drink> {
        &self.drinks
    }
    pub fn set_drinks(&mut self, drinks: Vec<Drink>) {
        self.drinks = drinks;
    }
}

#[allow(dead_code)]
impl Command {
    pub fn feed_from_db(&mut self, datas: CmdFromDb)
    {
        self.set_id(match datas.0 {
            Some(x) => x,
            None => 0
        });
        self.set_date_time(match &datas.2 {
            Some(x) => x,
            None => ""
        });
        self.set_terminal(match datas.3 {
            Some(x) => x,
            None => 0
        });
        self.set_payed(match datas.4 {
            Some(x) => if x == 0 {false} else {true},
            None => false
        });
    }

    pub fn append_burgers_and_drinks(&mut self)
    {
        self.set_burgers(append_burgers());
        self.set_drinks(append_drinks());
    }
}

fn append_burgers() -> Vec<Burger>
{
    let file_path = "public/command_details/details_burgers.json";
    let mut file = File::open(file_path).unwrap();

    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    let mut list_burgers = Vec::new();
    if !file_content.is_empty()
    {
        list_burgers = serde_json::from_str(&file_content).unwrap();
    }

    list_burgers
}
fn append_drinks() -> Vec<Drink>
{
    let file_path = "public/command_details/details_drinks.json";
    let mut file = File::open(file_path).unwrap();

    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    let mut list_drinks = Vec::new();
    if !file_content.is_empty()
    {
        list_drinks = serde_json::from_str(&file_content).unwrap();
    }

    list_drinks
}

pub fn get_local_to_string() -> String
{
    let local = Local::now();
    let text = local.to_string();
    let re = Regex::new(r"(^.*)\.(.*$)").unwrap();
    let mut result = String::from("");
    // captures the DateTime of the string up until milliseconds
    // in order to push it to the Database
    result.push_str(&re.captures(&text).unwrap()[1]);

    result
}
