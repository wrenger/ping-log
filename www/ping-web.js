function create_table_row(columns, css_class = null) {
    let row = document.createElement("tr")
    for (let i = 0; i < columns.length; ++i) {
        let tdNode = document.createElement("td")
        tdNode.textContent = columns[i]
        row.appendChild(tdNode)
    }
    if (css_class) {
        row.classList.add(css_class(columns))
    }
    return row
}

function update_table(id, data, columns, css_class = null) {
    let table = document.getElementById(id)
    while (table.hasChildNodes()) {
        table.removeChild(table.lastChild)
    }
    for (let i = 0; i < data.length; ++i) {
        item = data[i]
        if (item instanceof Object) {
            let row = [];
            for (let j = 0; j < columns.length; j++) {
                if (item.hasOwnProperty(columns[j])) {
                    row.push(item[columns[j]])
                }
            }
            item = row
        }
        if (item instanceof Array && item.length >= 0) {
            let row = create_table_row(item, css_class)
            table.appendChild(row)
        }
    }
}

function update_stats(stats) {
    if (stats.hasOwnProperty("min")) {
        document.getElementById("stats-min").textContent = stats["min"]
    }
    if (stats.hasOwnProperty("max")) {
        document.getElementById("stats-max").textContent = stats["max"]
    }
    if (stats.hasOwnProperty("avg")) {
        document.getElementById("stats-avg").textContent = stats["avg"]
    }
    if (stats.hasOwnProperty("lost")) {
        document.getElementById("stats-lost").textContent = stats["lost"]
    }
}

function update_files(files) {
    let filesList = document.getElementById("log-files")
    while (filesList.hasChildNodes()) {
        filesList.removeChild(filesList.lastChild)
    }
    for (let i = 0; i < files.length; ++i) {
        let fileLink = document.createElement("a")
        fileLink.href = files[i]
        fileLink.textContent = files[i].replace(/^.*[\\\/]/, '')
        let fileNode = document.createElement("li")
        fileNode.appendChild(fileLink)
        filesList.appendChild(fileNode)
    }
}

function is_lost(a) {
    return a[1] >= 1000.0 ? "lost" : null
}

function update_content(data) {
    if (data) {
        if (data.hasOwnProperty("log") && data.log instanceof Array) {
            columns = ["time", "latency"]
            update_table("log-table", data.log, columns, is_lost)
        }
        if (data.hasOwnProperty("history") && data.history instanceof Array && data.history.length > 0) {
            update_stats(data.history[0])
            columns = ["time", "min", "max", "avg", "lost"]
            update_table("history-table", data.history.splice(1), columns)
        }
        if (data.hasOwnProperty("files") && data.files instanceof Array) {
            update_files(data.files);
        }
    }
}


function load_data(callback) {
    let xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function () {
        if (this.readyState === 4 && this.status === 200) {
            let data = JSON.parse(this.responseText)
            callback(data)
        }
    }
    xhttp.open("GET", "data.json", true)
    xhttp.send()
}


function update() {
    load_data(update_content)
}


function init() {
    load_data(function (data) {
        update_content(data)
        window.scrollTo(0, localStorage.getItem("scroll_y") | 0)
    })
    window.onbeforeunload = function () {
        localStorage.setItem("scroll_y", window.pageYOffset)
    }
    setInterval(update, 10000)
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

ready(init)
