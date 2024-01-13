use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main] //tells actix_web that is a main function
async fn main() {
    let server = HttpServer::new(|| {
	App::new()
	    .route("/", web::get().to(get_index)) //register get
	    .route("/gcd", web::post().to(post_gcd)) // register post
    });
    // || is a closure, a value that can be called as if it were a function
    // the { } is the body of the closure
    // actix starts a pool of threads, each thread calls  closure

    println!("serving on http://localhost:3666...");

    server
	.bind("127.0.0.1:3666").expect("Error binding server to address")
	.run().await.expect("error running server");
}


async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
	.content_type("text/html")
	.body( //use rust raw string
	    r#"
<title>GCD calculator</title>
<form action="/gcd" method="post">
<input type="text" name="m" />
<input type="text" name="n" />
<button type="submit">Compute GCD</button>
</form>
"#,
	)
}

// declarations can occur in any order
use serde::Deserialize; //use serde to process the form data (POST)

#[derive(Deserialize)] //tell the serde to examine the struct and generate code to parse a value of this type
// from data in the format that HTML forms use for POST requests
struct GcdParameters {
    n: u64,
    m: u64,
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
	return HttpResponse::BadRequest()
	    .content_type("text/html")
	    .body("Zero GCD");
    }

    let response = //sprintf
	format!("The GCD of {} and {} \
		 is <b>{}</b>\n",
		form.n, form.m, gcd(form.n, form.n));
    HttpResponse::Ok()
	.content_type("text/html")
	.body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
	if m < n {
	    let t = m; //could be let t: u64 = n;
	    m = n;
	    n = t;
	}
	m = m % n;
    }
    n //function return, not followed by a semicolon
}
