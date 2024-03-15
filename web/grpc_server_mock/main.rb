# frozen_string_literal: true

require '/home/ironman/github/stsh89/darts/web/lib/proto/games_pb'
require '/home/ironman/github/stsh89/darts/web/lib/proto/games_services_pb'
require 'google/protobuf/well_known_types'
require 'time'
require 'securerandom'

# GamesService implementation
class GamesService < Proto::Playground::Games::Service
  def initialize
    @games = []
    path = File.join(Dir.pwd, 'grpc_server_mock/games.txt')

    File.foreach(path) do |line|
      @games << parse_game(line)
    end

    super
  end

  def get_game_details(request, _call)
    game = @games.find { |g| g.id == request.game_id }

    raise GRPC::NotFound, 'Game not found' if game.nil?

    default_game_details(game)
  end

  def list_games(_request, _call)
    Proto::Playground::ListGamesResponse.new(games: @games)
  end

  def create_game(_request, _call)
    id = SecureRandom.uuid
    start_time = Time.now

    @games << Proto::Playground::Game.new(id:, start_time:)

    @games.last
  end

  private

  def parse_game(line)
    id, time_string = line.split(',')
    start_time = Time.parse(time_string)

    Proto::Playground::Game.new(id:, start_time:)
  end

  def default_player_details
    [
      Proto::Playground::PlayerDetails.new(name: 'Player1', points_to_win: 301),
      Proto::Playground::PlayerDetails.new(name: 'Player2', points_to_win: 301)
    ]
  end

  def default_game_details(game)
    Proto::Playground::GameDetails.new(
      game_id: game.id,
      player: 'Player1',
      player_points_to_win: 301,
      player_details: default_player_details
    )
  end
end

port = '[::1]:50051'
s = GRPC::RpcServer.new
s.add_http2_port(port, :this_port_is_insecure)
GRPC.logger.info("... running insecurely on #{port}")
s.handle(GamesService.new)
s.run_till_terminated_or_interrupted([1, 'int', 'SIGQUIT'])
