use animanga_scrapper;

#[cfg(test)]

fn build_manga() {
    let manga_from_new = animanga_scrapper::new(
        "Tower of God".to_string(),
        "https://alpha-scans.org/tower-of-god-chapter-".to_string,
        548,
        548,
        Some(122663),
        Mangasite::Alphascans,
        None,
    );

    let usual_manga = animanga_scrapper::Manga{
        title: "Tower of God".to_string(),
        base_scrap_link: "https://alpha-scans.org/tower-of-god-chapter-".to_string,
        current_chapter: 548,
        last_chapter: 548,
        animelistid:Some(122663),
        manga_site: Mangasite::Alphascans,
        last_manga_link: None,
    };
    assert_eq!{manga_from_new,usual_manga};
}
