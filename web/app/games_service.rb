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
    def get_game_details(game_id:)
      with_resque do
        game_details = GameDetails.new(GamesApi.new.get_game_details(game_id:))

        Result.ok(game_details)
      end
    end

    def create_game
      with_resque do
        game = Game.new(GamesApi.new.create_game)

        Result.ok(game)
      end
    end

    def list_games
      with_resque do
        response = GamesApi.new.list_games
        games = response.games.map { |proto| Game.new(proto) }

        Result.ok(games)
      end
    end

    private

    def with_resque(&block)
      block.call
    rescue StandardError => e
      case e
      when GRPC::Unavailable then Result.err('Backend is not available')
      else
        warn e
        warn e.backtrace

        Result.err('Something went wrong')
      end
    end
  end
end
