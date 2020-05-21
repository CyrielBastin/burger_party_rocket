use rocket::response::Redirect;
use rocket::request::{Form, FromForm};
use crate::validators::cookie_validator::are_datas_valid;
use crate::cookie_handler::c_commande::add_cookies;

/*
 * Handles all requests that will setup Cookies
 */

//==================================================================================================
// All routes ares prefixed with /cookie
//==================================================================================================

/*
 * Structure to receive the kind (burger or drink), its `id` and `quantity` to add to the command
 */
#[derive(FromForm, Debug)]
pub struct CmdQte
{
    pub kind: String,
    pub id: u32,
    pub quantite: u8
}

#[post("/set", data = "<cmd_det>")]
pub fn set_cookie(cmd_det: Form<CmdQte>) -> Redirect
{
    if are_datas_valid(&cmd_det)
    {
        add_cookies();
    }

    Redirect::to(uri!(super::homepage_controller::index))
}
