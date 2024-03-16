# frozen_string_literal: true

require_relative './games_api'
require_relative './game'
require_relative './game_details'

# Result
class Result
  attr_reader :ok, :err

  def to_h
    return { ok: @ok } unless @ok.nil?
    return { err: @err } unless @err.nil?

    raise StandardError, 'Invalid state'
  end

  class << self
    def ok(value)
      new(:ok, value)
    end

    def err(value)
      new(:err, value)
    end
  end

  private

  def initialize(type, value)
    case type
    when :ok then @ok = value
    when :err then @err = value
    end
  end
end

# Games service
class GamesService
  class << self
    def cancel_last_score(game_id:)
      response = GamesApi.new.cancel_last_score(game_id:)
      game_details = GameDetails.new(response.game_details)

      Result.ok(game_details)
    end

    def count_points(game_id:, points:)
      response = GamesApi.new.count_points(game_id:, points:)
      game_details = GameDetails.new(response.game_details)

      Result.ok(game_details)
    end

    def get_game_details(game_id:)
      game_details = GameDetails.new(GamesApi.new.get_game_details(game_id:))

      Result.ok(game_details)
    end

    def create_game
      game = Game.new(GamesApi.new.create_game)

      Result.ok(game)
    end

    def list_games
      response = GamesApi.new.list_games
      games = response.games.map { |proto| Game.new(proto) }

      Result.ok(games)
    end
  end
end
