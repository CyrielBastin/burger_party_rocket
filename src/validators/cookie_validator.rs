use crate::controller::cookie_controller::CmdQte;
use crate::data_access::DAOFactory;

pub fn are_datas_valid (obj: &CmdQte) -> bool
{
    is_valid_kind_and_id(&obj.kind, obj.id) && is_valid_qty(obj.quantite)
}

fn is_valid_kind_and_id (kind: &str, id: u32) -> bool
{
    match kind
    {
        "burger" => is_valid_burger_id(id),
        "boisson" => is_valid_drink_id(id),
        _ => false
    }
}

fn is_valid_burger_id (id: u32) -> bool
{
    let mut is_valid = false;
    let mut bur_repo = DAOFactory::create_dao_burger();
    let list_ids = bur_repo.find_id_all();

    for id_in_list in list_ids
    {
        if id == id_in_list
        {
            is_valid = true;
        }
    }

    is_valid
}

fn is_valid_drink_id (id: u32) -> bool
{
    let mut is_valid = false;
    let mut drk_repo = DAOFactory::create_dao_boisson();
    let list_ids = drk_repo.find_id_all();

    for id_in_list in list_ids
    {
        if id == id_in_list
        {
            is_valid = true;
        }
    }

    is_valid
}

fn is_valid_qty (qte: u8) -> bool
{
    let mut is_valid = true;
    if qte > 99
    {
        is_valid = false;
    }

    is_valid
}
