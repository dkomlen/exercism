class Matrix
  attr_reader :rows, :columns

  def initialize(mat)
    @rows = mat.each_line.map do |row|
      row.split.map(&:to_i)
    end
    @columns = @rows.transpose
  end
end
