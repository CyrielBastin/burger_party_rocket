/*
 * Here we fetch command details to add to the right navbar
 */

const burgers_container = document.querySelector(".bur-section");
const drinks_container = document.querySelector(".drk-section");


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
 * @param elements: Array<entity::Burger || entity::Drink>
 * @returns {void}
 */
function add_cmd_element_to_DOM (elements)
{
    const el_kind = elements['kind']
    for (const el of elements)
    {
        /*
         * create the div in which in each burger and drink will be in !
         */
        const div_item = document.createElement("div")
        div_item.className = "div-item"

        const div_qty = document.createElement("div")
        div_qty.className = "item-qty"
        const text_qty = document.createTextNode(el['quantity'] + " X")
        div_qty.appendChild(text_qty)

        const div_background = document.createElement("div")
        if (el_kind === "burger") div_background.className = "orange_background"
        else div_background.className = "background-blue"
        div_background.className += " slide-right-1"

        const t_image = document.createElement("img")
        t_image.src = `/public/image/get/${el_kind}s/${el['image']}/svg`
        t_image.className = "item_mini-img"
        div_background.appendChild(t_image)

        const div_name = document.createElement("div")
        div_name.className = "item_mini-name"
        const text_name = document.createTextNode(el['name'])
        div_name.appendChild(text_name)
        div_background.appendChild(div_name)

        div_item.appendChild(div_qty)
        div_item.appendChild(div_background)

        if (el_kind === "burger") burgers_container.appendChild(div_item)
        else drinks_container.appendChild(div_item)
    }
}
