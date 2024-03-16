require '/home/ironman/github/stsh89/darts/web/lib/proto/games_pb'
require '/home/ironman/github/stsh89/darts/web/lib/proto/games_services_pb'
require 'google/protobuf/well_known_types'

# Games API
class GamesApi
  def initialize
    @stub = Proto::Playground::Games::Stub.new('[::1]:50051', :this_channel_is_insecure)
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

  def get_game_details(game_id:)
    @stub.get_game_details(
      Proto::Playground::GetGameDetailsRequest.new(game_id:)
    )
  end

  def list_games
    @stub.list_games(Proto::Playground::ListGamesRequest.new)
  end

  def create_game
    @stub.create_game(Proto::Playground::CreateGameRequest.new)
  end
end
