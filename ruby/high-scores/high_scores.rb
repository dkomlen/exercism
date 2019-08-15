# Using Sorbet gradual type checker: https://sorbet.org/
# typed: true
require "sorbet-runtime"

class HighScores
  extend T::Sig
  attr_reader :scores, :latest, :personal_best, :personal_top_three

  sig { params(scores: T::Array[Integer]).void }
  def initialize(scores)
    @scores = scores
    @latest = scores.last
    @personal_top_three = scores.sort.reverse[0..2]
    @personal_best = personal_top_three.first
  end
end
