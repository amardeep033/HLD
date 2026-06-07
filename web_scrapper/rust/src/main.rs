use std::collections::{HashMap, HashSet};

use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://www.phonepe.com";
    let current_page = "/contact-us";
    let html_string = fetch_html(&format!("{}{}", base_url, current_page)).await?;

    let links_store = extract_links(&html_string);
    println!("Found {} links", links_store.len());
    for (i, (link, display_msgs)) in links_store.iter().enumerate() {
        //handling validlinks, relative and absolute URLs
        if link.starts_with('/') {
            println!("{}. {}{} ----- {:?}", i + 1, base_url, link, display_msgs);
        } else if link.starts_with("https://") || link.starts_with("http://") {
            println!("{}. {} ----- {:?}", i + 1, link, display_msgs);
        } else {
            println!("{}. Invalid link : {:?} ----- {:?}", i+1, link, display_msgs);
        }
    }
    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    reqwest::get(url)
        .await?
        .error_for_status()?
        .text()
        .await
}

//hashset is used to store unique links and avoid duplicates
fn extract_links(html: &str) -> HashMap<String,HashSet<String>> {
    let mut links_store: HashMap<String, HashSet<String>> = HashMap::new();

    let parsed_document = Html::parse_document(html);
    let link_selector = Selector::parse("a").unwrap();

    // example: <a href="https://www.phonepe.com/about-us">About Us</a>
    // anchor is a tag ---- parsed_document.select(&link_selector)
    // href is attribute name of anchor tag ---- matched_element.value().attr("href")
    // url is the value of href attribute ---- url = matched_element.value().attr("href")
    // and text is the display message of the link ---- value = matched_element.text().collect::<Vec<_>>().join(" ")

    for matched_element in parsed_document.select(&link_selector) {
        let url = match matched_element.value().attr("href"){
            Some(val) => val,
            None => {
                continue
            }
        };
        let value = matched_element.text().collect::<Vec<_>>().join(" ");
        links_store.entry(url.to_string())
        .and_modify(|values_set: &mut HashSet<String>| { values_set.insert(value.clone()); })
        .or_insert_with(|| HashSet::from([value]));
    }
    
    links_store
}