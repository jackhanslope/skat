use yew::prelude::*;

use crate::game_player_view::GamePlayerView;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="main">
                <h1>{ "Skat" }</h1>
                <GamePlayerView name="Alice"/>
                <GamePlayerView name="Bob"/>
                <GamePlayerView name="Eve"/>
            </div>
        }
    }
}
