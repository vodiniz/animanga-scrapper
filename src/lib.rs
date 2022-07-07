use failure;
use headless_chrome::Browser;
use scraper;
use std::iter::zip;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Debug)]
pub struct MangaplusReference {
    pub manga_chapter: u16,
    pub manga_id: u32,
}

#[derive(Debug, PartialEq)]
pub enum Mangasite {
    Alphascans,
    Cosmicscans,
    Flamescans,
    Mangaplus,
}

#[derive(Debug, PartialEq)]
pub struct Manga {
    pub title: String,
    pub base_scrap_link: String,
    pub current_chapter: u16,
    pub last_chapter: u16,
    pub animelist_id: Option<u32>,
    pub manga_site: Mangasite,
}

impl Manga {
    // fn add_chapter(&mut self){
    //     self.chapter += 1;
    // }

    //fn to browse mangaplus available chapters and return chapter number + chapter comments(for reader id)
    pub fn new(
        title: String,
        base_scrap_link: String,
        current_chapter: u16,
        last_chapter: u16,
        animelist_id: Option<u32>,
        manga_site: Mangasite,
    ) -> Manga {
        Manga {
            title,
            base_scrap_link,
            current_chapter,
            last_chapter,
            animelist_id,
            manga_site,
        }
    }

    fn browse_mangaplus(&self) -> Result<Vec<MangaplusReference>, failure::Error> {
        let mut MangaplusReference_vector: Vec<MangaplusReference> = Vec::new();

        let browser = Browser::default()?;
        let tab = browser.wait_for_initial_tab()?;
        tab.set_default_timeout(std::time::Duration::from_secs(20));

        tab.navigate_to(&self.base_scrap_link)?;
        let contents = tab.wait_for_elements("p.ChapterListItem-module_name_3h9dj")?;

        let chapter_link =
            tab.wait_for_elements("a.ChapterListItem-module_commentContainer_1P6qt")?;

        for (content, link) in zip(contents, chapter_link) {
            let mut chapter: u16 = 0;
            let mut id: u32 = 0;
            for attribute in link.get_attributes()? {
                let comments = attribute.get("href").unwrap().to_string();
                let split_comments: Vec<String> =
                    comments.split("/").map(|s| s.to_string()).collect();
                id = split_comments[2]
                    .parse::<u32>()
                    .expect("Error unwraping chapter id from mangaplus");
            }

            for content_children in content.get_description()?.children {
                for element in content_children {
                    chapter = element.node_value.
                        replace("#","")
                        .parse().unwrap_or(0);
                }
            }
            MangaplusReference_vector.push(MangaplusReference {
                manga_chapter: chapter,
                manga_id: id,
            })
        }
        Ok(MangaplusReference_vector)
    }

    pub fn get_last_manga(&self) -> String {
        match self.manga_site {
            Mangasite::Alphascans => {
                format!("{}{}", self.base_scrap_link, self.last_chapter.to_string())
            }
            Mangasite::Cosmicscans => {
                format!("{}{}", self.base_scrap_link, self.last_chapter.to_string())
            }
            Mangasite::Flamescans => {
                format!("{}{}", self.base_scrap_link, self.last_chapter.to_string())
            }
            Mangasite::Mangaplus => {
                let mut mangaplus = self
                    .browse_mangaplus()
                    .expect("Invalid Mangaplus Reference");
                mangaplus.sort_by_key(|vec| vec.manga_chapter);
                let last = mangaplus.last().expect("Empty Vector!");

                format!(
                    "{}{}",
                    "https://mangaplus.shueisha.co.jp/viewer/",
                    last.manga_id.to_string()
                )
            }
        }
    }

    pub fn scrap_manga(&mut self) {
        let client = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .expect("Error building client");

        match &self.manga_site {
            Mangasite::Alphascans => {
                let response = client.get(self.get_last_manga()).send();

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
                let response = client.get(self.get_last_manga()).send();

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
                let response = client.get(self.get_last_manga()).send();

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
                Ok(mangaplus_vec) => {
                    for reference in mangaplus_vec {
                        if reference.manga_chapter == self.last_chapter {
                            self.send_message();
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
            self.last_chapter,
            self.get_last_manga(),
        );
        println!("--------------------------------");
    }
}
