use skat::card::Card;
use yew::prelude::*;
use crate::config::HOST;
use reqwest;

pub struct PlayerView {
    hand: [Option<Card>; 10],
    game_id: u32,
    player_id: u32,
    name: String,
    link: ComponentLink<Self>,
}

pub enum Msg {
    NewGame(Result<u32, reqwest::Error>),
    NewGameRequest,
    JoinGame(Result<u32, reqwest::Error>),
    JoinGameRequest(Option<u32>),
    NewRound(Result<bool, reqwest::Error>),
    NewRoundRequest,
    GetHand(Result<[Option<Card>; 10], reqwest::Error>),
    GetHandRequest,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
}

impl Component for PlayerView {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        PlayerView {
            hand: [None; 10],
            game_id: 0,
            player_id: 0,
            name: props.name,
            link: link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewGameRequest => {
                let callback = self.link.callback(Msg::NewGame);
                wasm_bindgen_futures::spawn_local( async move {
                    PlayerView::new_game_request(callback).await;
                });
            }
            Msg::NewGame(Ok(game_id)) => { 
                self.game_id = game_id;
                self.link.callback(Msg::JoinGameRequest).emit(None);
            }
            Msg::NewGame(Err(_)) => (), // TODO

            Msg::JoinGameRequest(None) => 
                self.link.callback(Msg::JoinGameRequest).emit(Some(self.game_id)),
            Msg::JoinGameRequest(Some(game_id)) => {
                let callback = self.link.callback(Msg::JoinGame);
                self.game_id = game_id;
                wasm_bindgen_futures::spawn_local( async move {
                    PlayerView::join_game_request(game_id, callback).await;
                });
            }
            Msg::JoinGame(Ok(player_id)) => {
                self.player_id = player_id;
                return true;
            }
            Msg::JoinGame(Err(_)) => (), // TODO

            Msg::NewRoundRequest => {
                let callback = self.link.callback(Msg::NewRound);
                let game_id = self.game_id;
                wasm_bindgen_futures::spawn_local( async move {
                    PlayerView::new_round_request(game_id, callback).await;
                });
            }
            Msg::NewRound(Ok(true)) => (),
            Msg::NewRound(Ok(false)) => (), //TODO
            Msg::NewRound(Err(_)) => (), // TODO

            Msg::GetHandRequest => {
                let callback = self.link.callback(Msg::GetHand);
                let game_id = self.game_id;
                let player_id = self.player_id;
                wasm_bindgen_futures::spawn_local( async move {
                    PlayerView::get_player_hand_request(game_id, player_id, callback).await;
                });
            }
            Msg::GetHand(Ok(hand)) => {
                self.hand = hand;
                return true;
            }
            Msg::GetHand(Err(_)) => (), // TODO
        }
        return false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let hand = serde_json::to_string(&self.hand).unwrap();
        html! {
            <div>
                <h2> { &self.name } </h2>
                <nav class="menu">
                    <button onclick=self.link.callback(|_| Msg::NewGameRequest)>
                        { "New Game" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::NewRoundRequest)>
                        { "New Round" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::GetHandRequest)>
                        { "Get Hand" }
                    </button>
                    <input type="text" oninput=self.link.callback(|e: yew::InputData| Msg::JoinGameRequest(Some(e.value.parse::<u32>().unwrap())))/>
                </nav>
                <p>{ format!("Game Id: {}", self.game_id) }</p>
                <p>{ format!("Player Id: {}", self.player_id) }</p>
                <p>{ format!("Hand: {}", hand) }</p>
            </div>
        }
    }
}

impl PlayerView {
    async fn new_game_request(then: Callback<Result<u32, reqwest::Error>>) {
        match reqwest::Client::new().post(&format!("{}/api/game", HOST)).send().await {
            Err(err) => then.emit(Err(err)),
            Ok(res) => {
                match res.text().await.unwrap().parse::<u32>() {
                    Err(_) => (), // TODO
                    Ok(id) => then.emit(Ok(id)),
                }
            }
        }
    }
    
    async fn join_game_request(game_id: u32, then: Callback<Result<u32, reqwest::Error>>) {
        let url = format!("{}/api/game/{}/join", HOST, game_id);
        match reqwest::Client::new().post(&url).send().await {
            Err(err) => then.emit(Err(err)),
            Ok(res) => {
                match res.text().await.unwrap().parse::<u32>() {
                    Err(_) => (), // TODO
                    Ok(id) => then.emit(Ok(id)),
                }
            }
        }
    }

    async fn new_round_request(game_id: u32, then: Callback<Result<bool, reqwest::Error>>) {
        let url = format!("{}/api/game/{}/round", HOST, game_id);
        match reqwest::Client::new().post(&url).send().await {
            Err(err) => then.emit(Err(err)),
            Ok(_) => then.emit(Ok(true)),
        }
    }

    async fn get_player_hand_request(game_id: u32, player_id: u32, then: Callback<Result<[Option<Card>; 10], reqwest::Error>>) {
        let url = format!("{}/api/game/{}/round?player_id={}", HOST, game_id, player_id);
        match reqwest::Client::new().get(&url).send().await {
            Err(err) => then.emit(Err(err)),
            Ok(res) => {
                let hand = serde_json::from_str(&res.text().await.unwrap()).unwrap();
                then.emit(Ok(hand));
            }
        }
    }
}
