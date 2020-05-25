/*
 * Used to retrieve, and setup command details from `public/command_details{burger/boisson}.txt`
 * to further convert them to JSON.
 */

use crate::controller::commande_controller::CmdQte;
use crate::data_access::{DAOFactory, DAO};
use crate::entity::{Burger, Boisson};
use std::fs::File;
use std::io::{Read, Write};



pub fn write_cmd_details(cmd_details: &CmdQte)
{
    if cmd_details.quantite == 0 { return }

    if cmd_details.kind == "burger"
    {
        let burger = find_burger_and_update_qte(&cmd_details, cmd_details.id);
        add_burger_to_file(burger).expect("Alea Problem es");

    }
    else // cmd_details.kind == "boisson" at this point
    {
        let boisson = find_boisson_and_update_qte(&cmd_details, cmd_details.id);
    }
}

fn find_burger_and_update_qte(cmd_det: &CmdQte, id: u32) -> Burger
{
    let mut repo = DAOFactory::create_dao_burger();
    let mut burger = repo.find_by_id(id);
    burger.set_quantite(cmd_det.quantite);

    burger
}

fn find_boisson_and_update_qte(cmd_det: &CmdQte, id: u32) -> Boisson
{
    let mut repo = DAOFactory::create_dao_boisson();
    let mut boisson = repo.find_by_id(id);
    boisson.set_quantite(cmd_det.quantite);

    boisson
}

fn add_burger_to_file(burger: Burger) -> std::io::Result<()>
{
    let mut file = File::open("public/command_details_burger.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("{}", content);

    if content.is_empty()
    {
        let mut file = File::create("public/command_details_burger.txt")?;
        write!(file, "[{}]", burger.to_json_string())?;
    }
    else {
        let mut list_burgers: Vec<Burger> = serde_json::from_str(&content).unwrap();
        let mut found_element = false;
        for b in &mut list_burgers
        {
            if b.get_id() == burger.get_id() {
                b.set_quantite(b.get_quantite() + burger.get_quantite());
                found_element = true;
            }
        }
        if !found_element {
            list_burgers.push(burger);
        }
        write!(file, "{:#?}", list_burgers)?;
    }

    Ok(())
}
