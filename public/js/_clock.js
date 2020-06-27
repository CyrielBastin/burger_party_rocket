const clock_container = document.getElementById("clock");

(async function ()
{
    const current_datetime = new Date()
    const current_clock =
        {
            hour: current_datetime.getHours(),
            minute: current_datetime.getMinutes(),
            second: current_datetime.getSeconds()
        }

    setInterval(function () {
        current_clock.second ++
        if (current_clock.second === 60) {
            current_clock.second = 0
            current_clock.minute ++
        }
        if (current_clock.minute === 60) {
            current_clock.minute = 0
            current_clock.hour ++
        }
        if (current_clock.hour === 24) current_clock.hour = 0

        clock_container.style.display = "block"
        update_numbers_format(current_clock)
        update_UI(current_clock)
    }, 1000)
})()

function update_numbers_format(current_clock)
{
    if (current_clock.hour < 10) current_clock.hour = "0" + current_clock.hour
    if (current_clock.minute < 10) current_clock.minute = "0" + current_clock.minute
    if (current_clock.second < 10) current_clock.second = "0" + current_clock.second
}

async function update_UI(clock)
{
    clock_container.innerText = `${clock.hour}:${clock.minute}:${clock.second}`
}
