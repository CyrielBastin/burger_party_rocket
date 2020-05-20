use rocket::response::Redirect;
use rocket::request::{Form, FromForm};

/*
 * Handle all requests that will setup Cookies
 */

//==================================================================================================
// All routes ares prefixed with /cookie
//==================================================================================================

#[derive(FromForm, Debug)]
pub struct CmdQte
{
    kind: String,
    id: u32,
    quantite: u8
}

#[post("/set", data = "<cmd_det>")]
pub fn set_cookie(cmd_det: Form<CmdQte>) -> Redirect
{
    println!("{:#?}", cmd_det);

    Redirect::to(uri!(super::homepage_controller::index))
}
