# frozen_string_literal: true

require '/home/ironman/github/stsh89/darts/web/lib/proto/games_pb'
require '/home/ironman/github/stsh89/darts/web/lib/proto/games_services_pb'
require 'google/protobuf/well_known_types'

# GamesService implementation
class GamesService < Proto::Games::Games::Service
  def list_games(_request, _call)
    games = [
      Proto::Games::Game.new(id: '35523d4d-2eeb-4351-b876-1fa19ec5f835', start_time: Time.new(2024, 3, 14, 14, 0, 0)),
      Proto::Games::Game.new(id: '35523d4d-2eeb-4351-b876-1fa19ec5f834', start_time: Time.new(2024, 3, 14, 13, 0, 0)),
      Proto::Games::Game.new(id: '35523d4d-2eeb-4351-b876-1fa19ec5f833', start_time: Time.new(2024, 3, 14, 12, 0, 0)),
      Proto::Games::Game.new(id: '35523d4d-2eeb-4351-b876-1fa19ec5f832', start_time: Time.new(2024, 3, 14, 11, 0, 0)),
      Proto::Games::Game.new(id: '35523d4d-2eeb-4351-b876-1fa19ec5f831', start_time: Time.new(2024, 3, 14, 10, 0, 0))
    ]

    Proto::Games::ListGamesResponse.new(
      games:
    )
  end
end

port = '[::1]:50051'
s = GRPC::RpcServer.new
s.add_http2_port(port, :this_port_is_insecure)
GRPC.logger.info("... running insecurely on #{port}")
s.handle(GamesService.new)
s.run_till_terminated_or_interrupted([1, 'int', 'SIGQUIT'])
