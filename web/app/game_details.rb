# frozen_string_literal: true

# Game stats
class GameDetails
  attr_reader :game_id, :winner, :player, :player_points_to_win, :rounds, :rounds_number, :player_details

  def initialize(proto)
    %i[game_id winner player player_points_to_win rounds player_details].each do |method_name|
      send("assign_#{method_name}", proto.send(method_name))
    end

    @rounds_number = assing_rounds_number(proto.rounds)
  end

  private

  def assign_game_id(game_id)
    @game_id = game_id
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
  attr_reader :number, :points

  def initialize(proto)
    @number = proto.number
    @points = proto.points.map { |point| Point.new(point) }
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
    kind == :overthrow
  end
end

# Winner
class Winner
  attr_reader :name

  def initialize(name)
    @name = name
  end
end
