use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article {
        article: String::from("how to work with json in rust"),
        author: String::from("Dipak"),
        paragraph: vec![
            Paragraph {
                name: String::from("first sentence")
            },
            Paragraph {
                name: String::from("body of sentence")
            },
            Paragraph {
                name: String::from("end of sentence")
            },
        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("The json is {:?}", json);
}