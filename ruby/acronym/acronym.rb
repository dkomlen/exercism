# Using Sorbet gradual type checker: https://sorbet.org/
# typed: strict
require "sorbet-runtime"

class Acronym
  extend T::Sig

  sig { params(term: String).returns(String) }
  def self.abbreviate(term)
    term.scan(/\b\w/).join('').upcase
  end
end
