const bur_pane = document.querySelector(".bur-pane");
const drk_pane = document.querySelector(".drk-pane");
const btn_goto_cmd_det = document.getElementById("goto-command_details");

document.querySelector(".tab-burger")
        .addEventListener("click", function (e) {
            e.preventDefault()
            drk_pane.style.display = "none"
            bur_pane.style.display = "grid"
        });
document.querySelector(".tab-drink")
        .addEventListener("click", function (e) {
            e.preventDefault()
            bur_pane.style.display = "none"
            drk_pane.style.display = "grid"
        });


(async function()
{
    const resp_burgers = await fetch("/command/fetch/burgers")
    const burgers_data = await resp_burgers.text()

    const resp_drinks = await fetch("/command/fetch/drinks")
    const drinks_data = await resp_drinks.text()

    if (burgers_data === "" && drinks_data === "")
    {
        btn_goto_cmd_det.removeAttribute("href")
        btn_goto_cmd_det.style.display = "none"
    }
})()
