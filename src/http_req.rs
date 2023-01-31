extern crate reqwest;

// fn long_way_http() {
//     match reqwest::get("url") {
//         Ok(mut response) => {
//             match response.text() {
//                 Ok(text) => println!("{}", text),
//                 Err(_) => println!("Error getting text"),
//             }
//         },
//         Err(_) => println!("Error getting response"),
//     }
// }

async fn short_way_http() {
    let text = reqwest::get("url").await.unwrap().text().await.unwrap(); //unwrap is a macro that returns the value if the result is Ok, otherwise it panics
    println!("{}", text);
}