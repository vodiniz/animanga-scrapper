use animanga_scrapper;
use rusqlite::Connection;
use std::error::Error;

#[test]

fn build_manga() {
    let manga_from_new = animanga_scrapper::Manga::new(
        "Tower of God".to_string(),
        "https://alpha-scans.org/tower-of-god-chapter-".to_string(),
        548,
        548,
        Some(122663),
        animanga_scrapper::Mangasite::Alphascans,
    );

    let usual_manga = animanga_scrapper::Manga {
        title: "Tower of God".to_string(),
        base_scrap_link: "https://alpha-scans.org/tower-of-god-chapter-".to_string(),
        current_chapter: 548,
        last_chapter: 548,
        animelist_id: Some(122663),
        manga_site: animanga_scrapper::Mangasite::Alphascans,
    };
    assert_eq! {manga_from_new,usual_manga};
}
#[test]
fn manga_link() {
    let manga = animanga_scrapper::Manga::new(
        "Tower of God".to_string(),
        "https://alpha-scans.org/tower-of-god-chapter-".to_string(),
        548,
        548,
        Some(122663),
        animanga_scrapper::Mangasite::Alphascans,
    );

    assert_eq!(
        manga.get_last_manga(),
        "https://alpha-scans.org/tower-of-god-chapter-548"
    )
}

#[test]
fn test_browse_mangaplus() {
    let tmp = animanga_scrapper::Manga {
        title: String::from("One Piece"),
        base_scrap_link: String::from("https://mangaplus.shueisha.co.jp/titles/100020"),
        current_chapter: 1053,
        last_chapter: 1053,
        animelist_id: Some(13),
        manga_site: animanga_scrapper::Mangasite::Mangaplus,
    };
    // for chapter in tmp.browse_mangaplus() {
    //     dbg!(chapter);
    // }
}

#[test]
fn mangaplus_last_manga_link() {
    let tmp = animanga_scrapper::Manga {
        title: String::from("One Piece"),
        base_scrap_link: String::from("https://mangaplus.shueisha.co.jp/titles/100020"),
        current_chapter: 1053,
        last_chapter: 1053,
        animelist_id: Some(13),
        manga_site: animanga_scrapper::Mangasite::Mangaplus,
    };
    assert_eq!(
        tmp.get_last_manga(),
        "https://mangaplus.shueisha.co.jp/viewer/1013489"
    );
}
#[test]
fn scrap_manga() {
    let tmp = animanga_scrapper::Manga {
        title: String::from("One Piece"),
        base_scrap_link: String::from("https://mangaplus.shueisha.co.jp/titles/100020"),
        current_chapter: 1053,
        last_chapter: 1053,
        animelist_id: Some(13),
        manga_site: animanga_scrapper::Mangasite::Mangaplus,
    };
    tmp.scrap_manga();
}

#[test]
fn create_db_table() {
    animanga_scrapper::run_db();
}

#[test]
fn add_manga_to_db() -> Result<(), Box<dyn Error>> {
    let db_conn = Connection::open(animanga_scrapper::DATABASE_PATH)?;
    let manga_list = animanga_scrapper::basic_manga_list();
    let test = animanga_scrapper::add_mangas_db(db_conn, manga_list);
    dbg!(&test);
    Ok(())
}
