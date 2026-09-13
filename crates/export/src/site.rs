use std::path::Path;

const INDEX_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/docsify@4/lib/themes/vue.css">
</head>
<body>
<div id="app">Loading...</div>
<script>
window.$docsify = {
  name: "{title}",
  homepage: "README.md",
  loadSidebar: true,
  subMaxLevel: 2,
};
</script>
<script src="https://cdn.jsdelivr.net/npm/docsify@4"></script>
<script src="https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/docsify-mermaid@2/dist/docsify-mermaid.js"></script>
<script>mermaid.initialize({ startOnLoad: false });</script>
</body>
</html>
"#;

pub fn write_site_loader(output_dir: impl AsRef<Path>, title: &str) -> std::io::Result<()> {
    let out = output_dir.as_ref();
    std::fs::create_dir_all(out)?;
    std::fs::write(out.join("index.html"), INDEX_HTML.replace("{title}", title))?;
    std::fs::write(out.join(".nojekyll"), "")?;
    Ok(())
}
