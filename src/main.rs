use reqwest;
use scraper;


//Struct with base info ( title, link, current chapter)

struct Manga {
    title: String,
    base_scrap_link: String,
    chapter: u16,

}
//print variable type
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


//create Manga Struct
fn create_manga(title: String, link:String, chapter_number:u16) -> Manga{
    let tmp_manga : Manga = Manga {
        title: title,
        base_scrap_link: link,
        chapter: chapter_number,
    };

    tmp_manga
}

//create the basic manga list with some entries
fn base_manga_list () -> Vec<Manga>{

    let mut scrap_list: Vec<Manga> = Vec::new();

    scrap_list.push({
        create_manga(
            "Tower of God".to_string(),
            "https://alpha-scans.org/tower-of-god-chapter-".to_string(),
            548
        )
    });

    scrap_list
}

fn check_valid_response(response:&Result<reqwest::blocking::Response, reqwest::Error>) -> bool {

    match response {
        Ok(response) => true,
        Err(_) => false,
        }
}
//check if manga is released
fn check_valid_manga (html:&String) -> bool{

    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse("div.chdesc>p").expect("Invalid css selector");

    for element in document.select(&selector){
        return true;
    }

    // funciona mas é um iterator, uma array seria melhor para checar as ocorrencias
    //valid_manga.for_each(| item | println!("{}", item));
    
    //println!("{}",valid_manga);

    false

}
//scrap the webpage of a single manga
fn scrap_manga(manga:Manga){
    let response = reqwest::blocking::get(manga.base_scrap_link+&manga.chapter.to_string());

    //test if page is valid
    //Test if had an error return

    if check_valid_response(&response){
        //convert response to 
    } else {
        println!("Error on response");
        return;
    }

    let response = &response
    .expect("Invalid Response")
    .text()
    .expect("Invalid text method");

    if check_valid_manga(response){
        println!("Manga Found!")
    } else {
        println!("No manga found!")
    }

}


//scrap all elements of the list
fn scrap_all(scrap_list:Vec<Manga>){
    for element in scrap_list{
        scrap_manga(element);
    }
}

fn main() {
    println!("Welcome to the manga scrapper");
    println!("--------------------------------");
    scrap_all(base_manga_list());
}
