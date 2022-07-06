use failure;
use headless_chrome::Browser;
use scraper;
use std::iter::zip;

// Name your user agent for browser
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

//Struct with base info ( title, link, current chapter)


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
    for element in scrap_list {
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
