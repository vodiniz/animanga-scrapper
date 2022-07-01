use failure;
use headless_chrome::Browser;
use scraper;
use std::iter::zip;
// Name your user agent after your app?
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

//Struct with base info ( title, link, current chapter)

enum Mangasite {
    Flamescans,
    Alphascans,
    Cosmicscans,
    Mangaplus,
}

struct Manga {
    title: String,
    base_scrap_link: String,
    chapter: u16,
    manga_site: Mangasite,
    last_manga: Option<String>,
}

impl Manga {

    // fn add_chapter(&mut self){
    //     self.chapter += 1;
    // }

    fn browse_mangaplus(&self) -> Result<(Vec<String>, Vec<String>), failure::Error> {
        let mut chapter_vec: Vec<String> = Vec::new();
        let mut link_vec: Vec<String> = Vec::new();

        let browser = Browser::default()?;
        let tab = browser.wait_for_initial_tab()?;

        tab.navigate_to(&self.base_scrap_link)?;
        let contents = tab.wait_for_elements("p.ChapterListItem-module_name_3h9dj")?;
        let chapter_link =
            tab.wait_for_elements("a.ChapterListItem-module_commentContainer_1P6qt")?;

        for (content, link) in zip(contents, chapter_link) {
            for attribute in link.get_attributes()? {
                link_vec.push(attribute.get("href").unwrap().to_string());
            }

            for content_children in content.get_description()?.children {
                for element in content_children {
                    chapter_vec.push(element.node_value);
                }
            }
        }
        Ok((chapter_vec, link_vec))
    }

    fn scrap_manga(&mut self) {
        let client = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .expect("Error building client");

        match &self.manga_site {
            Mangasite::Alphascans => {
                self.last_manga =
                    Some(String::from(&self.base_scrap_link) + &self.chapter.to_string());

                let response = client
                    .get(self.last_manga.as_ref().expect("Error on latest link"))
                    .send();

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
                        self.send_message();
                    }
                } else {
                    println!("Error on response");
                }
            }

            Mangasite::Flamescans => {
                self.last_manga =
                    Some(String::from(&self.base_scrap_link) + &self.chapter.to_string());
                let response = client
                    .get(self.last_manga.as_ref().expect("Error on latest link"))
                    .send();

                if response.is_ok() {
                    let html = &response
                        .expect("Invalid Response")
                        .text()
                        .expect("Invalid text method");

                    let document = scraper::Html::parse_document(&html);
                    let selector =
                        scraper::Selector::parse("div.chapterbody").expect("Invalid css selector");

                    let len = document.select(&selector).count();
                    if len > 0 {
                        self.send_message();
                    }
                } else {
                    println!("Error on response");
                }
            }

            Mangasite::Cosmicscans => {
                self.last_manga =
                    Some(String::from(&self.base_scrap_link) + &self.chapter.to_string());

                let response = client
                    .get(self.last_manga.as_ref().expect("Error on latest link"))
                    .send();
                if response.is_ok() {
                    let html = &response
                        .expect("Invalid Response")
                        .text()
                        .expect("Invalid text method");

                    let document = scraper::Html::parse_document(&html);
                    let selector =
                        scraper::Selector::parse("div.chapterbody").expect("Invalid css selector");

                    let len = document.select(&selector).count();
                    if len > 0 {
                        self.send_message();
                    }
                } else {
                    println!("Error on response");
                }
            }
            Mangasite::Mangaplus => match self.browse_mangaplus() {
                Ok(mangaplus_tuple) => {
                    for (chapter, link) in zip(mangaplus_tuple.0, mangaplus_tuple.1) {
                        let v: Vec<String> = link.split("/").map(|s| s.to_string()).collect();

                        match chapter.replace("#", "").parse::<u16>() {
                            Ok(number) => {
                                if number == self.chapter {
                                    self.last_manga = Some(format!(
                                        "{}{}",
                                        "https://mangaplus.shueisha.co.jp/viewer/", &v[2]
                                    ));
                                    self.send_message();
                                }
                            }
                            Err(_) => (),
                        }
                    }
                }
                Err(_) => (),
            },
        }
    }

    fn send_message(&self) {
        println!(
            "The {} chapter {} is available on at: {}",
            self.title,
            self.chapter,
            self.last_manga.as_ref().unwrap(),
        );
        println!("--------------------------------");
    }
}

//create the basic manga list with some entries
fn build_manga_list() -> Vec<Manga> {
    let scrap_list = vec![
        Manga {
            title: "Tower of God".to_string(),
            base_scrap_link: String::from("https://alpha-scans.org/tower-of-god-chapter-"),
            chapter: 548,
            manga_site: Mangasite::Alphascans,
            last_manga: (None),
        },
        Manga {
            title: "Omniscient Reader’s Viewpoint Chapter".to_string(),
            base_scrap_link: String::from(
                "https://flamescans.org/1656345662-omniscient-readers-viewpoint-chapter-",
            ),
            chapter: 112,
            manga_site: Mangasite::Flamescans,
            last_manga: (None),
        },
        Manga {
            title: String::from("A Returner’s Magic Should be Special"),
            base_scrap_link: String::from(
                "https://cosmicscans.com/a-returners-magic-should-be-special-chapter-",
            ),
            chapter: 194,
            manga_site: Mangasite::Cosmicscans,
            last_manga: (None),
        },
        Manga {
            title: String::from("Mashle: Magic and Muscles"),
            base_scrap_link: String::from("https://mangaplus.shueisha.co.jp/titles/100083"),
            chapter: 113,
            manga_site: Mangasite::Mangaplus,
            last_manga: (None),
        },
        Manga {
            title: String::from("Boku no Hero"),
            base_scrap_link: String::from("https://mangaplus.shueisha.co.jp/titles/100017"),
            chapter: 357,
            manga_site: Mangasite::Mangaplus,
            last_manga: (None),
        },
        Manga {
            title: String::from("Jujutsu Kaisen"),
            base_scrap_link: String::from("https://mangaplus.shueisha.co.jp/titles/100034"),
            chapter: 189,
            manga_site: Mangasite::Mangaplus,
            last_manga: (None),
        },
        Manga {
            title: String::from("One Piece"),
            base_scrap_link: String::from("https://mangaplus.shueisha.co.jp/titles/100020"),
            chapter: 1053,
            manga_site: Mangasite::Mangaplus,
            last_manga: (None),
        },
        Manga {
            title: String::from("Kaiju Monster #8"),
            base_scrap_link: String::from("https://mangaplus.shueisha.co.jp/titles/100110"),
            chapter: 65,
            manga_site: Mangasite::Mangaplus,
            last_manga: (None),
        },
    ];
    scrap_list
}

//scrap all elements of the list
fn scrap_all(scrap_list: &mut Vec<Manga>) {
    for mut element in scrap_list {
        element.scrap_manga();
    }
}

// fn add_chapter_to_all(manga_list: &mut Vec<Manga>){
//     for element in manga_list {
//         element.add_chapter();
//     }
// }


fn main() {
    println!("Welcome to the manga scrapper");
    println!("--------------------------------");
    let mut manga_list = build_manga_list();
    //add_chapter_to_all(&mut manga_list);

    scrap_all(&mut manga_list);
}

//print variable type
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
