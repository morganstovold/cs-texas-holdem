use futures::lock::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use tokio_tungstenite::tungstenite::protocol::Message;
use uuid::Uuid;
use warp::Filter;
use rand::seq::SliceRandom;

struct Game {
    tx: tokio::sync::broadcast::Sender<Message>,
    players: HashMap<Uuid, Player>,
    deck: Vec<String>,
    big_blind: u32,
    small_blind: u32,
    pot: u32,
    current_bet: u32,
    current_player: Uuid,
}

impl Game {
    fn new (tx: tokio::sync::broadcast::Sender<Message>) -> Self {
        Self {
            tx,
            players: HashMap::new(),
            deck: vec![],
            big_blind: 0,
            small_blind: 0,
            pot: 0,
            current_bet: 0,
            current_player: Uuid::new_v4(),
        }
    }

    fn start_game(&mut self) {
        self.deck = create_deck();
        self.start_betting_round();
        println!("Starting game");
    }

    fn add_player(&mut self, player: Player) {
        self.players.insert(player.id, player);
    }

    fn start_betting_round(&self) {
        println!("Starting betting round");
    }
}

struct Player {
    id: Uuid,
    name: String,
    ws: warp::ws::WebSocket,
}

#[tokio::main]
async fn main() {
    let games: Arc<Mutex<HashMap<Uuid, Game>>> = Arc::new(Mutex::new(HashMap::new()));

    let create_game = warp::path("create_game")
        .and(warp::post())
        .and(with_games(games.clone()))
        .and_then(handle_create_game);

    let join_game = warp::path!("join_game" / Uuid)
        .and(warp::ws())
        .and(with_games(games.clone()))
        .and_then(handle_join_game);

    let routes = create_game.or(join_game);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_games(
    games: Arc<Mutex<HashMap<Uuid, Game>>>,
) -> impl Filter<Extract = (Arc<Mutex<HashMap<Uuid, Game>>>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || games.clone())
}

async fn handle_create_game(
    games: Arc<Mutex<HashMap<Uuid, Game>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let game_id = Uuid::new_v4();
    let (tx, _rx) = tokio::sync::broadcast::channel(9);

    games.lock().await.insert(game_id, Game::new(tx));

    Ok(warp::reply::json(&game_id))
}

async fn handle_join_game(
    game_id: Uuid,
    ws: warp::ws::Ws,
    games: Arc<Mutex<HashMap<Uuid, Game>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(game) = games.lock().await.get_mut(&game_id) {
        let player_id = Uuid::new_v4();
        let player_name = format!("Player {}", player_id);

        let games_clone = games.clone();
        Ok(ws.on_upgrade(move |socket| handle_websocket(socket, game_id, player_id, player_name, games_clone)))
    } else {
        Err(warp::reject::not_found())
    }
}

async fn handle_websocket(
    ws: warp::ws::WebSocket,
    game_id: Uuid,
    player_id: Uuid,
    player_name: String,
    games: Arc<Mutex<HashMap<Uuid, Game>>>,
) {
    let player = Player {
        id: player_id,
        name: player_name,
        ws: ws,
    };

    if games.lock().await.get_mut(&game_id).is_some() {
        games.lock().await.get_mut(&game_id).unwrap().add_player(player);
    }
}


fn create_deck() -> Vec<String> {
    let mut deck = vec![];

    for suit in ["H", "D", "C", "S"].iter() {
        for rank in [
            "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
        ]
        .iter()
        {
            deck.push(format!("{}{}", rank, suit));
        }
    }

    deck.shuffle(&mut rand::thread_rng());

    deck
}