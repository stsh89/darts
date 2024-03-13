# frozen_string_literal: true

# Game list item
class GameListItem
  attr_reader :id, :start_time

  def initialize(proto_game)
    @id = proto_game.id
    @start_time = Google::Protobuf::Timestamp.new(nanos: proto_game.start_time.nanos,
                                                  seconds: proto_game.start_time.seconds).to_time.strftime('%Y-%m-%d %H:%M:%S')
  end
end
