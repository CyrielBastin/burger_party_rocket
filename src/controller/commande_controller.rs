use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;

//==================================================================================================
// All routes ares prefixed with /commande
//==================================================================================================

#[get("/new")]
pub fn commande_new() -> Template
{
    let context = Context::new();

    Template::render("commande/commande_new", context)
}
