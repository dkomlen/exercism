# Using Sorbet gradual type checker: https://sorbet.org/
# typed: true
require "sorbet-runtime"

class Complement
  extend T::Sig

  COMPLEMENTS = {
    "G" => "C",
    "C" => "G",
    "T" => "A",
    "A" => "U",
  }

  sig { params(dna: String).returns(String) }
  def self.of_dna(dna)
    dna.chars.map { |c| COMPLEMENTS[c] }.join
  end
end
