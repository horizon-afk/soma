use reqwest::blocking::{Client, multipart};
use std::path::Path;
use scraper::{Html, Selector};
use regex::Regex;


pub fn glens() -> String {

    println!("Running post()");
    let file_path = "output_image.png"; // Change this to the path of your file
    let path = Path::new(file_path);
    let form = multipart::Form::new().file("encoded_image", path).expect("");


    let url = "https://lens.google.com/v3/upload?ep=ccm";
    let client = Client::builder().referer(false).build().expect("Error building http client");
    let request = client.post(url).header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)").multipart(form).build().expect("Error sending POST");


    let response = client.execute(request).expect("Internet not working");
    let body = response.text().expect("couldnt get a response");
    let link = generate_link(&body);
    link
}


fn generate_link(response_body: &str) -> String{
    let document = Html::parse_document(&response_body);
    let selector = Selector::parse("c-wiz").expect("Not working");
    
    //ig the required one is the first one
    let link_data = document.select(&selector).next().expect("scrapping error").value().attr("data-p");

    let p = find_p(link_data.expect("oops"));

    format!("https://lens.google.com/search?ep=ccm&p={}", p)
   

}

fn find_p(text : &str) -> &str {

    let re = Regex::new(r"(Abrf[^\\\s]*)\\").unwrap();
    if let Some(value) = re.captures(text) {
        let capture = value.get(1).unwrap().as_str();
        return capture; 
    } else {
        return "";
    }
}
