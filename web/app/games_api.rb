# frozen_string_literal: true

require 'proto/games_pb'
require 'proto/games_services_pb'
require 'google/protobuf/well_known_types'

# Games API
class GamesApi
  def initialize
    @stub = Proto::Playground::Games::Stub.new("#{GRPC_API_HOST}:50051", :this_channel_is_insecure)
  end

  def cancel_last_score(game_id:)
    @stub.cancel_last_score(
      Proto::Playground::CancelLastScoreRequest.new(game_id:)
    )
  end

  def count_points(game_id:, points:)
    @stub.count_points(
      Proto::Playground::CountPointsRequest.new(
        game_id:,
        points:
      )
    )
  end

  def get_game(game_id:)
    @stub.get_game(
      Proto::Playground::GetGameRequest.new(game_id:)
    )
  end

  def list_games
    @stub.list_games(Proto::Playground::ListGamesRequest.new)
  end

  def create_game
    @stub.create_game(Proto::Playground::CreateGameRequest.new)
  end
end
