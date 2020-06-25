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

    function getJSON(url, callback, err_callback = null) {
        var request = new XMLHttpRequest();
        request.onload = (e) => {
            if (e.target.status === 200) {
                callback(JSON.parse(e.target.responseText));
            } else if (err_callback != null) {
                err_callback(e);
            } else {
                console.error("request error:", e.target.status, e.target.responseURL);
            }
        };
        if (err_callback != null) {
            request.onerror = err_callback;
        } else {
            request.onerror = e => console.error("request error:", e.target.status, e.target.responseURL);
        }
        request.open("GET", url, true);
        request.send();
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

    function getHw(callback) {
        getJSON(encodeURI(API_HW), callback, _ => callback([]));
    }

    function updateHw(status) {
        if (status instanceof Object) {
            document.getElementById("hw-load").textContent = status["load"] * 100;
            document.getElementById("hw-temperature").textContent = status["temperature"].toFixed(2);
            document.getElementById("hw-mem-used").textContent = status["memory_used"].toFixed(2);
            document.getElementById("hw-mem-total").textContent = status["memory_total"].toFixed(2);
        }
    }

    function getMc(callback) {
        getJSON(encodeURI(API_MC), callback, _ => callback([]));
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
        const now = Math.round(Date.now() / 1000);
        const hour = Math.round(moment().subtract(1, 'hour') / 1000);
        getLog((data) => {
            if (data instanceof Array) {
                updateStats(data[0]);
                updateRecentChart(data[1]);
            }
        }, now, hour);
        getMc(updateMc);
        getHw(updateHw);

        let day = moment().subtract(DAY_SELECT.value, "day");
        const dayStart = Math.round(day.startOf("day").valueOf() / 1000);
        const dayEnd = Math.round(day.endOf("day").valueOf() / 1000);
        getHistory(updateDailyChart, dayEnd, dayStart);
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
