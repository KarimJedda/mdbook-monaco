// Function to initialize the editor with the first file in the data model
function initializeEditor(files, id) {
    var initialFile = files[0];
    var initialContent = localStorage.getItem(initialFile.name) || initialFile.content;
    window.editor = monaco.editor.create(document.getElementById(id), {
        value: initialContent,
        language: initialFile.language
    });

    // Set the initial event listener for content changes
    setCurrentFile(initialFile.name, files);
}

// Function to update localStorage when the content of the editor changes
function updateStorage(fileName) {
    localStorage.setItem(fileName, editor.getValue());
}

// Function to set the current file and update event listeners
function setCurrentFile(fileName, files) {
    var file = files.find(f => f.name === fileName);
    var fileContent = localStorage.getItem(fileName) || file.content;
    var fileLanguage = file.language;

    editor.setModel(monaco.editor.createModel(fileContent, fileLanguage));

    // Remove previous content change listeners to avoid duplicates
    if (window.contentChangeListener) {
        window.contentChangeListener.dispose();
    }

    // Add new content change listener
    window.contentChangeListener = editor.onDidChangeModelContent(function () {
        updateStorage(fileName);
    });
}

// Function to add event listeners to file tree items
function addFileTreeEventListeners(files) {
    files.forEach(file => {
        document.getElementById('monaco-file-' + file.name.replace(".", "-")).addEventListener('click', function() {
            setCurrentFile(file.name, files);
        });
    });
}

// Example usage with your data model
// Assuming you have an array of file objects like:
// var files = [{name: 'index.html', content: '<!-- HTML content here -->', language: 'html'}, ...];
// You would call these functions like so:
// addFileTreeEventListeners(files);
// initializeEditor(files, 'editor1_chapter2');
