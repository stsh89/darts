# frozen_string_literal: true

# Game list item
class Game
  attr_reader :id, :start_time, :end_time

  def initialize(proto_game)
    assign_id(proto_game.id)
    assign_start_time(proto_game.start_time) if proto_game.start_time
  end

  private

  def assign_id(id)
    @id = id
  end

  def assign_start_time(proto_start_time)
    seconds = proto_start_time.seconds
    nanos = proto_start_time.nanos
    time = Google::Protobuf::Timestamp.new(nanos:, seconds:).to_time

    @start_time = time.strftime('%Y-%m-%d %H:%M:%S')
  end
end
