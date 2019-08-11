# Using Sorbet gradual type checker: https://sorbet.org/
# typed: true
require "sorbet-runtime"
require "set"

class Pangram
  extend T::Sig
  PANGRAM_LEN = 26

  sig { params(sentence: String).returns(T::Boolean) }
  def self.pangram?(sentence)
    sentence
      .downcase
      .chars
      .select { |c| c =~ /[[:alpha:]]/ }
      .to_set
      .size == PANGRAM_LEN
  end
end
