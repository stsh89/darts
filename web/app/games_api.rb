require '/home/ironman/github/stsh89/darts/web/lib/proto/games_pb'
require '/home/ironman/github/stsh89/darts/web/lib/proto/games_services_pb'
require 'google/protobuf/well_known_types'

# Games API
class GamesApi
  def initialize
    @stub = Proto::Games::Games::Stub.new('[::1]:50051', :this_channel_is_insecure)
  end

  def list_games
    @stub.list_games(Proto::Games::ListGamesRequest.new)
  end
end
