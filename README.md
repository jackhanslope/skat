# skat

Skat is a German trick taking card game with 3 players and an optional dealer.


## The Basics

The game is played with `J 10 A K Q 9 8 7` in 4 suits for a total of 32 cards.
At the start of the game these are dealt to the players with each receiving 10
and the remaining 2 becoming the "skat". After the deal there is a bidding
phase that decides which player will be playing alone against the other 2. The
alone player may ( or may not ) switch out some of their cards with the ones in
the skat, and then declare a game (the alone players is often called the
declarer). After a game has been announced play proceeds with the player to the
left of the dealer. Depending on the game there may or may not be trump cards,
and the declarer may or may not need to to win tricks to win the hand.

## Design Spec


### Server Side

* needs to store current game state
* assign limited state to each player (deal the cards)
* make limited state available to each player. Either ws or endpoint
* calculate allowed moves for each player

### Client Side

* make a new game request
* allow players to join game
* make request or receive limited state for each player
* display state
