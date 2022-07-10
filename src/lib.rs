use failure;
use headless_chrome::Browser;
use rusqlite::{params, Connection, Result, ToSql};
use scraper;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::iter::zip;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
pub static DATABASE_PATH: &str = "./data/manga_db.sqlite3";

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

impl fmt::Display for Mangasite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mangasite::Alphascans => write!(f, "Alphascans"),
            Mangasite::Cosmicscans => write!(f, "Cosmicscans"),
            Mangasite::Flamescans => write!(f, "Flamescans"),
            Mangasite::Mangaplus => write!(f, "Mangaplus"),
        }
    }
}

impl rusqlite::ToSql for Mangasite {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(self.to_string()))
    }
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

    pub fn browse_mangaplus(&self) -> Result<Vec<MangaplusReference>, failure::Error> {
        let mut mangaplus_reference_vector: Vec<MangaplusReference> = Vec::new();

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
                    chapter = element.node_value.replace("#", "").parse().unwrap_or(0);
                }
            }
            mangaplus_reference_vector.push(MangaplusReference {
                manga_chapter: chapter,
                manga_id: id,
            })
        }
        Ok(mangaplus_reference_vector)
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

    pub fn scrap_manga(&self) {
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

pub fn basic_manga_list() -> Vec<Manga> {
    vec![
        Manga {
            title: "Tower of God".to_string(),
            base_scrap_link: String::from("https://alpha-scans.org/tower-of-god-chapter-"),
            current_chapter: 548,
            last_chapter: 548,
            animelist_id: Some(122663),
            manga_site: Mangasite::Alphascans,
        },
        Manga {
            title: "Omniscient Reader’s Viewpoint Chapter".to_string(),
            base_scrap_link: String::from(
                "https://flamescans.org/1656345662-omniscient-readers-viewpoint-chapter-",
            ),
            current_chapter: 112,
            last_chapter: 112,
            animelist_id: Some(132214),
            manga_site: Mangasite::Flamescans,
        },
        Manga {
            title: String::from("A Returner’s Magic Should be Special"),
            base_scrap_link: String::from(
                "https://cosmicscans.com/a-returners-magic-should-be-special-chapter-",
            ),
            current_chapter: 194,
            last_chapter: 194,
            animelist_id: Some(132247),
            manga_site: Mangasite::Cosmicscans,
        },
        Manga {
            title: String::from("Mashle: Magic and Muscles"),
            base_scrap_link: String::from("https://mangaplus.shueisha.co.jp/titles/100083"),
            current_chapter: 113,
            last_chapter: 113,
            animelist_id: Some(124085),
            manga_site: Mangasite::Mangaplus,
        },
        Manga {
            title: String::from("Boku no Hero"),
            base_scrap_link: String::from("https://mangaplus.shueisha.co.jp/titles/100017"),
            current_chapter: 357,
            last_chapter: 357,
            animelist_id: Some(75989),
            manga_site: Mangasite::Mangaplus,
        },
        Manga {
            title: String::from("Jujutsu Kaisen"),
            base_scrap_link: String::from("https://mangaplus.shueisha.co.jp/titles/100034"),
            current_chapter: 189,
            last_chapter: 189,
            animelist_id: Some(113138),
            manga_site: Mangasite::Mangaplus,
        },
        Manga {
            title: String::from("One Piece"),
            base_scrap_link: String::from("https://mangaplus.shueisha.co.jp/titles/100020"),
            current_chapter: 1053,
            last_chapter: 1053,
            animelist_id: Some(13),
            manga_site: Mangasite::Mangaplus,
        },
        Manga {
            title: String::from("Kaiju Monster #8"),
            base_scrap_link: String::from("https://mangaplus.shueisha.co.jp/titles/100110"),
            current_chapter: 65,
            last_chapter: 65,
            animelist_id: Some(127907),
            manga_site: Mangasite::Mangaplus,
        },
    ]
}

pub fn run_db() -> Result<(), Box<dyn Error>> {
    let db_conn = Connection::open(DATABASE_PATH)?;
    create_table(&db_conn);
    Ok(())
}

pub fn create_table(conn: &rusqlite::Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE manga_db (
                        title                   TEXT NOT NULL PRIMARY KEY,
                        base_scrap_link         TEXT NOT NULL,
                        current_chapter         INTEGER,
                        last_chapter            INTEGER,
                        animelist_id            INTEGER,
                        manga_site              TEXT NOT NULL
                        )",
            []
    )?;
    Ok(())
}

pub fn add_mangas_db(conn: rusqlite::Connection, list: Vec<Manga>) -> Result<()> {
    create_table(&conn).unwrap_or(());
    for manga in list {
        conn.execute(
            "INSERT INTO manga_db 
        (title, base_scrap_link, current_chapter, last_chapter, animelist_id, manga_site) 
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                manga.title,
                manga.base_scrap_link,
                manga.current_chapter,
                manga.last_chapter,
                manga.animelist_id,
                manga.manga_site
            ],
        )?;
    }
    Ok(())
}

pub fn add_manga_db(conn: rusqlite::Connection, manga: Manga) -> Result<()> {
    create_table(&conn).unwrap_or(());
    conn.execute(
        "INSERT INTO manga_db 
    (title, base_scrap_link, current_chapter, last_chapter, animelist_id, manga_site) 
    VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            manga.title,
            manga.base_scrap_link,
            manga.current_chapter,
            manga.animelist_id,
            manga.last_chapter,
            manga.manga_site
        ],
    )?;

    Ok(())
}
