const bur_pane = document.querySelector(".bur-pane")
const drk_pane = document.querySelector(".drk-pane")

document.querySelector(".tab-burger")
        .addEventListener("click", function (e) {
            e.preventDefault()
            drk_pane.style.display = "none"
            bur_pane.style.display = "grid"
        })
document.querySelector(".tab-drink")
        .addEventListener("click", function (e) {
            e.preventDefault()
            bur_pane.style.display = "none"
            drk_pane.style.display = "grid"
        })
