document.querySelector(".btn-validate_command")
        .addEventListener("click", function (e) {
            toggle_modal("open")
        })
const modal_payment = document.querySelector(".modal-payment")
const modal_step_one = document.querySelector(".step-1")
document.querySelector(".payment-btn-ok")
        .addEventListener("click", function (e) {
            start_timer()
        })
const modal_step_two = document.querySelector(".step-2")
const modal_step_three = document.querySelector(".step-3")
let is_modal_shown = false

window.addEventListener("keyup", function (e) {
    if (e.key === "Escape")
    {
        if (is_modal_shown) toggle_modal("close")
    }
})


function toggle_modal(option)
{
    switch (option) {
        case "open": {
            modal_payment.style.display = "block"
            is_modal_shown = !is_modal_shown
            break
        }
        case "close": {
            modal_payment.style.display = "none"
            is_modal_shown = !is_modal_shown
            break
        }
        default: break
    }
}

function start_timer()
{
    setTimeout(display_step_three, 1500)
    modal_step_one.style.display = "none"
    modal_step_two.style.display = "block"
}

function display_step_three()
{
    setTimeout(redirect_to_command_payed_and_accepted, 1500)
    modal_step_two.style.display = "none"
    modal_step_three.style.display = "block"
}

function redirect_to_command_payed_and_accepted()
{
    window.location.href = "/command/payed-and-accepted"
}
