function create_table_row(columns, css_class = null) {
    var row = document.createElement("tr")
    for (var i = 0; i < columns.length; ++i) {
        var tdNode = document.createElement("td")
        tdNode.textContent = columns[i]
        row.appendChild(tdNode)
    }
    if (css_class) {
        row.classList.add(css_class(columns))
    }
    return row
}

function update_table(id, data, css_class = null) {
    var table = document.getElementById(id)
    while (table.hasChildNodes()) {
        table.removeChild(table.lastChild)
    }
    for (var i = 0; i < data.length; ++i) {
        if (data[i] instanceof Array && data[i].length >= 0) {
            var row = create_table_row(data[i], css_class)
            table.appendChild(row)
        }
    }
}

function update_stats(stats) {
    if (stats instanceof Array && stats.length >= 5) {
        document.getElementById("stats-min").textContent = stats[1]
        document.getElementById("stats-max").textContent = stats[2]
        document.getElementById("stats-avg").textContent = stats[3]
        document.getElementById("stats-lost").textContent = stats[4]
    }
}

function update_files(files) {
    var filesList = document.getElementById("log-files")
    while (filesList.hasChildNodes()) {
        filesList.removeChild(filesList.lastChild)
    }
    for (var i = 0; i < files.length; ++i) {
        var fileLink = document.createElement("a")
        fileLink.href = files[i]
        fileLink.textContent = files[i].replace(/^.*[\\\/]/, '')
        var fileNode = document.createElement("li")
        fileNode.appendChild(fileLink)
        filesList.appendChild(fileNode)
    }
}

function update_content(data) {
    if (data) {
        if (data.hasOwnProperty("log") && data.log instanceof Array) {
            update_table("log-table", data.log, function (a) {
                return a[1] >= 1000.0 ? "lost" : null
            })
        }
        if (data.hasOwnProperty("history") && data.history instanceof Array && data.history.length > 0) {
            update_stats(data.history[0])
            update_table("history-table", data.history.splice(1))
        }
        if (data.hasOwnProperty("files") && data.files instanceof Array) {
            update_files(data.files);
        }
    }
}


function load_data(callback) {
    var xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function () {
        if (this.readyState === 4 && this.status === 200) {
            var data = JSON.parse(this.responseText)
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
