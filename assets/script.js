(async () => {
    const response = await fetch("/api/v1/quotes?ticker=AAPL");
    const quotes = await response.json()
    const timestamps = quotes.map((quote) => new Date(quote.timestamp * 1000))
    const stock_prices = quotes.map((quote) => quote.close)

    const ticker = document.getElementById("ticker")
    ticker.textContent = "AAPL"

    new Chart("chart", {
        type: "line",
        data: {
            labels: timestamps,
            datasets: [{
                data: stock_prices,
                fill: true,
                backgroundColor: "rgba(165, 227, 155, 0.47)",
                borderWidth: 1,
                borderColor: "rgb(27, 102, 18)",
                pointStyle: false,
                tension: 0.1
            }]
        },
        options: {
            responsive: true,
            interaction: {
                mode: 'index',
                intersect: false
            },
            plugins: {
                crosshair: {
                    line: {
                        color: "rgb(27, 102, 18)",
                        width: 1
                    },
                },
                legend: {
                    display: false,
                },
                tooltip: {
                    callbacks: {
                        label: function(context) {
                            return "$" + context.formattedValue
                        },
                        title: function(context) {
                            return moment(new Date(context[0].label)).format("ddd MMM YYYY")
                        }
                    }
                }
            },
            x: {
                type: "time",
                time: {
                    unit: 'month'
                }
            },
            scales: {
                x: {
                    ticks: {
                        callback: function(value) {
                            return moment(this.getLabelForValue(value)).format("MMM YYYY")
                        }
                    }
                },
                y: {
                    ticks: {
                        callback: function(value) {
                            return '$' + this.getLabelForValue(value)
                        }
                    }
                }
            }
        }
    });

})()
