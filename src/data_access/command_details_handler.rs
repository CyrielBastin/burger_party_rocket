/*
 * Used to retrieve, and setup command details from `public/command_details/...txt`
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
        add_burger_to_file(burger).unwrap();
    }
    else // cmd_details.kind == "boisson" at this point
    {
        let boisson = find_boisson_and_update_qte(&cmd_details, cmd_details.id);
        add_boisson_to_file(boisson).unwrap();
    }
}

/*
 * This method empties the content of the files related to command_details in order
 * to reset the command
 */
pub fn empty_command_details_content() -> std::io::Result<()>
{
    let _file = File::create("public/command_details/details_boisson.txt")?;
    let _file = File::create("public/command_details/details_burger.txt")?;

    Ok(())
}

//==================================================================================================
// Burger's section
//==================================================================================================

fn find_burger_and_update_qte(cmd_det: &CmdQte, id: u32) -> Burger
{
    let mut repo = DAOFactory::create_dao_burger();
    let mut burger = repo.find_by_id(id);
    burger.set_quantite(cmd_det.quantite);

    burger
}

fn add_burger_to_file(burger: Burger) -> std::io::Result<()>
{
    let file_path = "public/command_details/details_burger.txt";
    let mut file = File::open(file_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    if content.is_empty()
    {
        let mut file = File::create(file_path)?;
        write!(file, "[{}]", serde_json::to_string_pretty(&burger)?)?;
    }
    else {
        let list_burgers = get_current_burger_file_content_and_update_qte(burger, content);
        write_new_burger_content_to_file(list_burgers, file_path).unwrap();
    }

    Ok(())
}

fn get_current_burger_file_content_and_update_qte(burger: Burger, file_content: String) -> Vec<Burger>
{
    let mut list_burgers: Vec<Burger> = serde_json::from_str(&file_content).unwrap();
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

    list_burgers
}

fn write_new_burger_content_to_file(list_burgers: Vec<Burger>, file_path: &str) -> std::io::Result<()>
{
    let new_content = serde_json::to_string_pretty(&list_burgers).unwrap();
    let mut file = File::create(file_path)?;
    write!(file, "{}", new_content)?;

    Ok(())
}

//==================================================================================================
// Drink's section
//==================================================================================================

fn find_boisson_and_update_qte(cmd_det: &CmdQte, id: u32) -> Boisson
{
    let mut repo = DAOFactory::create_dao_boisson();
    let mut boisson = repo.find_by_id(id);
    boisson.set_quantite(cmd_det.quantite);

    boisson
}

fn add_boisson_to_file(boisson: Boisson) -> std::io::Result<()>
{
    let file_path = "public/command_details/details_boisson.txt";
    let mut file = File::open(file_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    if content.is_empty()
    {
        let mut file = File::create(file_path)?;
        write!(file, "[{}]", serde_json::to_string_pretty(&boisson)?)?;
    }
    else {
        let list_boissons = get_current_boisson_file_content_and_update_qte(boisson, content);
        write_new_boisson_content_to_file(list_boissons, file_path).unwrap();
    }

    Ok(())
}

fn get_current_boisson_file_content_and_update_qte(boisson: Boisson, file_content: String) -> Vec<Boisson>
{
    let mut list_boissons: Vec<Boisson> = serde_json::from_str(&file_content).unwrap();
    let mut found_element = false;
    for b in &mut list_boissons
    {
        if b.get_id() == boisson.get_id() {
            b.set_quantite(b.get_quantite() + boisson.get_quantite());
            found_element = true;
        }
    }
    if !found_element {
        list_boissons.push(boisson);
    }

    list_boissons
}

fn write_new_boisson_content_to_file(list_boissons: Vec<Boisson>, file_path: &str) -> std::io::Result<()>
{
    let new_content = serde_json::to_string_pretty(&list_boissons).unwrap();
    let mut file = File::create(file_path)?;
    write!(file, "{}", new_content)?;

    Ok(())
}
