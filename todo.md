# TODO

The game lib
* rename or refactor some things. I think it's weird that `Round` is defined in
 `game.rs` kinda like tennis's game,set,match we have trick,round,game
* All other available actions.
* Score a finished round base on state.


The backend
* the `Game` type should probably live in the lib. It could be a class with
  lots of helper methods or more similar to how round is.
* Need an available actions endpoint
* Some better way of dealing with errors, probably need return types. e.g.

 ```
 { "response_code": null, "error_msg": null, data: ["game_id": 12345]}
 { "response_code": 1, "error_msg": "Game Id doesn't exist in database. Create a game first." , data: null}
 ```

 * Kinda need sse or ws endpoint that will push available actions and apply_action changes to the clients. If we don't clients need to poll in loop (or ask user to refresh) which might appear laggy?

The frontend
* figure out to read host from environment probably
* maybe follow some sort of framework layout? like [here](https://github.com/jetli/rust-yew-realworld-example-app/tree/master/crates/conduit-wasm/src)
* not all the buttons need to be visable all the time
* like css and stuff maybe
* I don't really know how this works
