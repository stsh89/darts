use crate::{bad_request, internal_server_error, layout, not_found, AppState};
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};
use gateway::{
    max_game_score,
    referee::{CountScoreParameters, NewGameParameters, Score},
    repo::Repo,
    spectator::{GetGameParameters, ListGamesParameters},
    Game, TotalGameScore,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct ScoreForm {
    score: String,
}

struct GameList {
    items: Vec<GameListItem>,
}

struct GameListItem {
    id: String,
}

impl GameList {
    fn into_html(self) -> String {
        let items = self
            .items
            .into_iter()
            .map(|item| item.into_html())
            .collect::<Vec<String>>()
            .join("\n");

        format!("<ul>{}</ul>", items)
    }
}

impl GameListItem {
    fn into_html(self) -> String {
        format!(
            r#"
<li><a href="/games/{}/">{}</a></li>"#,
            self.id, self.id
        )
    }
}

struct Scoreboard {
    game_id: String,
    player: String,
    player1: Vec<u8>,
    player2: Vec<u8>,
    player1_left: u16,
    player2_left: u16,
}

impl Scoreboard {
    fn into_html(self) -> String {
        let rounds_number = self.player1.len();
        let mut rounds_header = vec![];

        for i in (1..=rounds_number).rev() {
            rounds_header.push(format!("<th>{}</th>", i));
        }

        let cell = if self.player1.len() == self.player2.len() {
            ""
        } else {
            "<td></td>"
        };

        let player1_scores = self
            .player1
            .into_iter()
            .map(|x| format!("<td>{}</td>", x))
            .collect::<Vec<String>>()
            .join("");

        let player2_scores = self
            .player2
            .into_iter()
            .map(|x| format!("<td>{}</td>", x))
            .collect::<Vec<String>>()
            .join("");

        format!(
            r#"
<div class="block">
<form class="block" action="/games/{}/scores/" method="post" autocomplete="off">
<div class="field">
  <label class="label is-size-5" name="score">{} to throw</label>
  <div class="control">
    <input class="input" inputmode="numeric" type="text" name="score" pattern="[0-9\+]+" autofocus="true">
  </div>
</div>
</form>
</div>
<div class="block">
<div class="table-container">
<table class="table is-striped is-size-4">
<thead>
      <tr>
        <th>#</th><th>#</th>{}
      </tr>
    </thead>
<tbody>
<tr>
    <td>Player1</td><td class="has-text-weight-bold">{}</td>{}
</tr>
<tr>
    <td>Player2</td><td class="has-text-weight-bold">{}{}</td>{}
</tr>
</tbody>
</table>
</div>
</div>
        "#,
            self.game_id,
            self.player,
            rounds_header.join(""),
            self.player1_left,
            player1_scores,
            self.player2_left,
            cell,
            player2_scores
        )
    }
}

impl From<Game> for Scoreboard {
    fn from(game: Game) -> Self {
        Self {
            game_id: game.id.to_string(),
            player1_left: (&max_game_score() - &game.player1_scores.iter().total_game_score())
                .value(),
            player2_left: (&max_game_score() - &game.player2_scores.iter().total_game_score())
                .value(),
            player: game.player_number.name().to_string(),
            player1: game
                .player1_scores
                .into_iter()
                .rev()
                .map(|score| score.into_inner())
                .collect(),
            player2: game
                .player2_scores
                .into_iter()
                .rev()
                .map(|score| score.into_inner())
                .collect(),
        }
    }
}

impl From<Game> for GameListItem {
    fn from(game: Game) -> Self {
        Self {
            id: game.id.to_string(),
        }
    }
}

pub async fn add_score(
    Path(game_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Form(input): Form<ScoreForm>,
) -> Response {
    let Ok(game_id) = Uuid::parse_str(&game_id) else {
        return internal_server_error();
    };

    let score = parse_input_score(&input.score);

    let Ok(score) = score else {
        return bad_request();
    };

    let game = gateway::referee::count_score(CountScoreParameters {
        game_id,
        score: Score::new(score),
        games: Arc::new(Repo::new(state.database_pool.clone())),
        scores: Arc::new(Repo::new(state.database_pool.clone())),
    })
    .await;

    let Ok(game) = game else {
        return internal_server_error();
    };

    render_game(game).into_response()
}

pub async fn create(State(state): State<Arc<AppState>>) -> Response {
    let game = gateway::referee::new_game(NewGameParameters {
        games: Arc::new(Repo::new(state.database_pool.clone())),
    })
    .await;

    let Ok(game) = game else {
        return internal_server_error();
    };

    // render_game(game).into_response()

    Redirect::to(&format!("/games/{}/", game.id)).into_response()
}

pub async fn index(State(state): State<Arc<AppState>>) -> Response {
    let games = gateway::spectator::list_games(ListGamesParameters {
        games: Arc::new(Repo::new(state.database_pool.clone())),
    })
    .await;

    let Ok(games) = games else {
        return internal_server_error();
    };

    render_game_list(games).into_response()
}

pub async fn show(Path(game_id): Path<String>, State(state): State<Arc<AppState>>) -> Response {
    let game_id = Uuid::parse_str(&game_id);

    let Ok(game_id) = game_id else {
        return internal_server_error();
    };

    let game = gateway::spectator::get_game(GetGameParameters {
        game_id,
        games: Arc::new(Repo::new(state.database_pool.clone())),
    })
    .await;

    let Ok(game) = game else {
        return not_found();
    };

    render_game(game).into_response()
}

fn render_game_list(games: Vec<Game>) -> Html<String> {
    let game_list = GameList {
        items: games.into_iter().map(GameListItem::from).collect(),
    }
    .into_html();

    let partial = format!(
        r#"<h1 class="title">Recent games</h1>
    <div class="block">
    <form action="/games/" method="post">
      <div class="field">
        <div class="control">
          <button class="button is-link" type="submit" autofocus="true">New game</button>
        </div>
      </div>
    </form>
  </div>
    {}"#,
        game_list
    );

    layout(partial)
}

fn render_game(game: Game) -> Html<String> {
    let scoreboard = Scoreboard::from(game).into_html();

    let partial = format!(
        r#"<div class="block"><a href="/">Back to recent games</a></div>
{}"#,
        scoreboard
    );

    layout(partial)
}

fn parse_input_score(score: &str) -> Result<u8, ()> {
    if score.is_empty() {
        return Ok(0);
    }

    let result = score
        .split('+')
        .map(|value| value.parse::<u8>())
        .collect::<Result<Vec<u8>, _>>();

    match result {
        Ok(scores) => Ok(scores.iter().sum()),
        Err(_) => Err(()),
    }
}
