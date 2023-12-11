const get_time_data = async () => {
    const response = await fetch("data.json");
    const rust_data = await response.json();

    const day_array = rust_data.map(entry => entry.day);
    const minutes_array = rust_data.map(entry => entry.minutes);

    console.log(day_array);
    console.log(minutes_array);

    const ctx = document.getElementById("myChart");

    new Chart(ctx, {
        type: "bar",
        data: {
            labels: day_array,
            datasets: [{
                label: "minutes worked per day",
                data: minutes_array,
                borderWidth: 1
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

get_time_data();