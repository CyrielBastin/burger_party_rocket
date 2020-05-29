const id_burger = document.getElementById("id_burger")
const id_boisson = document.getElementById("id_boisson")
const list_ingr = document.querySelector(".list-ingr")
const btn_minus = document.querySelector(".btn-minus")
const btn_plus = document.querySelector(".btn-plus")
const inp_quantite = document.getElementById("bur_drk-qte")
const form = document.getElementById("form-qte")
const list_form_fields = document.querySelectorAll("#form-qte input")
const btn_send_qte = document.getElementById("send-qte")
let quantite = 0


if (id_burger)
{
    fetch(`/public/json-string/fetch/burger/${id_burger.innerText}`)
        .then(data => data.json())
        .then(burger => {
            quantite = burger.quantite
            inp_quantite.value = quantite
            return burger.ingredients
        })
        .then(ingredient => gather_img_and_qte_into_array(ingredient))
        .then(ingr_array => add_img_and_qte_to_DOM(ingr_array))
        .then(_ => add_event_listeners())
        .catch(error => console.log(error))
}
else
    // if we don't have `id` for burger, we have `id` for boisson
{
    fetch(`/public/json-string/fetch/boisson/${id_boisson.innerText}`)
        .then(data => data.json())
        .then(boisson => {
            quantite = boisson.quantite
            inp_quantite.value = quantite
            return boisson
        })
        .then(_ => add_event_listeners())
        .catch(error => console.log(error))
}

function add_event_listeners()
{
    btn_minus.addEventListener("click", function(e) {
        e.preventDefault()
        quantite--
        if (quantite < 0) quantite = 0
        inp_quantite.value = quantite
    })

    btn_plus.addEventListener("click", function(e) {
        e.preventDefault()
        quantite++
        if (quantite > 99) quantite = 99
        inp_quantite.value = quantite
    })

    btn_send_qte.addEventListener("click", function(e) {
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

function gather_img_and_qte_into_array (burger_array)
{
    const images = []
    let burger_top, burger_bottom = ""

    for (let i = 0; i < burger_array.length; i++) {
        if (i === 0) {
            const arr = burger_array[0].image.split("+")
            burger_top = arr[0]
            burger_bottom = arr[1]
            images.push({img: burger_top, qte: 1})
        } else {
            images.push({img: burger_array[i].image, qte: burger_array[i]['quantite']})
        }
    }
    images.push({img: burger_bottom, qte: 1})

    return images
}

function add_img_and_qte_to_DOM (ingr_array)
{
    for (const ingr of ingr_array) {
        const div_ingr_det = document.createElement("div")
        div_ingr_det.className = "ingr_det"
        const div_qte = document.createElement("div")
        div_qte.className = "ingr_qte"
        const qte_text = document.createTextNode(`${ingr.qte}x`)
        div_qte.appendChild(qte_text)

        const img_bur = document.createElement("img")
        img_bur.setAttribute("src", `/public/image/get/ingredients/${ingr.img}/svg`)
        img_bur.setAttribute("alt", "ingredient's image")
        img_bur.className = "list-ingr-img"

        div_ingr_det.appendChild(div_qte)
        div_ingr_det.appendChild(img_bur)

        list_ingr.appendChild(div_ingr_det)
    }
}
