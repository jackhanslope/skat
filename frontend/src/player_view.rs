use skat::game;
use yew::prelude::*;
use yew::services::ConsoleService;

pub struct PlayerView {
    player: game::Player,
    name: String,
    link: ComponentLink<Self>,
    console: ConsoleService,
}

pub enum Msg {
    NewDeck,
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
            player: game::Player { hand: Vec::new() },
            name: props.name,
            link: link,
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewDeck => {
                self.player.hand = Vec::new();
                self.console.log("New Hand");
            }
        }
        true
    }

    fn view(&self) -> Html {
        let hand = serde_json::to_string(&self.player).unwrap();
        html! {
            <div>
                <h2> { &self.name } </h2>
                <nav class="menu">
                    <button onclick=self.link.callback(|_| Msg::NewDeck)>
                        { "New Hand" }
                    </button>
                </nav>
                <p>{ hand }</p>
            </div>
        }
    }
}
