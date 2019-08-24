class Matrix
  attr_reader :rows, :columns

  def initialize(mat)
    @rows = mat.split("\n").map do |row|
      row.split.map do |num|
        num.to_i
      end
    end
    n_cols = @rows[0]&.size || 0
    @columns =  Array.new(n_cols){[]}
    @rows.each do |row|
      row.each.with_index do |item, col|
        columns[col].append(item)
      end
    end
  end
end
