const cmd_datetime = document.getElementById("datetime").textContent;
const div_cmd_datetime = document.querySelector(".cmd-datetime");
const div_cmd_terminal = document.querySelector(".cmd-terminal");
const section_cmd_burgers = document.querySelector(".cmd-burgers");
const section_cmd_drinks = document.querySelector(".cmd-drinks");
const cmd_total_price = document.querySelector(".cmd-total .items-price");


(async function()
{
    const resp_cmd = await fetch(`/command/fetch/command/${cmd_datetime}`)
    const cmd_json = await resp_cmd.json()

    add_header_to_DOM(cmd_json['date_time'], cmd_json['terminal']).then(_ => _)
    add_content_to_DOM(cmd_json['burgers'], "burger").then(_ => _)
    add_content_to_DOM(cmd_json['drinks'], "drink").then(_ => _)
    compute_total_price(cmd_json).then(price => cmd_total_price.insertAdjacentText("afterbegin", `${price} €`))
})()


async function add_header_to_DOM(datetime, terminal)
{
    convert_datetime_to_french_format(datetime)
        .then(new_datetime => div_cmd_datetime.insertAdjacentText("beforeend", new_datetime))
    div_cmd_terminal.insertAdjacentText("beforeend", terminal)
}


/*
 * Convert from "2020-06-09 19:21:50"
 * to "Le 09/06/2020, à 19:21:50"
 */
async function convert_datetime_to_french_format(datetime)
{
    let _datetime_split = datetime.split(' ')
    let _date = _datetime_split[0]
    let _time = _datetime_split[1]

    let _date_split = _date.split('-')
    let _day = _date_split[2]
    let _month = _date_split[1]
    let _year = _date_split[0]

    const french_date = `${_day}/${_month}/${_year}`

    return `Le ${french_date}, à ${_time}`
}


/**
 *
 * @param {Array<>} elements
 * @param {string} type
 * @returns {Promise<void>}
 */
async function add_content_to_DOM(elements, type)
{
    if (elements.length === 0)
    {
        switch (type)
        {
            case "burger": {
                section_cmd_burgers.style.display = "none"
                break
            }
            case "drink": {
                section_cmd_drinks.style.display = "none"
                break
            }
            default: break
        }
    }
    else
    {
        for (const el of elements)
        {
            const div_container = document.createElement("div")
            div_container.className = "item-details"

            const div_name = document.createElement("div")
            div_name.className = "item-name"
            div_name.insertAdjacentText("afterbegin", `${el['name']}`)
            const div_qty = document.createElement("div")
            div_qty.className = "item-qty"
            div_qty.insertAdjacentText("afterbegin", `${el['quantity']} x`)
            const div_price = document.createElement("div")
            div_price.className = "item-price"
            div_price.insertAdjacentText("afterbegin", `${el['quantity'] * el['price']} €`)

            div_container.appendChild(div_qty)
            div_container.appendChild(div_name)
            div_container.appendChild(div_price)

            if (type === "burger") {
                section_cmd_burgers.appendChild(div_container)
            } else {
                section_cmd_drinks.appendChild(div_container)
            }
        }
    }
}

async function compute_total_price(command)
{
    let total_price = 0

    if (command['burgers'].length !== 0)
    {
        command['burgers'].forEach(function (burger) {
            total_price += (burger['quantity'] * burger['price'])
        })
    }

    if (command['drinks'].length !== 0)
    {
        command['drinks'].forEach(function (drink) {
            total_price += (drink['quantity'] * drink['price'])
        })
    }

    return total_price
}
