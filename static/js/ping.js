(function () {
    "use strict";

    const API_LOG = "/api/pings";
    const API_HISTORY = "/api/history";
    const API_HW = "/api/hw";
    const API_MC = "/api/mc";

    Chart.defaults.global.defaultFontColor = "#eeeeee"

    const RECENT_CHART = new Chart("recent-log", {
        type: 'line',
        options: {
            scales: {
                yAxes: [{
                    id: "ms",
                    type: "linear",
                    ticks: {
                        beginAtZero: true,
                    }
                }, {
                    id: "lost",
                    type: "linear",
                    ticks: {
                        beginAtZero: true,
                        precision: 0,
                    },
                    position: "right"
                }],
                xAxes: [{
                    type: "time"
                }]
            },
            legend: {
                display: true
            },
            maintainAspectRatio: false,
            elements: {
                line: {
                    tension: 0, // disables bezier curves
                }
            }
        }
    });
    const DAILY_CHART = new Chart("daily-log", {
        type: 'line',
        options: {
            scales: {
                yAxes: [{
                    id: "ms",
                    type: "linear",
                    ticks: {
                        beginAtZero: true,
                    }
                }, {
                    id: "lost",
                    type: "linear",
                    ticks: {
                        beginAtZero: true,
                        precision: 0,
                    },
                    position: "right"
                }],
                xAxes: [{
                    type: "time"
                }]
            },
            legend: {
                display: true
            },
            maintainAspectRatio: false,
            elements: {
                line: {
                    tension: 0, // disables bezier curves
                }
            }
        }
    });

    const DAY_SELECT = document.getElementById('select-day');
    const RELOAD_BTN = document.getElementById("reload");

    function pURL(url, data) {
        return encodeURI(url + "?" + new URLSearchParams(data).toString())
    }

    function parseJson(response) {
        return response.ok ? response.json() : Promise.reject(new Error(response.statusText));
    }

    function updateStats(stats) {
        if (stats instanceof Object) {
            document.getElementById("stats-min").textContent = (stats["min"] || 0).toFixed(2);
            document.getElementById("stats-max").textContent = (stats["max"] || 0).toFixed(2);
            document.getElementById("stats-avg").textContent = (stats["avg"] || 0).toFixed(2);
            document.getElementById("stats-lost").textContent = (stats["lost"] * 100).toFixed(2);
        }
    }

    function updateRecentChart(log) {
        if (!log || !(log instanceof Array)) {
            return;
        }

        let labels = [];
        let data = [];
        let lost = [];
        log.reverse();
        log.forEach(element => {
            labels.push(new Date(element.time * 1000));
            if (element.ping < 1000) {
                data.push(element.ping);
                lost.push(0);
            } else {
                data.push(0);
                lost.push(1);
            }
        });
        RECENT_CHART.data.labels = labels;
        RECENT_CHART.data.datasets = [{
            label: "Ping",
            data: data,
            borderColor: "#4996fa",
            backgroundColor: "#4996fa",
            fill: false,
            yAxisID: 'ms'
        }, {
            label: "Lost",
            data: lost,
            borderColor: "#d85858",
            backgroundColor: "#d85858",
            fill: false,
            yAxisID: 'lost'
        }];
        RECENT_CHART.update({
            duration: 0
        });
    }

    function updateDailyChart(history) {
        if (!history || !(history instanceof Array)) {
            return;
        }

        let labels = [];
        let dataAvg = [];
        let dataMin = [];
        let dataMax = [];
        let dataLost = [];
        history.reverse();
        history.forEach(element => {
            labels.push(new Date(element.time * 1000));
            dataAvg.push(element.avg);
            dataMin.push(element.min);
            dataMax.push(element.max);
            dataLost.push(element.lost);
        });
        DAILY_CHART.data.labels = labels;
        DAILY_CHART.data.datasets = [{
            label: "Avg",
            data: dataAvg,
            borderColor: "#4996fa",
            backgroundColor: "#4996fa",
            fill: false,
            yAxisID: 'ms'
        }, {
            label: "Min",
            data: dataMin,
            borderColor: "#58d878",
            backgroundColor: "#58d878",
            fill: false,
            yAxisID: 'ms'
        }, {
            label: "Max",
            data: dataMax,
            borderColor: "#d8d658",
            backgroundColor: "#d8d658",
            fill: false,
            yAxisID: 'ms'
        }, {
            label: "Lost",
            data: dataLost,
            borderColor: "#d85858",
            backgroundColor: "#d85858",
            fill: false,
            yAxisID: 'lost'
        }];
        DAILY_CHART.update({
            duration: 0
        });
    }

    function updateHw(status) {
        if (status instanceof Object) {
            document.getElementById("hw-load").textContent = ((status["load"] || 0) * 100).toFixed(2);
            document.getElementById("hw-temperature").textContent = (status["temperature"] || 0).toFixed(2);
            document.getElementById("hw-mem-used").textContent = (status["memory_used"] || 0).toFixed(2);
            document.getElementById("hw-mem-total").textContent = (status["memory_total"] || 0).toFixed(2);
        }
    }

    function updateMc(stats) {
        const mcRoot = document.getElementById("mc-root");
        while (mcRoot.childElementCount > 1) {
            mcRoot.removeChild(mcRoot.lastElementChild);
        }
        const mcTemplate = mcRoot.firstElementChild;
        if (stats instanceof Array) {
            stats.forEach(status => {
                if (status instanceof Object) {
                    let elem = mcTemplate.cloneNode(true);
                    elem.hidden = false;
                    elem.querySelector("#mc-addr").textContent = status["addr"];
                    elem.querySelector("#mc-description").textContent = status["description"];
                    elem.querySelector("#mc-version").textContent = status["version"];
                    elem.querySelector("#mc-curr").textContent = status["players"];
                    elem.querySelector("#mc-max").textContent = status["max_players"];
                    mcRoot.appendChild(elem);
                }
            });
        }
    }

    function update() {
        fetch(pURL(API_LOG, {
            start: Math.round(Date.now() / 1000),
            end: Math.round(moment().subtract(1, 'hour') / 1000),
            count: 60,
        }))
            .then(parseJson)
            .then((data) => {
                if (data instanceof Array) {
                    updateStats(data[0]);
                    updateRecentChart(data[1]);
                }
            })
            .catch(console.error);

        fetch(API_MC)
            .then(parseJson)
            .then(updateMc)
            .catch(_ => updateMc([]));
        fetch(API_HW)
            .then(parseJson)
            .then(updateHw)
            .catch(_ => updateHw([]));

        let day = moment().subtract(DAY_SELECT.value, "day");
        fetch(pURL(API_HISTORY, {
            start: Math.round(day.startOf("day").valueOf() / 1000),
            end: Math.round(day.endOf("day").valueOf() / 1000),
            count: 24,
        }))
            .then(parseJson)
            .then(updateDailyChart)
            .catch(console.error);
    }

    // -- Init --

    RELOAD_BTN.addEventListener("click", update);

    for (var i = 0; i <= 7; ++i) {
        var option = document.createElement("option");
        option.text = moment().subtract(i, "day").format("dd DD.MM.YYYY");
        option.value = i;
        DAY_SELECT.options.add(option)
    }

    DAY_SELECT.addEventListener("change", update);

    update();
    setInterval(update, 30000);
})();
