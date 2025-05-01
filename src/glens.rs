use reqwest::blocking::{Client, multipart};
use std::path::Path;

use reqwest::header::{USER_AGENT};
use reqwest::redirect::Policy;


pub fn glens() -> String {

     println!("Running post()");
    let file_path = "output_image.png"; // Change this to the path of your file
    let path = Path::new(file_path);
    let form = multipart::Form::new().file("encoded_image", path).expect("");


    let url = "https://lens.google.com/v3/upload?ep=ccm";
    let client = Client::builder().use_rustls_tls()
    .redirect(Policy::none())
    .build().expect("Error building http client");
    let request = client.post(url)
    .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36")
    .multipart(form).build().expect("Error sending POST");

    let response = client.execute(request).expect("Internet not working");

    let headers = response.headers();
    println!("{:?}", headers);
    let url = headers.get("location").expect("no redirect found");
    // let body = &response.text().expect("couldnt get a response");
    
    println!("{:?}", url);
    url.to_str().unwrap().to_string()
    
    

}
