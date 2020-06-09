const bur_list = document.querySelector(".bur-list")
const bur_cal_price = document.querySelector(".bur-cal_price")
const drk_list = document.querySelector(".drk-list")
const drk_cal_price = document.querySelector(".drk-cal_price")
const summary_cal = document.querySelector(".summary-cal .value-cal")
const summary_price = document.querySelector(".summary-price .value-price")
const bur_cmd_det = document.querySelector(".bur-cmd-details")
const drk_cmd_det = document.querySelector(".drk-cmd-details")

let total_calories = 0; let total_price = 0;

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
        //
        const calories_and_price = get_calories_and_price(burgers)
        add_bur_drk_details_to_DOM(calories_and_price, "burger")
    }
    else
    {
        bur_cmd_det.style.display = "none"
    }


    const resp_drinks = await fetch("/command/fetch/drinks")
    const drinks_data = await resp_drinks.text()
    if (drinks_data !== "")
    {
        const drinks = JSON.parse(drinks_data)
        drinks.kind = "drink"
        // console.table(drinks)
        add_cmd_element_to_DOM(drinks)
        //
        const calories_and_price = get_calories_and_price(drinks)
        add_bur_drk_details_to_DOM(calories_and_price, "drink")
    }
    else
    {
        drk_cmd_det.style.display = "none"
    }

    set_total_calories_and_price()
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
 * @param {object<{calories: number, price: number}>} cal_price_obj
 * @param {string} kind
 */
function add_bur_drk_details_to_DOM (cal_price_obj, kind)
{
    const text_cal = document.createTextNode(`calories : ${cal_price_obj.calories} Kcal,`)
    const span = document.createElement("span")
    span.className = "inner-margin"
    const text_price = document.createTextNode(`prix : ${cal_price_obj.price} €`)
    if (kind === "burger") {
        bur_cal_price.appendChild(text_cal)
        bur_cal_price.appendChild(span)
        bur_cal_price.appendChild(text_price)
    } else {
        drk_cal_price.appendChild(text_cal)
        drk_cal_price.appendChild(span)
        drk_cal_price.appendChild(text_price)
    }
}

/**
 * @param {Array<entity::Burger | entity::Drink>} elements
 * @returns {object<{calories: number, price: number}>}
 */
function get_calories_and_price (elements)
{
    const el_kind = elements['kind']
    let _calories = 0; let _price = 0

    for (const el of elements)
    {
        _price += (el['price'] * el['quantity'])
        if (el_kind === "drink") {
            _calories += (el['calories'] * el['quantity'])
        }
        else {
            let bur_cal = 0
            for (const ingr of el['ingredients']) {
                bur_cal += (ingr['calories'] * ingr['quantity'])
            }
            bur_cal *= el['quantity']
            _calories += bur_cal
        }
    }
    total_calories += _calories
    total_price += _price

    return {
        calories: _calories,
        price: _price
    }
}

function set_total_calories_and_price ()
{
    summary_cal.insertAdjacentText("afterbegin", `${total_calories} KCal`)
    summary_price.insertAdjacentText("afterbegin", `${total_price} €`)
}

