use actix_web::{HttpResponse, Responder};

pub async fn index() -> impl Responder {
    let html = r#"
    <html>
        <head>
            <title>Text Prediction Api</title>
        </head>
        <body>
            <h1>Text Prediction Api</h1>
        </body>
    </html>
    "#;
    HttpResponse::Ok().content_type("text/html").body(html)
}

pub async fn not_found() -> impl Responder {
    let html = r#"
    <html>
        <head>
            <title>404 Not Found</title>
        </head>
        <body>
            <h1>404 Not Found</h1>
            <p>The page you requested could not be found.</p>
        </body>
    </html>
    "#;

    HttpResponse::NotFound()
        .content_type("text/html")
        .body(html)
}
