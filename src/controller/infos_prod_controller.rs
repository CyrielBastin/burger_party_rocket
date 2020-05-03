use rocket_contrib::templates::Template;
use rocket::http::RawStr;
use std::collections::HashMap;

//==================================================================================================
// All routes ares prefixed with /informations-produits
//==================================================================================================

#[get("/<product>")]
pub fn infos_produits(product: &RawStr) -> Template
{
    let context: HashMap<u8, u8> = HashMap::new();

    Template::render("infos_produits/infos_produits", context)
}
