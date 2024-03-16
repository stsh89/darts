# frozen_string_literal: true

require 'sinatra'
require_relative './games_service'

get '/' do
  result = GamesService.list_games
  games = handle_result(result)

  erb :home, locals: { games: }
end

get '/games/:game_id' do
  result = GamesService.get_game_details(game_id: params[:game_id])
  game_details = handle_result(result)

  erb :scoreboard, locals: { game_details: }
end

post '/games/:game_id/count_points' do
  points = params[:score].to_i
  result = GamesService.count_points(game_id: params[:game_id], points:)
  game_details = handle_result(result)

  if game_details.winner
    redirect "/games/#{game_details.game_id}"
  else
    erb :scores, layout: false, locals: { game_details: }
  end
end

post '/games/:game_id/cancel_last_score' do
  result = GamesService.cancel_last_score(game_id: params[:game_id])
  game_details = handle_result(result)

  erb :scores, layout: false, locals: { game_details: }
end

post '/games' do
  result = GamesService.create_game
  game = handle_result(result)

  redirect "/games/#{game.id}"
rescue StandardError => _e
  erb :internal_server_error
end

def handle_result(result, default_value = [])
  case result.to_h
  in {ok: value} then value
  in {err: error_message}
    @error_message = error_message
    default_value
  else
    raise Sinatra::Error
  end
end
