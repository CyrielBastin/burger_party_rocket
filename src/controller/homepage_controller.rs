use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/")]
pub fn index() -> Template {
    let context: HashMap<u8, u8> = HashMap::new();
    Template::render("index", context)
}
