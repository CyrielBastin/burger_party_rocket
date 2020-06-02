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
        const div_qty = document.createElement("div")
        div_qty.className = "item-qty"
        const text_qty = document.createTextNode(el['quantity'])
        div_qty.appendChild(text_qty)

        const t_image = document.createElement("img")
        t_image.src = `/public/image/get/${el_kind}s/${el['image']}/svg`
        t_image.className = "item_mini-img"

        const div_name = document.createElement("div")
        div_name.className = "item_mini-name"
        const text_name = document.createTextNode(el['name'])
        div_name.appendChild(text_name)

        if (el_kind === "burger")
        {
            burgers_container.appendChild(div_qty)
            burgers_container.appendChild(t_image)
            burgers_container.appendChild(div_name)
        }
        else
        {
            drinks_container.appendChild(div_qty)
            drinks_container.appendChild(t_image)
            drinks_container.appendChild(div_name)
        }
    }
}
