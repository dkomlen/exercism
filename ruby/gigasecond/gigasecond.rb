class Gigasecond
  TIME_DELTA = 10 ** 9
  def self.from(time)
    time + TIME_DELTA
  end
end
