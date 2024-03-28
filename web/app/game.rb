# frozen_string_literal: true

# Game stats
class Game
  attr_reader :id, :winner, :player, :player_points_to_win, :rounds, :rounds_number, :player_details,
              :create_time, :update_time

  def initialize(proto)
    %i[id winner player player_points_to_win rounds player_details create_time update_time].each do |method_name|
      send("assign_#{method_name}", proto.send(method_name))
    end

    @rounds_number = assing_rounds_number(proto.rounds)
  end

  private

  def assign_id(id)
    @id = id
  end

  def assign_create_time(create_time)
    @create_time = date_time(create_time)
  end

  def assign_update_time(update_time)
    @update_time = date_time(update_time)
  end

  def assign_winner(winner_name)
    @winner = Winner.new(winner_name) unless winner_name.empty?
  end

  def assign_player(player)
    @player = player
  end

  def assign_player_points_to_win(player_points_to_win)
    @player_points_to_win = player_points_to_win
  end

  def assign_rounds(rounds)
    @rounds = rounds.map { |round| Round.new(round) }
  end

  def assign_player_details(player_details)
    @player_details = player_details.map { |details| PlayerDetails.new(details) }
  end

  def assing_rounds_number(rounds)
    @rounds_number = rounds.size
  end

  def date_time(proto)
    seconds = proto.seconds
    nanos = proto.nanos
    time = Google::Protobuf::Timestamp.new(nanos:, seconds:).to_time

    time.strftime('%Y-%m-%d %H:%M:%S')
  end
end

# Player details
class PlayerDetails
  attr_reader :name, :points_to_win

  def initialize(proto)
    @name = proto.name
    @points_to_win = proto.points_to_win
  end
end

# Round
class Round
  attr_reader :number, :scores

  def initialize(proto)
    @number = proto.number
    @scores = proto.points.map { |point| Point.new(point) }
  end
end

# Point
class Point
  attr_reader :value, :kind

  def initialize(proto)
    @value = proto.value
    @kind = proto.kind.to_sym
  end

  def overthrow?
    kind == :POINT_KIND_OVERTHROW
  end
end

# Winner
class Winner
  attr_reader :name

  def initialize(name)
    @name = name
  end
end
