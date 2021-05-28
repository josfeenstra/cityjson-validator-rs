export { setupFileSelectors };


/** process a json
 * @param {string} jsonString
 */
function readJson(jsonString) {
    console.log("size", jsonString.length);
    let json = JSON.parse(jsonString);
    console.log(json);
}


/** process a file
 * @param {File} file
 * @param {(a: string) => void} callback callback for what to do with the recovered data
 */
function processFile(file, callback) {
   
    var reader = new FileReader();
    reader.addEventListener("load", (e2)=> {
        callback(e2.target.result);
    });
    reader.readAsText(file); // start reading the file data.
}


//#region dragging & dropping, wiring, and other js business


/** process all files, recieved by either a drag&drop or file selection
 * @param {FileList} files the list of files
 * @param {string} desiredExtention what type of file to look for
 * @param {(a: string) => void} callback callback for what to do with the recovered data
 * @return {boolean} true on succes, false on failure 
 */
function processMultipleFiles(files, callback, desiredExtention="json") {

    for (let i = 0; i < files.length; i++) {
        const file = files[i];
        let type = getFileExtention(file.name);
        console.log(`file ${i}: name: ${file.name}, type: ${type}`)
        if (type == desiredExtention) {
            processFile(file, callback);
            return;
        }
    }

    alert(`please give me at least one file that ends on .${desiredExtention}!`);
}


/**
 * Setup a bunch of events so the element can be used as a drag & drop element
 * @param {HTMLElement} element The HTML element to drag & drop onto.
 * @param {(files: FileList) => void} callback what to run when files are recieved
 */
function setupDragAndDrop(element, callback, dropclass="drop-enter") {

    console.log("setting up drag events...");

    element.addEventListener("dragenter", (ev) => {
        ev.preventDefault();
        element.classList.add(dropclass)
        // console.log("entering entering...");
        return true;
    }, true);

    element.addEventListener("dragleave", (ev) => {
        ev.preventDefault();
        element.classList.remove(dropclass)
        // console.log("leaving drag....");
        return true;
    }, true);

    // note: this is a bit ugly, keep adding drop-enter every frame.
    // but due to an overlap 'bug' this is needed.
    element.addEventListener("dragover", (ev) => {
        ev.preventDefault();
        element.classList.add(dropclass)
        return true;
    }, true);

    element.addEventListener("drop", (ev) => {
        //prevent browser from opening the file when drop off
        ev.stopPropagation();
        ev.preventDefault();
        element.classList.remove(dropclass)

        var files = ev.dataTransfer.files;
        callback(files);

        return true;
    }, true);
}

/**
 * Hook up everything
 */
function setupFileSelectors() {

    // setup file input
    let input = document.getElementById("file-input");
    input.addEventListener("change", (ev) => {
        console.clear();
        processMultipleFiles(input.files, readJson, "json");
    }) 

    // setup drop mechanic
    let drops = document.querySelectorAll(".dropzone");
    if (drops.length == 0) {
        console.error("cant find the dropbox...");
        return;
    }
    for(let drop of drops) {

        setupDragAndDrop(drop, (files) => {
            console.clear();
            processMultipleFiles(files, readJson, "json")
        });

        // make the link within clickable
        let s = drop.querySelector(".selector");
        if (!s) {
            console.error("cant find selector...");
            return;
        }

        s.addEventListener("mouseenter", () => {
            s.classList.add("mouse-on")
        })

        s.addEventListener("mouseleave", () => {
            s.classList.remove("mouse-on")
        })

        // a little hack to make anything able to ask for file submissions
        s.addEventListener("click", (e) => {
            e.preventDefault();
            document.getElementById("file-input").click();
        })
    }
}


/** process all files, recieved by either a drag&drop or file selection
 * @param {string} filename
 * @return {string} 
 */
function getFileExtention(filename) {
    let parts = filename.split(".")
    if (parts.length > 1) {
        return parts.pop();
    } else {
        return ""
    }
}
