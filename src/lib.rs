use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Result;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CodeBlockKind::*, Event, Options, Parser, Tag};

pub struct Monaco;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MonacoBlock {
    id: String,
    files: Vec<File>,
    actions: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize)]
struct File {
    name: String,
    language: String,
    editable: Option<bool>,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Action {
    name: String,
    function: String,
}

impl Preprocessor for Monaco {
    fn name(&self) -> &str {
        "monaco"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let mut res = None;
        book.for_each_mut(|item: &mut BookItem| {
            if let Some(Err(_)) = res {
                return;
            }

            if let BookItem::Chapter(ref mut chapter) = *item {
                res = Some(Monaco::add_monaco(chapter).map(|md| {
                    chapter.content = md;
                }));
            }
        });

        res.unwrap_or(Ok(())).map(|_| book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

fn add_monaco(content: &str) -> Result<String> {
    let mut monaco_content = String::new();
    let mut in_monaco_block = false;

    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    let mut code_span = 0..0;
    let mut start_new_code_span = true;

    let mut monaco_blocks = vec![];

    let events = Parser::new_ext(content, opts);
    for (e, span) in events.into_offset_iter() {
        log::debug!("e={:?}, span={:?}", e, span);
        if let Event::Start(Tag::CodeBlock(Fenced(code))) = e.clone() {
            if &*code == "monaco" {
                in_monaco_block = true;
                monaco_content.clear();
            }
            continue;
        }

        if !in_monaco_block {
            continue;
        }

        // We're in the code block. The text is what we want.
        // Code blocks can come in multiple text events.
        if let Event::Text(_) = e {
            if start_new_code_span {
                code_span = span;
                start_new_code_span = false;
            } else {
                code_span = code_span.start..span.end;
            }

            continue;
        }

        if let Event::End(Tag::CodeBlock(Fenced(code))) = e {
            assert_eq!(
                "monaco", &*code,
                "After an opening monaco code block we expect it to close again"
            );
            in_monaco_block = false;

            let monaco_content_raw = &content[code_span.clone()];
            
            // Parse the content as a MonacoBlock
            match serde_yaml::from_str::<MonacoBlock>(&monaco_content_raw) {
                Ok(monaco_block) => {
                    // Successfully parsed the MonacoBlock
                    // Use monaco_block here for further processing
            
                    let mut file_tree_items = String::new();
                    let mut files_array_items = vec![];

                    for file in &monaco_block.files {
                        let list_item = format!("<li id=\"monaco-file-{id}\">{name}</li>\n", id = file.name.replace(".", "-"), name = file.name);
                        file_tree_items.push_str(&list_item);

                        // Escape single quotes in file content to avoid breaking the JS string
                        let escaped_content = file.content.replace("'", "\\'");

                        files_array_items.push(format!(
                            "{{name: '{}', content: '{}', language: '{}'}}",
                            file.name,
                            escaped_content.replace("\n", "\\n").replace("\"", "\\\""),
                            file.language
                        ));
                    }
                    
                    let file_tree_html = format!(
                        "<div class=\"monaco-file-tree\">\n<ul>\n{}</ul>\n</div>\n",
                        file_tree_items
                    );
                    
                    let monaco_editor_html = format!(
                        "<div id=\"{}\" class=\"monaco-editor\" style=\"min-height: 200px\"></div>\n\n",
                        monaco_block.id
                    );

                    for file in &monaco_block.files {
                        let list_item = format!("<li id=\"monaco-file-{id}\">{name}</li>\n", id = file.name.replace(".", "-"), name = file.name);
                        file_tree_items.push_str(&list_item);
                    }

                    let files_array_string = files_array_items.join(", ");

                    // TODO Maybe consider using indoc or something similar to avoid this non esthetic stuff
                    let custom_js = format!(
                        r#"<script>
window.onload = function() {{
    var files = [{}];
    addFileTreeEventListeners(files);
    initializeEditor(files, "{}");
}};
</script>"#,
                        files_array_string, monaco_block.id
                    );
                    let monaco_html = format!("{}{}{}", file_tree_html, monaco_editor_html, custom_js);
                    
                    // Append the HTML and the span to the monaco_blocks vector
                    monaco_blocks.push((span, monaco_html));
            
                    // Indicate the start of a new code span if needed
                    start_new_code_span = true;
                }
                Err(e) => {
                    // Handle parsing errors
                    eprintln!("Error parsing MonacoBlock: {}", e);
                    // Optionally, handle the error more gracefully
                }
            }
        }
    }

    let mut content = content.to_string();
    for (span, block) in monaco_blocks.iter().rev() {
        let pre_content = &content[0..span.start];
        let post_content = &content[span.end..];
        content = format!("{}\n{}{}", pre_content, block, post_content);
    }
    Ok(content)
}

impl Monaco {
    fn add_monaco(chapter: &mut Chapter) -> Result<String> {
        add_monaco(&chapter.content)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::add_monaco;

    #[test]
    fn adds_monaco() {
        let content = r#"# Chapter

```monaco
id: editor1_chapter1
files:
  - name: "main.js"
    language: javascript
    editable: false
    content: |
      console.log("Hello World");
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

Text
"#;

        let expected = r#"# Chapter


<div class="monaco-file-tree">
<ul>
<li id="monaco-file-main-js">main.js</li>
<li id="monaco-file-index-html">index.html</li>
</ul>
</div>
<div id="editor1_chapter1" class="monaco-editor" style="min-height: 200px"></div>

<script>
window.onload = function() {
    var files = [{name: 'main.js', content: 'console.log(\"Hello World\");\n', language: 'javascript'}, {name: 'index.html', content: '<!-- HTML content here -->\n', language: 'html'}];
    addFileTreeEventListeners(files);
    initializeEditor(files, "editor1_chapter1");
};
</script>

Text
"#;

        assert_eq!(expected, add_monaco(content).unwrap());
    }

}