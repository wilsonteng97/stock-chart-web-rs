(async () => {

    const url_params = new URLSearchParams(window.location.search);
    url_params.set("ticker", url_params.get("ticker") || "AAPL")
    url_params.set("interval", url_params.get("interval") || "1d")
    url_params.set("period", url_params.get("period") || "1y")

    // TODO: currency should be returned in the quotes
    const currency = "$";

    const ticker = url_params.get("ticker").toUpperCase()
    const response = await fetch("/api/v1/quotes?" + url_params.toString());
    const quotes = await response.json()
    const timestamps = quotes.map((quote) => new Date(quote.timestamp * 1000))
    const stock_prices = quotes.map((quote) => quote.close)

    document.getElementById("ticker").textContent = ticker

    const all_periods = [
        "1mo",
        "6mo",
        "ytd",
        "1y",
        "5y"
    ]

    for (period of all_periods) {
        const element = document.getElementById("period-" + period)
        const parent = element.parentNode;
        const link = document.createElement("a")
        url_params.set("period", period)
        link.href = "?" + url_params.toString()
        link.appendChild(element.cloneNode(true))
        parent.replaceChild(link, element)
    }


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

            animation: {
                duration: 100,
            },

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
                            return currency + context.formattedValue
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
                            return currency + this.getLabelForValue(value)
                        }
                    }
                }
            }
        }
    });

})()
