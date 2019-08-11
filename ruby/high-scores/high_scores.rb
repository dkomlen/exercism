# Using Sorbet gradual type checker: https://sorbet.org/
# typed: true
require "sorbet-runtime"

class HighScores
  extend T::Sig
  attr_reader :scores, :latest
  attr_reader :personal_best, :personal_top_three

  sig { params(scores: T::Array[Integer]).void }
  def initialize(scores)
    @scores = scores
    @latest = scores[-1]
    @personal_top_three = scores.sort.reverse[0..2]
    @personal_best = personal_top_three[0]
  end
end
