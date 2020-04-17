use crate::data_access::{DAO, DAOBurger};
use crate::entity::{Burger, Ingredient};
use mysql::{Row, from_row};
use mysql::prelude::Queryable;

type DataFromDb = (Option<u32>, Option<String>, Option<f32>, Option<String>,
                   Option<String>, Option<String>);

type IngrFromDb = (Option<u32>, Option<String>, Option<String>, Option<f32>,
                   Option<u16>, Option<u8>, Option<u8>, Option<String>, Option<u8>);

impl DAO<Burger> for DAOBurger
{
    fn create(&self, _obj: Burger) -> bool {
        false
    }

    fn update(&self, _obj: Burger) -> bool {
        false
    }

    fn delete(&self, _obj: Burger) -> bool {
        false
    }

    fn find_by_id(&mut self, id: u32) -> Burger
    {
        let query = "SELECT * FROM `burger` WHERE `id` = ?";
        let result: Row = self.conn.exec_first(query, (id,)).unwrap().unwrap();
        let datas = from_row::<DataFromDb>(result);
        let mut burger = Burger::new();
        burger.feed_from_db(datas);
        burger.set_ingredients(fetch_ingredients(self, id));

        burger
    }

    fn find_all(&mut self) -> Vec<Burger>
    {
        unimplemented!()
    }
}

pub fn fetch_ingredients(dao: &mut DAOBurger, burger_id: u32) -> Vec<Ingredient>
{
    let query =
    "SELECT ingredient.*, burger_ingredient.quantite \
    FROM `burger_ingredient` INNER JOIN `ingredient` \
        ON burger_ingredient.ingredient_id = ingredient.id \
    WHERE burger_ingredient.burger_id = ?";
    let result: Vec<Row> = dao.conn.exec(query, (burger_id,)).unwrap();
    let mut list_ingredients = Vec::new();
    push_row_to_vec_ingr(result, &mut list_ingredients);

    list_ingredients
}

fn push_row_to_vec_ingr(result: Vec<Row>, list_ingr: &mut Vec<Ingredient>)
{
    for row in result
    {
        let datas = from_row::<IngrFromDb>(row);
        let mut ingredient = Ingredient::new();
        ingredient.feed_from_db(datas);
        list_ingr.push(ingredient);
    }
}
