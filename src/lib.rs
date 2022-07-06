enum Mangasite {
    Flamescans,
    Alphascans,
    Cosmicscans,
    Mangaplus,
}

pub struct Manga {
    pub title: String,
    pub base_scrap_link: String,
    pub current_chapter:u16,
    pub last_chapter: u16,
    pub animelistid: Option<u16>,
    pub manga_site: Mangasite,
    pub last_manga_link: Option<String>,
}

impl Manga {

    // fn add_chapter(&mut self){
    //     self.chapter += 1;
    // }

    //fn to browse mangaplus available chapters and return chapter number + chapter comments(for reader id)
    pub fn new(
        title:String,
        base_scrap_link:String,
        current_chapter:u16,
        last_chapter:u16,
        animelistid:Option<u16>,
        manga_site: Mangasite,
        last_manga_link:Option<String>,
    ) -> Manga{
        Manga{
            title,
            base_scrap_link,
            current_chapter,
            last_chapter,
            animelistid,
            manga_site,
            last_manga_link,

        }

    }

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