(function () {
    "use strict";

    const UPLOAD_URL = "/upload";
    const LIST_URL = "/list";
    const REMOVE_URL = "/remove";

    const FILE_INPUT = document.getElementById("drop-file");
    const STATUS_LABEL = document.getElementById("drop-status");
    const DROP_TABLE = document.getElementById("drops");

    function getJSON(url, callback) {
        fetch(url)
            .then(response => { if (!response.ok) throw Error(response.statusText); return response })
            .then(response => response.json())
            .then(callback)
            .catch(error => console.log("request error:", error));
    }

    function uploadFile(event) {
        const files = event.target.files;
        const formData = new FormData();
        formData.append("file", files[0]);

        STATUS_LABEL.textContent = "Uploading...";

        fetch(UPLOAD_URL, {
            method: "POST",
            body: formData,
        })
            .then(response => { if (!response.ok) throw Error(response.statusText); return response })
            .then(response => response.json())
            .then(data => displayUrl(data))
            .catch(error => STATUS_LABEL.textContent = "Error: " + error)
    }

    function displayUrl(url) {
        STATUS_LABEL.textContent = "Finish: URL: " + url;
        getJSON(LIST_URL, updateDrops);
    }

    function removeDrop(row_node, url) {
        fetch(REMOVE_URL + url, {
            method: "DELETE"
        })
            .then(response => { if (!response.ok) throw Error(response.statusText); return response })
            .then(_ => DROP_TABLE.removeChild(row_node))
            .catch(error => console.log("request error:", error))
    }

    function updateDrops(data) {
        while (DROP_TABLE.firstChild)
            DROP_TABLE.removeChild(DROP_TABLE.lastChild);
        if (data instanceof Array) {
            data.forEach(url => {
                let url_cell = document.createElement("td");
                url_cell.textContent = url;

                let rm_cell = document.createElement("td");
                let rm_btn = document.createElement("button");
                rm_btn.textContent = "remove";
                rm_cell.appendChild(rm_btn);

                let row = document.createElement("tr");
                row.appendChild(url_cell);
                row.appendChild(rm_cell);

                rm_btn.addEventListener("click", _ => removeDrop(row, url));

                DROP_TABLE.appendChild(row);
            });
        }
    }

    function init() {
        FILE_INPUT.addEventListener("change", uploadFile);
        getJSON(LIST_URL, updateDrops);
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
