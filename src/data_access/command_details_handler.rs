/*
 * Used to retrieve, and setup command details from `public/command_details/...json`
 * to further convert them to JSON.
 *
 * =================================================================================================
 *
 * It's working BUT it's GARBAGE
 * Will refactor this file and DAOFactory with <Generics> IF I have time
 * First focus on finishing the minimum requirements for the project !
 */

use crate::controller::command_controller::CmdQty;
use crate::data_access::{DAOFactory, DAO};
use crate::entity::{Burger, Drink};
use std::fs::File;
use std::io::{Read, Write};



pub fn write_cmd_details(cmd_details: &CmdQty)
{
    if cmd_details.quantity == 0
    {
        if cmd_details.kind == "burger" {
            remove_burger(cmd_details.id);
        }
        else {
            remove_drink(cmd_details.id);
        }

        return;
    }

    if cmd_details.kind == "burger"
    {
        let burger = find_burger_and_update_qty(&cmd_details, cmd_details.id);
        add_burger_to_file(burger).unwrap();
    }
    else // cmd_details.kind == "drink" at this point
    {
        let drink = find_drink_and_update_qty(&cmd_details, cmd_details.id);
        add_drink_to_file(drink).unwrap();
    }
}

fn remove_burger(id: u32)
{
    let file_path = "public/command_details/details_burgers.json";
    let mut file = File::open(file_path).unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    if file_content.is_empty() {
        return;
    }
    let list_burgers: Vec<Burger> = serde_json::from_str(&file_content).unwrap();
    let mut new_list_burgers = Vec::new();

    for b in list_burgers
    {
        if !(b.get_id() == id) {
            new_list_burgers.push(b);
        }
    }
    let mut file = File::create(file_path).unwrap();
    if !new_list_burgers.is_empty() {
        write!(file, "{}", serde_json::to_string_pretty(&new_list_burgers).unwrap()).unwrap();
    }
}

fn remove_drink(id: u32)
{
    let file_path = "public/command_details/details_drinks.json";
    let mut file = File::open(file_path).unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    if file_content.is_empty() {
        return;
    }
    let list_drinks: Vec<Drink> = serde_json::from_str(&file_content).unwrap();
    let mut new_list_drinks = Vec::new();

    for b in list_drinks
    {
        if !(b.get_id() == id) {
            new_list_drinks.push(b);
        }
    }
    let mut file = File::create(file_path).unwrap();
    if !new_list_drinks.is_empty() {
        write!(file, "{}", serde_json::to_string_pretty(&new_list_drinks).unwrap()).unwrap();
    }
}

/*
 * This method empties the content of the files related to command_details in order
 * to reset the command
 */
pub fn empty_command_details_content() -> std::io::Result<()>
{
    let _file = File::create("public/command_details/details_burgers.json")?;
    let _file = File::create("public/command_details/details_drinks.json")?;

    Ok(())
}

//==================================================================================================
// Burger's section
//==================================================================================================

fn find_burger_and_update_qty(cmd_det: &CmdQty, id: u32) -> Burger
{
    let mut repo = DAOFactory::create_dao_burger();
    let mut burger = repo.find_by_id(id);
    burger.set_quantity(cmd_det.quantity);

    burger
}

fn add_burger_to_file(burger: Burger) -> std::io::Result<()>
{
    let file_path = "public/command_details/details_burgers.json";
    let mut file = File::open(file_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    if content.is_empty()
    {
        let mut file = File::create(file_path)?;
        write!(file, "[{}]", serde_json::to_string_pretty(&burger)?)?;
    }
    else {
        let list_burgers = get_current_burger_file_content_and_update_qty(burger, content);
        write_new_burger_content_to_file(list_burgers, file_path).unwrap();
    }

    Ok(())
}

fn get_current_burger_file_content_and_update_qty(burger: Burger, file_content: String) -> Vec<Burger>
{
    let mut list_burgers: Vec<Burger> = serde_json::from_str(&file_content).unwrap();
    let mut found_element = false;
    for b in &mut list_burgers
    {
        if b.get_id() == burger.get_id() {
            b.set_quantity(burger.get_quantity());
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

fn find_drink_and_update_qty(cmd_det: &CmdQty, id: u32) -> Drink
{
    let mut repo = DAOFactory::create_dao_drink();
    let mut drink = repo.find_by_id(id);
    drink.set_quantity(cmd_det.quantity);

    drink
}

fn add_drink_to_file(drink: Drink) -> std::io::Result<()>
{
    let file_path = "public/command_details/details_drinks.json";
    let mut file = File::open(file_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    if content.is_empty()
    {
        let mut file = File::create(file_path)?;
        write!(file, "[{}]", serde_json::to_string_pretty(&drink)?)?;
    }
    else {
        let list_drinks = get_current_drink_file_content_and_update_qty(drink, content);
        write_new_drink_content_to_file(list_drinks, file_path).unwrap();
    }

    Ok(())
}

fn get_current_drink_file_content_and_update_qty(drink: Drink, file_content: String) -> Vec<Drink>
{
    let mut list_drinks: Vec<Drink> = serde_json::from_str(&file_content).unwrap();
    let mut found_element = false;
    for b in &mut list_drinks
    {
        if b.get_id() == drink.get_id() {
            b.set_quantity(drink.get_quantity());
            found_element = true;
        }
    }
    if !found_element {
        list_drinks.push(drink);
    }

    list_drinks
}

fn write_new_drink_content_to_file(list_drinks: Vec<Drink>, file_path: &str) -> std::io::Result<()>
{
    let new_content = serde_json::to_string_pretty(&list_drinks).unwrap();
    let mut file = File::create(file_path)?;
    write!(file, "{}", new_content)?;

    Ok(())
}

//==================================================================================================
// Section for retrieving burgers and drinks of the command and send them to the controller
//==================================================================================================

pub fn fetch_cmd_burgers() -> Option<String>
{
    let file_path = "public/command_details/details_burgers.json";
    let mut file = File::open(file_path).unwrap();

    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    Some(file_content)
}

pub fn fetch_cmd_drinks() -> Option<String>
{
    let file_path = "public/command_details/details_drinks.json";
    let mut file = File::open(file_path).unwrap();

    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    Some(file_content)
}

pub fn fetch_cmd_by_datetime(datetime: &str) -> Option<String>
{
    let mut dao_cmd = DAOFactory::create_dao_command();
    let cmd_id = dao_cmd.get_id_from_datetime(datetime);
    let cmd = dao_cmd.find_by_id(cmd_id);

    serde_json::to_string(&cmd).ok()
}
