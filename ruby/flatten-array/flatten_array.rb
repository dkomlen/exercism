class FlattenArray
  def self.flatten(array)
    result = []
    array.each do |item|
      if item.respond_to?("each")
        result.concat(flatten(item))
      elsif !item.nil?
        result.push(item)
      end
    end
    result
  end
end
