use webbrowser;
use std::process::Command;
mod screensnap;
mod context;
mod glens;


fn main() -> anyhow::Result<()>  {
    //starts the screenshots app
    screensnap::screensnap().expect("Coundn't start");
    
    //everything is synced
    let url = glens::glens();

    //open the link in a browser
    // webbrowser::open_browser(webbrowser::Browser::Default, url.as_str()).expect("something went wrong with browsers");


    let firefox_path = r"C:\Program Files\Mozilla Firefox\firefox.exe";
    
    let _ = Command::new(firefox_path)
    .arg(url.as_str())
    .spawn()
    .expect("Failed to open URL in Firefox");
    
    Ok(())
}