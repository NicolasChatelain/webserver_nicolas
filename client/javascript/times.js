const get_time_data = async () => {
    const response = await fetch("data.json");
    const rust_data = await response.json();

    const day_array = rust_data.map(entry => entry.day);
    const minutes_array = rust_data.map(entry => entry.minutes);

    const ctx = document.getElementById("myChart");


    new Chart(ctx, {
        type: "bar",
        data: {
            labels: day_array,
            datasets: [{
                label: "minutes worked per day",
                data: minutes_array,
                borderWidth: 1,
                backgroundColor: "#41E169",
                color: "black",
            }]
        },
        options: {
            scales: {
                y: {
                    beginAtZero: true
                }
            }
        }
    });
}

const addNewTime = () => {
    const modal = document.querySelector("#new_time_modal")
    const openModal = document.querySelector("#new_time_button");
    const closeModel = document.querySelector("#close_modal");

    openModal.addEventListener("click", () => {
        modal.showModal();
    })

    closeModel.addEventListener("click", () => {
        modal.close();
    })
}

const newTimeSubmitted = async () => {
    const form = document.querySelector("#work_time_form");


    form.addEventListener("submit",  async () => {

        const day_input = document.getElementById("day_input");
        const day = new Date(day_input.value);
        const formattedDay = `${day.getDate()}/${day.getMonth() + 1}/${day.getFullYear()}`;


        const formdata = {
            day: formattedDay,
            minutes: document.getElementById("time_input").value,
        }


        const json_data = JSON.stringify(formdata);

        try {

            const response = await fetch("http://localhost:7878/time/new", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: json_data
            });

            window.location.reload(true);

        } catch (error) {
            console.error(error);
        }

    });
}

newTimeSubmitted();
get_time_data();
addNewTime();