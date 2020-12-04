use reqwest::header::COOKIE;
static COOKIE_VALUE: &str = "session=53616c7465645f5f46336af00478d6d9ceb0e606829dca860550b1bc169d6d84990a72eb9fbba73679142f6cb5823527";

fn http_get(url: String) -> reqwest::blocking::Response {
    let client = reqwest::blocking::Client::new();
    let data = client
        .get(&url)
        .header(COOKIE, COOKIE_VALUE)
        .send();

    return data.unwrap();
}

pub fn fetch_data(day: i32) -> String {
    println!("Fetching data...");
    let url = format!("https://adventofcode.com/2020/day/{}/input", day);
    let res = http_get(url);
    let s = res.text();
    return s.unwrap();
}
