class Series
  def initialize(input)
    @nums = input
  end

  def slices(len)
    if len < 1 || len > @nums.length
      raise ArgumentError, 'Invalid slice length'
    end

    @nums.chars.each_cons(len).map(&:join)
  end
end
