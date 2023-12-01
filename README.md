# mdbook-monaco
Multi-file editing for mdBook. 

# How it works

Custom pre-processor (or Plugin) for mdBook to support multi-file editors. 

Using this will create a pane where you can edit files. The file contents are stored in localStorage. 

````
```monaco
id: editor1_chapter1
files:
  - name: "main.js"
    language: javascript
    editable: false
    content: |
      // JavaScript code here
  - name: "index.html"
    language: html
    content: |
      <!-- HTML content here -->
actions:
  - name: "run"
    function: runCode
  - name: "build"
    function: buildProject
```
````

will generate the following:

You can adapt the functionality by providing the runCode yourself. 
