/*
 * Script file for the details of the command
 * before validating it and let the customer pays
 */

const bur_list = document.querySelector(".bur-list");
const bur_cal_price = document.querySelector(".bur-cal_price");
const drk_list = document.querySelector(".drk-list");
const drk_cal_price = document.querySelector(".drk-cal_price");


(async function ()
{
    const resp_burgers = await fetch("/command/fetch/burgers")
    const burgers_data = await resp_burgers.text()
    if (burgers_data !== "")
    {
        const burgers = JSON.parse(burgers_data)
        burgers.kind = "burger"
        // console.table(burgers)
        add_cmd_element_to_DOM(burgers)
    }


    const resp_drinks = await fetch("/command/fetch/drinks")
    const drinks_data = await resp_drinks.text()
    if (drinks_data !== "")
    {
        const drinks = JSON.parse(drinks_data)
        drinks.kind = "drink"
        // console.table(drinks)
        add_cmd_element_to_DOM(drinks)
    }
})()


/**
 *
 * @param {Array<entity::Burger | entity::Drink>} elements
 * @returns {void}
 */
function add_cmd_element_to_DOM (elements)
{
    const el_kind = elements['kind']
    for (const el of elements)
    {
        /*
         * create the div in which in each burgers and drinks will be in !
         */
        const div_item_details = document.createElement("div")
        div_item_details.className = "item-details"

        const div_qty = document.createElement("div")
        div_qty.className = "details-item_qty"
        const text_qty = document.createTextNode(el['quantity'])
        div_qty.appendChild(text_qty)

        const div_img_name = document.createElement("div")
        div_img_name.className = "div_img_name"
        const t_image = document.createElement("img")
        t_image.src = `/public/image/get/${el_kind}s/${el['image']}/svg`
        t_image.className = "details-item_img"

        const div_name = document.createElement("div")
        div_name.className = "details-item_name"
        const text_name = document.createTextNode(el['name'])
        div_name.appendChild(text_name)

        div_img_name.appendChild(t_image)
        div_img_name.appendChild(div_name)

        div_item_details.appendChild(div_qty)
        div_item_details.appendChild(div_img_name)

        if (el_kind === "burger") bur_list.appendChild(div_item_details)
        else drk_list.appendChild(div_item_details)
    }
}

/**
 * @param
 * @returns {object<{calories: number, price: number}>}
 */
function get_calories_and_price ()
{
}

