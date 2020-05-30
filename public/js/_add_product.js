const id_burger = document.getElementById("id_burger")
const id_drink = document.getElementById("id_drink")
const list_ingr = document.querySelector(".list-ingr")
const btn_minus = document.querySelector(".btn-minus")
const btn_plus = document.querySelector(".btn-plus")
const inp_quantity = document.getElementById("bur_drk-qty")
const form = document.getElementById("form-qty")
const list_form_fields = document.querySelectorAll("#form-qty input")
const btn_send_qty = document.getElementById("send-qty")
let quantity = 0


if (id_burger)
{
    fetch(`/public/json-string/fetch/burger/${id_burger.innerText}`)
        .then(data => data.json())
        .then(burger => {
            quantity = burger.quantity
            inp_quantity.value = quantity
            return burger.ingredients
        })
        .then(ingredient => gather_img_and_qty_into_array(ingredient))
        .then(ingr_array => add_img_and_qty_to_DOM(ingr_array))
        .then(_ => add_event_listeners())
        .catch(error => console.log(error))
}
else
    // if we don't have `id` for burger, we have `id` for drink
{
    fetch(`/public/json-string/fetch/drink/${id_drink.innerText}`)
        .then(data => data.json())
        .then(drink => {
            quantity = drink.quantity
            inp_quantity.value = quantity
            return drink
        })
        .then(_ => add_event_listeners())
        .catch(error => console.log(error))
}

function add_event_listeners()
{
    btn_minus.addEventListener("click", function(e) {
        e.preventDefault()
        quantity--
        if (quantity < 0) quantity = 0
        inp_quantity.value = quantity
    })

    btn_plus.addEventListener("click", function(e) {
        e.preventDefault()
        quantity++
        if (quantity > 99) quantity = 99
        inp_quantity.value = quantity
    })

    btn_send_qty.addEventListener("click", function(e) {
        e.preventDefault()
        for (const field of list_form_fields)
        {
            field.disabled = false
        }
        form.submit()
    })
}

//====================================================================================================
// Functions
//====================================================================================================

function gather_img_and_qty_into_array (burger_array)
{
    const images = []
    let burger_top, burger_bottom = ""

    for (let i = 0; i < burger_array.length; i++) {
        if (i === 0) {
            const arr = burger_array[0].image.split("+")
            burger_top = arr[0]
            burger_bottom = arr[1]
            images.push({img: burger_top, qty: 1})
        } else {
            images.push({img: burger_array[i].image, qty: burger_array[i]['quantity']})
        }
    }
    images.push({img: burger_bottom, qty: 1})

    return images
}

function add_img_and_qty_to_DOM (ingr_array)
{
    for (const ingr of ingr_array) {
        const div_ingr_det = document.createElement("div")
        div_ingr_det.className = "ingr_det"
        const div_qty = document.createElement("div")
        div_qty.className = "ingr_qty"
        const qty_text = document.createTextNode(`${ingr.qty}x`)
        div_qty.appendChild(qty_text)

        const img_bur = document.createElement("img")
        img_bur.setAttribute("src", `/public/image/get/ingredients/${ingr.img}/svg`)
        img_bur.setAttribute("alt", "ingredient's image")
        img_bur.className = "list-ingr-img"

        div_ingr_det.appendChild(div_qty)
        div_ingr_det.appendChild(img_bur)

        list_ingr.appendChild(div_ingr_det)
    }
}
