use axum::response::Html;

/// API documentation handler using Stoplight Elements
pub async fn docs_html() -> Html<&'static str> {
    let html = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">
    <title>Nokizaru Bot API Documentation</title>

    <script src="https://unpkg.com/@stoplight/elements/web-components.min.js"></script>
    <link rel="stylesheet" href="https://unpkg.com/@stoplight/elements/styles.min.css">
  </head>
  <body>
    <elements-api
      apiDescriptionUrl="http://localhost:3000/api-docs/openapi.json"
      router="hash"
    />
  </body>
</html>"#;

    Html(html)
}
