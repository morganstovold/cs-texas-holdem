// this is the game master for our poker game
// it will keep track of games and players in each game
// each player should have a name and current balance aswell as current hand which contains their 2 cards
// each game object should keep track of each player in the game, current pot, aswell as the games flop, turn, and river

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

