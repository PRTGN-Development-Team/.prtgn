
pub fn fox() {

    fetch();




}
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn fetch() -> Result<(), reqwest::Error> {

    let url = "https://api.fox.pics/v1/get-random-foxes?amount=1";

    eprintln!("Fetching {url:?}...");

    let res = reqwest::get(url).await?;

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

        let mut chars = body.chars();
        chars.next();
        chars.next_back();
        chars.next();
        chars.next_back();


    println!("{}", chars.as_str().to_string());

    Ok(())


}