# Using Sorbet gradual type checker: https://sorbet.org/
# typed: true
require "sorbet-runtime"

class ResistorColorDuo
  extend T::Sig

  COLOR_MAP = {
    "black" => 0,
    "brown" => 1,
    "red" => 2,
    "orange" => 3,
    "yellow" => 4,
    "green" => 5,
    "blue" => 6,
    "violet" => 7,
    "grey" => 8,
    "white" => 9,
  }

  sig { params(colors: T::Array[String]).returns(Integer) }
  def self.value(colors)
    colors.reduce(0) { |agg, c| agg * 10 + COLOR_MAP[c] }
  end
end
