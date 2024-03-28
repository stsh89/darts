# frozen_string_literal: true

require 'sinatra'
require_relative './games_service'

GRPC_API_HOST = ENV['GRPC_API_HOST'] || '[::1]'

get '/' do
  result = GamesService.list_games
  games = handle_result(result)

  erb :home, locals: { games: }
end

get '/games/:game_id' do
  result = GamesService.get_game(game_id: params[:game_id])
  game = handle_result(result)

  erb :scoreboard, locals: { game: }
end

post '/games/:game_id/count_points' do
  points = params[:score].to_i
  result = GamesService.count_points(game_id: params[:game_id], points:)
  game = handle_result(result)

  if game.winner
    redirect "/games/#{game.id}"
  else
    erb :scores, layout: false, locals: { game: }
  end
end

post '/games/:game_id/cancel_last_score' do
  result = GamesService.cancel_last_score(game_id: params[:game_id])
  game = handle_result(result)

  erb :scores, layout: false, locals: { game: }
end

post '/games' do
  result = GamesService.create_game
  game = handle_result(result)

  redirect "/games/#{game.id}"
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
