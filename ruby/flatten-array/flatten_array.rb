class FlattenArray
  def self.flatten(xs)
    ret = []
    for x in xs
      if x.respond_to?("count")
        ret += flatten(x)
      elsif x
        ret += [x]
      end
    end
    return ret
  end
end
