(function () {
    "use strict";

    const API_LOG = "/api/pings";
    const API_HISTORY = "/api/history";

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

    function getJSON(url, callback) {
        var xhttp = new XMLHttpRequest();
        xhttp.onload = (e) => {
            if (e.target.status === 200) {
                callback(JSON.parse(e.target.responseText));
            } else {
                console.error("request error:", e.target.status, e.target.responseURL);
            }
        };
        xhttp.onerror = (e) => console.error("request error:", e.target.status, e.target.responseURL);
        xhttp.open("GET", url, true);
        xhttp.send();
    }

    function param(values) {
        values = Object.keys(values).map(key => {
            return encodeURIComponent(key) + "=" + encodeURIComponent(values[key]);
        });
        return values.join("&");
    }

    function getLog(callback, start = null, end = null, offset = 0, count = 60) {
        let params = {
            count: count,
        };
        if (start) params.start = start;
        if (end) params.end = end;
        if (offset > 0) params.offset = offset;
        getJSON(encodeURI(API_LOG + "?" + param(params)), callback);
    }

    function getHistory(callback, start = null, end = null, offset = 0, count = 24) {
        let params = {
            count: count,
        };
        if (start) params.start = start;
        if (end) params.end = end;
        if (offset > 0) params.offset = offset;
        getJSON(encodeURI(API_HISTORY + "?" + param(params)), callback);
    }

    function updateStats(stats) {
        if (stats instanceof Object) {
            document.getElementById("stats-min").textContent = stats["min"];
            document.getElementById("stats-max").textContent = stats["max"];
            document.getElementById("stats-avg").textContent = stats["avg"];
            document.getElementById("stats-lost").textContent = stats["lost"] * 100;
        }
    }

    function updateRecentChart(log) {
        if (!log || !(log instanceof Array)) {
            return;
        }

        let labels = [];
        let data = [];
        let lost = [];
        log.forEach(element => {
            labels.unshift(new Date(element.time * 1000));
            if (element.ping < 1000) {
                data.unshift(element.ping);
                lost.unshift(0);
            } else {
                data.unshift(0);
                lost.unshift(1);
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
        history.forEach(element => {
            labels.unshift(new Date(element.time * 1000));
            dataAvg.unshift(element.avg);
            dataMin.unshift(element.min);
            dataMax.unshift(element.max);
            dataLost.unshift(element.lost);

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

    function update() {
        const now = Math.round(Date.now() / 1000);
        const hour = Math.round(moment().subtract(1, 'hour') / 1000);
        getLog(updateRecentChart, now, hour);

        let day = moment().subtract(DAY_SELECT.value, "day");
        const dayStart = Math.round(day.startOf("day").valueOf() / 1000);
        const dayEnd = Math.round(day.endOf("day").valueOf() / 1000);
        getHistory(function (data) {
            updateStats(data[0]);
            updateDailyChart(data);
        }, dayEnd, dayStart);
    }

    function init() {
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
    }

    function ready(fn) {
        if (document.attachEvent ?
            document.readyState === "complete" :
            document.readyState !== "loading") {
            fn();
        } else {
            document.addEventListener('DOMContentLoaded', fn);
        }
    }

    ready(init);
})();
