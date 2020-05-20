const id_burger = document.getElementById("id_burger").innerText
const list_ingr = document.querySelector(".list-ingr")
const btn_minus = document.querySelector(".btn-minus")
const btn_plus = document.querySelector(".btn-plus")
const div_quantite = document.getElementById("bur_drk-qte")
let quantite = div_quantite.innerText

if (id_burger)
{
    fetch(`/public/from-json/fetch/ingredients-for-burger/${id_burger}`)
        .then(data => data.json())
        .then(ingredient => gather_img_and_qte_into_array(ingredient))
        .then(ingr_array => add_img_and_qte_to_DOM(ingr_array))
}

btn_minus.addEventListener("click", function(e) {
    e.preventDefault()
    quantite--
    if (quantite < 0) quantite = 0
    div_quantite.innerText = quantite
})

btn_plus.addEventListener("click", function(e) {
    e.preventDefault()
    quantite++
    if (quantite > 99) quantite = 99
    div_quantite.innerText = quantite
})

/*
 * Change width to fixed width for panel qte_number
 *
 */


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
