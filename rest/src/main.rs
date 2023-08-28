use actix_web::{web, App, HttpServer, Responder};
use actix_rt;
use reqwest;
use serde::Deserialize;


const MAGIC_8_BALL_ANSWERS: [&str; 20] = [
    "Yes.",
    "No.",
    "Maybe.",
    "It is certain.",
    "Ask again later.",
    "Cannot predict now.",
    "Better not tell you now.",
    "Don't count on it.",
    "My sources say no.",
    "Outlook not so good.",
    "Very doubtful.",
    "You may rely on it.",
    "Most likely.",
    "Signs point to yes.",
    "Reply hazy, try again.",
    "Concentrate and ask again.",
    "My reply is no.",
    "Yes, definitely.",
    "Without a doubt.",
    "As I see it, yes.",
];
const FIRST_NAMES: [&str; 10] = [
    "Liam", "Olivia", "Noah", "Emma", "Sophia", "Isabella", "Ava", "Mia", "Luna", "Ethan",
];

const LAST_NAMES: [&str; 10] = [
    "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis", "Rodriguez", "Martinez",
];
#[derive(Deserialize)]
struct SearchRequest {
    q: String,
    maxResults: Option<u32>,
}
#[derive(Deserialize)]
struct YouTubeResponse {
    items: Vec<YouTubeVideo>,
}
#[derive(Deserialize)]
struct YouTubeVideo {
    id: YouTubeVideoId,
}
#[derive(Deserialize)]
struct YouTubeVideoId {
    videoId: String,
}
#[derive(Serialize)]
struct VideoResponse {
    videoLink: String,
}
#[derive(Deserialize)]
struct GoogleSearchItem {
    title: String,
    link: String,
}
#[derive(Deserialize)]
struct GoogleSearchResponse {
    items: Vec<GoogleSearchItem>,
}
#[derive(Serialize)]
struct SearchResult {
    title: String,
    link: String,
}
#[derive(Deserialize)]
struct QuoteResponse {
    contents: QuoteContents,
}

#[derive(Deserialize)]
struct QuoteContents {
    quotes: Vec<Quote>,
}

#[derive(Deserialize)]
struct Quote {
    quote: String,
}
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}
#[derive(serde::Deserialize)]
struct CatApiResponse {
    file: String,
}
async fn hello_world() -> impl Responder {
    "Hello World From Rust Resp Api !\nMade with love By `abdulkarim` "
}
async fn random_cat_picture() -> impl Responder {
    let url = "https://aws.random.cat/meow";

    match reqwest::get(url).await {
        Ok(response) => {
            let cat_response: CatApiResponse = response.json().await.unwrap();
            let cat_picture_url = cat_response.file;

            web::Json(cat_picture_url)
        }
        Err(_) => web::Json(ErrorResponse {
            error: "An error occurred while fetching a cat picture.".to_owned(),
        }),
    }
}
async fn quote_of_the_day() -> impl Responder {
    let url = "https://quotes.rest/qod?category=inspire";

    match reqwest::get(url).await {
        Ok(response) => {
            let quote_response: QuoteResponse = response.json().await.unwrap();
            let quote = quote_response.contents.quotes[0].quote;

            web::Json(quote)
        }
        Err(_) => web::Json(ErrorResponse {
            error: "An error occurred while fetching the quote.".to_owned(),
        }),
    }
}

async fn google_search(search_params: web::Query<SearchRequest>) -> impl Responder {
    let search_query = &search_params.q;

    println!("Received Google search request: q={}", search_query);

    let url = format!(
        "https://www.googleapis.com/customsearch/v1?q={}&key=YOUR_GOOGLE_API_KEY&cx=AIzaSyBoIr75bm_nhYRx9FYCeoXOpyvKRbBYdM0",
        search_query
    );
    match reqwest::get(&url).await {
        Ok(response) => {
            let google_response: GoogleSearchResponse = response.json().await.unwrap();
            let search_results = google_response
                .items
                .into_iter()
                .map(|item| SearchResult {
                    title: item.title,
                    link: item.link,
                })
                .collect::<Vec<_>>();

            web::Json(search_results)
        }
        Err(_) => web::Json(ErrorResponse {
            error: "An error occurred while performing the Google search.".to_owned(),
        }),
    }
}
async fn youtube_video(search_params: web::Query<SearchRequest>) -> impl Responder {
    let search_query = &search_params.q;
    let max_results = search_params.maxResults.unwrap_or(1);
    println!("Received YouTube video search request: q={}", search_query);
    let url = format!(
        "https://www.googleapis.com/youtube/v3/search?part=snippet&maxResults={}&q={}&type=video&key=AIzaSyAROmuYBMXG4w6VZelKi9sAjO04Ig_FAiQ",
        max_results, search_query
    );
    match reqwest::get(&url).await {
        Ok(response) => {
            let youtube_response: YouTubeResponse = response.json().await.unwrap();
            if let Some(video_id) = youtube_response.items.get(0).map(|item| &item.id.videoId) {
                let video_link = format!("https://www.youtube.com/watch?v={}", video_id);
                println!("Video ID: {}", video_id);
                println!("Video Link: {}", video_link);
                web::Json(VideoResponse { videoLink: video_link })
            } else {
                web::Json(ErrorResponse {
                    error: "No videos found.".to_owned(),
                })
            }
        }
        Err(_) => web::Json(ErrorResponse {
            error: "An error occurred while searching YouTube.".to_owned(),
        }),
    }
}
async fn name_generator() -> impl Responder {
    let random_first_name = FIRST_NAMES.choose(&mut rand::thread_rng()).unwrap();
    let random_last_name = LAST_NAMES.choose(&mut rand::thread_rng()).unwrap();
    let full_name = format!("{} {}", random_first_name, random_last_name);

    web::Json(full_name)
}
async fn magic_8_ball() -> impl Responder {
    let random_answer = MAGIC_8_BALL_ANSWERS.choose(&mut rand::thread_rng()).unwrap();
    web::Json(random_answer.to_string())
}
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello_world))
            .route("/youtube", web::get().to(youtube_video))
            .route("/search", web::get().to(google_search))
            .route("/dayqoute", web::get().to(google_search))
            .route("/autoname",web::get().to(name_generator))
            .route("/cat", web::get().to(random_cat_picture))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
#[actix_rt::main]
async fn start() -> std::io::Result<()> {
    main().await
}