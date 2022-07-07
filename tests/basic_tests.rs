use animanga_scrapper;

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
