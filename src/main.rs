use scraper;

//Struct with base info ( title, link, current chapter)

enum Mangasite {
    Flamescans,
    Alphascans,
}
struct Manga {
    title: String,
    base_scrap_link: String,
    chapter: u16,
    manga_site: Mangasite,
}

impl Manga {
    fn get_webpage(&self) -> Result<reqwest::blocking::Response, reqwest::Error> {
        match &self.manga_site {
            Mangasite::Alphascans => {
                let url = String::from(&self.base_scrap_link) + &self.chapter.to_string();
                reqwest::blocking::get(url)
            }
            Mangasite::Flamescans => {
                let url = String::from(&self.base_scrap_link) + &self.chapter.to_string();
                reqwest::blocking::get(url)
            }
        }
    }

    fn scrap_manga(&self) {
        match &self.manga_site {
            Mangasite::Alphascans => {
                let response = self.get_webpage();
                if response.is_ok() {
                    let html = &response
                        .expect("Invalid Response")
                        .text()
                        .expect("Invalid text method");

                    let document = scraper::Html::parse_document(&html);
                    let selector =
                        scraper::Selector::parse("div.chdesc>p").expect("Invalid css selector");

                        let len = document.select(&selector).count();
                        if len > 0 {
                            println!("Send telegram message!");
                        }

                } else {
                    println!("Error on response");
                }
            }

            Mangasite::Flamescans => (),
        }
    }
}

//create the basic manga list with some entries
fn build_manga_list() -> Vec<Manga> {
    let mut scrap_list: Vec<Manga> = Vec::new();

    scrap_list.push(Manga {
        title: "Tower of God".to_string(),
        base_scrap_link: "https://alpha-scans.org/tower-of-god-chapter-".to_string(),
        chapter: 548,
        manga_site: Mangasite::Alphascans,
    });

    scrap_list.push(Manga {
        title: "Omniscient Reader’s Viewpoint Chapter".to_string(),
        base_scrap_link: "https://flamescans.org/1656345662-omniscient-readers-viewpoint-chapter-"
            .to_string(),
        chapter: 112,
        manga_site: Mangasite::Flamescans,
    });

    scrap_list
}

//scrap all elements of the list
fn scrap_all(scrap_list: Vec<Manga>) {
    for element in scrap_list {
        element.scrap_manga();
    }
}

fn main() {
    println!("Welcome to the manga scrapper");
    println!("--------------------------------");
    scrap_all(build_manga_list());
}


//print variable type
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

