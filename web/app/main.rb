# frozen_string_literal: true

require 'sinatra'
require_relative './games_api'
require_relative './game_list_item'

get '/' do
  games = GamesApi.new.list_games.games.map { |game| GameListItem.new(game) }

  erb :home, locals: {games: games}
end

get '/games/:game_id' do
  erb :game
end

post '/games/:game_id/add_score' do
  erb :scores, layout: false
end

post '/games/:game_id/cancel_score' do
  erb :scores, layout: false
end

post '/games' do
  game_id = 1

  redirect "/games/#{game_id}"
end

error Sinatra::NotFound do
  erb :not_found
end
