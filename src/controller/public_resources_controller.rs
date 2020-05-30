use rocket::http::RawStr;
use rocket::response::NamedFile;
use serde_json;
use crate::data_access::{DAOFactory, DAO};
use crate::entity::{Burger, Drink};
use std::io;
use std::io::Read;
use std::fs::File;


//==================================================================================================
// All routes ares prefixed with /public
//==================================================================================================

#[get("/css/get/<file_name>")]
pub fn get_css(file_name: &RawStr) -> io::Result<NamedFile>
{
    let file_path = format!("./public/css/{}.css", file_name);

    NamedFile::open(file_path)
}

#[get("/font/get/<font_name>/<ext>")]
pub fn get_font(font_name: &RawStr, ext: &RawStr) -> io::Result<NamedFile>
{
    let file_path = format!("./public/fonts/{}.{}", font_name, ext);

    NamedFile::open(file_path)
}

#[get("/image/get/<kind>/<img_name>/<ext>")]
pub fn get_image(kind: &RawStr, img_name: &RawStr, ext: &RawStr) -> io::Result<NamedFile>
{
    let file_path = format!("./public/images/{}/{}.{}", kind, img_name, ext);

    NamedFile::open(file_path)
}

#[get("/js/get/<file_name>")]
pub fn get_js(file_name: &RawStr) -> io::Result<NamedFile>
{
    let file_path = format!("./public/js/{}.js", file_name);

    NamedFile::open(file_path)
}

#[get("/json-string/fetch/burger/<burger_id>")]
pub fn get_burger(burger_id: u32) -> Option<String>
{
    let mut burger_repo = DAOFactory::create_dao_burger();
    let mut burger = burger_repo.find_by_id(burger_id);

    let mut file = File::open("public/command_details/details_burgers.json").unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    if !file_content.is_empty()
    {
        let mut list_burgers: Vec<Burger> = serde_json::from_str(&file_content).unwrap();
        for b in &mut list_burgers
        {
            if b.get_id() == burger_id {
                burger.set_quantity(b.get_quantity());
            }
        }
    }

    serde_json::to_string(&burger).ok()
}

#[get("/json-string/fetch/drink/<drink_id>")]
pub fn get_drink(drink_id: u32) -> Option<String>
{
    let mut drink_repo = DAOFactory::create_dao_drink();
    let mut drink = drink_repo.find_by_id(drink_id);

    let mut file = File::open("public/command_details/details_drinks.json").unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    if !file_content.is_empty()
    {
        let mut list_drinks: Vec<Drink> = serde_json::from_str(&file_content).unwrap();
        for b in &mut list_drinks
        {
            if b.get_id() == drink_id {
                drink.set_quantity(b.get_quantity());
            }
        }
    }

    serde_json::to_string(&drink).ok()
}
