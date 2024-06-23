use card::card::Card;
use eval::Eval;
use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};

mod card;
mod eval;
mod hand_ranks;

#[derive(Serialize, Deserialize)]
struct GetResponse {
    hand: Vec<String>,
    rank: String,
}

#[derive(Serialize, Deserialize)]
struct PostResponse {
    rank: String,
}

#[derive(Serialize, Deserialize)]
struct PostRequest {
    cards: Vec<Card>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/api/v1/hand").get(|_| async {
        let cards: Vec<Card> = Card::new_hand();

        let cards_as_string_array: Vec<String> = cards.iter().map(|c| c.to_string()).collect();
        let eval = Eval::from(cards);

        match eval.evaluate() {
            Ok(v) => Body::from_json(&GetResponse {
                hand: cards_as_string_array,
                rank: v.to_string(),
            }),
            Err(_) => Err(tide::Error::from_str(
                500,
                "Interal server error, sorry about that",
            )),
        }
    });

    app.at("/api/v1/hand")
        .post(|mut request: Request<()>| async move {
            let Ok(req) = request.body_json::<PostRequest>().await else {
                return Ok(Response::new(400));
            };

            let eval = Eval::from(req.cards);
            match eval.evaluate() {
                Ok(v) => {
                    let mut res = Response::new(200);
                    res.set_body(Body::from_json(&PostResponse {
                        rank: v.to_string(),
                    })?);
                    Ok(res)
                }
                Err(e) => {
                    let mut res = Response::new(400);
                    res.set_body(e.to_string());
                    Ok(res)
                }
            }
        });

    app.listen("0.0.0.0:3000").await?;
    Ok(())
}
