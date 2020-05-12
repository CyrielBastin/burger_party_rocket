const id_burger = document.getElementById("id_burger").innerText

if (id_burger)
{
    fetch(`/public/json/fetch/ingredients-for-burger/${id_burger}`)
        .then(_ => _.json())
        .then(_ => console.table(_))
}
