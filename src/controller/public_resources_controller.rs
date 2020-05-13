use std::io;
use rocket::http::RawStr;
use rocket::response::NamedFile;
use serde_json;
use crate::data_access::{DAOFactory, DAO};

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

#[get("/from-json/fetch/ingredients-for-burger/<burger_id>")]
pub fn get_ingredients(burger_id: u32) -> Option<String>
{
    let mut burger_repo = DAOFactory::create_dao_burger();
    let burger = burger_repo.find_by_id(burger_id);
    let ingredients = burger.get_ingredients();

    serde_json::to_string(ingredients).ok()
}
