use webbrowser;

mod screensnap;
mod context;
mod glens;


fn main() -> anyhow::Result<()>  {
    //starts the screenshots app
    screensnap::screensnap().expect("Coundn't start");
    
    //everything is synced
    let url = glens::glens();

    //open the link in a browser
    webbrowser::open(url.as_str()).expect("Cant open url");
    
    Ok(())
}