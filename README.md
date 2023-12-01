# mdbook-monaco
Multi-file editing for mdBook. 

# How it works

Custom pre-processor (or Plugin) for mdBook to support multi-file editors. 

Using this will create a pane where you can edit files. The file contents are stored in localStorage. 

```
```monaco
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
action: customJSFunction
``````
