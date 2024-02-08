use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::fs;

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("Request received from: {}", _req.uri());
    /*
    let file = if _req.uri().path() == "/" {
        fs::read("public/index.html").unwrap()
    } else {
        let path = _req.uri().path().trim_start_matches("/");
        match fs::read(format!("public/{}", path)) {
            Ok(file) => file,
            Err(_) => fs::read("public/404.html").unwrap(),
        }
    };
    */

    //if path end with / , remove it
    let path = if _req.uri().path().ends_with("/") {
        _req.uri().path().trim_end_matches("/")
    } else {
        _req.uri().path()
    };

    //if path is folder, set path to index.html, then if not , check if it ends with .html, if not add .html to it
    let path = if fs::metadata(format!("public/{}", path)).unwrap().is_dir(){
        format!("{}/index.html", path)
    } else {
        path.to_string()
    };



   
    

    //if path is / then serve index.html, if path is not / then serve the file
    let file = if path == "" {
        fs::read("public/index.html").unwrap()
    }    
    else {
        match fs::read(format!("public/{}", path)) {
            Ok(file) => file,
            Err(_) => fs::read("public/404.html").unwrap(),
        }
    };

    //if file is / then serve index.html, if file is a css file then serve it with text/css content type
    let content_type =
     if _req.uri().path().ends_with(".css") {
        "text/css"
    } //add js content type
    else if _req.uri().path().ends_with(".js") {
        "text/javascript"
    } else {
        "text/html"
    };

    // Create a new response
    let response = Response::builder()
        .status(200)
        .header("Content-Type", content_type)
        .body(Body::from(file))
        .unwrap();
        Ok(response)

    

    
}

#[tokio::main]
async fn main() {
    // Create a new hyper Server
    let addr = ([127, 0, 0, 1], 3000).into();

    //if public folder is not present then create it
    if !std::path::Path::new("public").exists() {
        fs::create_dir("public").expect("Unable to create public folder");
    }

    
    let make_svc = make_service_fn(|_conn| {
        async {
            // Service to handle incoming requests
            Ok::<_, Infallible>(service_fn(handle_request))
        }
    });

    let server = hyper::server::Server::bind(&addr).serve(make_svc);

    println!("Server running on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}