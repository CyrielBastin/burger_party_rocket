const cmd_datetime = document.getElementById("datetime").textContent;

(async function()
{
    const resp_cmd = await fetch(`/command/fetch/command/${cmd_datetime}`)
    const cmd_json = await resp_cmd.json()

    console.log(cmd_json)
})()
