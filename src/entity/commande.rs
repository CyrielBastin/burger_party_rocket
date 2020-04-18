use super::{Commande, Burger, Boisson};
use std::fmt::{Display, Formatter, Result};
use chrono::Local;
use regex::Regex;

impl Commande
{
    pub fn new() -> Self
    {
        Commande {
            id: 0,
            terminal: 0,
            heure_complete: "".to_string(),
            paye: false,
            burgers: Vec::new(),
            boissons: Vec::new()
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

    pub fn get_heure(&self) -> &str {
        &self.heure_complete[..]
    }
    pub fn set_heure(&mut self, heure: &str) {
        self.heure_complete = heure.to_string();
    }

    pub fn get_paye(&self) -> bool {
        self.paye
    }
    pub fn set_paye(&mut self, paye: bool) {
        self.paye = paye;
    }

    pub fn get_burgers(&self) -> &Vec<Burger> {
        &self.burgers
    }
    pub fn set_burgers(&mut self, burgers: Vec<Burger>) {
        self.burgers = burgers;
    }

    pub fn get_boissons(&self) -> &Vec<Boisson> {
        &self.boissons
    }
    pub fn set_boissons(&mut self, boissons: Vec<Boisson>) {
        self.boissons = boissons;
    }
}

impl Display for Commande
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        write!(f, "Commande {{\n\tid: {},\n\tterminal: {},\n\theure: {},\n\t\
                paye: {}\n}}",
               self.get_id(), self.get_terminal(), self.get_heure(), self.get_paye())
    }
}

impl Commande {
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
}
