extern crate reqwest;
extern crate select;
#[macro_use]
extern crate error_chain;

use select::document::Document;
use select::predicate::Name;

error_chain! {
   foreign_links {
       ReqError(reqwest::Error);
       IoError(std::io::Error);
   }
}

const TARGET_PAGE: &str = "https://en.wikipedia.org/wiki/Multithreading_(computer_architecture)";

fn get_linked_pages(html_body: &str) -> Result<Vec<String>> {
    Ok(Document::from_read(html_body.as_bytes())?
        .find(Name("a"))
        .filter_map(|n| {
            if let Some(link_str) = n.attr("href") {
                if link_str.starts_with("/wiki/") {
                    Some(format!("{}/{}", "https://en.wikipedia.org", &link_str[1..]))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<String>>())
}

// Adapted from https://rust-lang-nursery.github.io/rust-cookbook/web/scraping.html
fn main() -> Result<()> {
    let html_body = reqwest::blocking::get(TARGET_PAGE)?.text()?;
    // Identify all linked wikipedia pages
    let links = get_linked_pages(&html_body)?;

    let mut longest_article_url = "".to_string();
    let mut longest_article_len = 0;
    for link in &links {
        let body = reqwest::blocking::get(link)?.text()?;
        let curr_len = body.len();
        if curr_len > longest_article_len {
            longest_article_len = curr_len;
            longest_article_url = link.to_string();
        }
    }
    println!(
        "{} was the longest article with length {}",
        longest_article_url, longest_article_len
    );
    Ok(())
}
