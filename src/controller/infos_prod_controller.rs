use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;
use rocket::http::RawStr;

//==================================================================================================
// All routes ares prefixed with /informations-produits
//==================================================================================================

#[get("/<product>")]
pub fn infos_produits(product: &RawStr) -> Template
{
    let context = Context::new();

    Template::render("infos_produits/infos_produits", context)
}
