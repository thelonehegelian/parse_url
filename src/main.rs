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

    /*************************
     * BUILD A URL FROM A BASE
     *************************/

    fn build_url(base: Url, path: &str) -> Result<Url, ParseError> {
        let mut url = base;
        url.set_path(path);
        Ok(url)
    }

    let base = Url::parse("https://archive.org").unwrap();
    let path = "/download/falloffeudalismi1904davi/falloffeudalismi1904davi.pdf";
    let new_url = build_url(base, path).unwrap();
    println!("{}", new_url);
}
