use url::{ParseError, Url};

fn main() {
    /************************
     * PARSE STRING TO A URL
     ************************/
    let s = "https://archive.org/download/chroniclesofcoun00grif/chroniclesofcoun00grif.pdf";
    // make a url from a string
    let url = Url::parse(s).unwrap();
    println!("{}", url);

    /******************
     * CREATE BASE URL
     *****************/

    fn base_url(mut url: Url) -> Url {
        match url.path_segments_mut() {
            Ok(mut path) => {
                path.clear();
            }
            Err(_) => {
                println!("Error");
            }
        }
        url
    }

    // create a base url remove paths
    let url = Url::parse("https://archive.org/download/chroniclesofcoun00grif/").unwrap();
    let base = base_url(url);
    println!("{}", base);
}
