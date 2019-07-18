# typed: strict

class Acronym
  
  sig {params(term: String).returns(String)}
  def self.abbreviate(term)
    term
    .gsub("-", " ")
    .split
    .map{|s| s[0]}
    .join
    .upcase
  end
end
