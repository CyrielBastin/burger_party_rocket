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

    console.log(cmd_json)

    div_cmd_datetime.insertAdjacentText("beforeend", cmd_json['date_time'])
    div_cmd_terminal.insertAdjacentText("beforeend", cmd_json['terminal'])

    add_content_to_DOM(cmd_json['burgers'], "burger").then(_ => _)
    add_content_to_DOM(cmd_json['drinks'], "drink").then(_ => _)
    compute_total_price(cmd_json).then(price => cmd_total_price.insertAdjacentText("afterbegin", `${price} €`))
})()

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
