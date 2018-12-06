# frozen_string_literal: true

input = File.read(ARGV[0]).chomp.chars

size = input.size

using(Module.new do
  CODE_DELTA = "z".ord - "Z".ord

  refine String do
    def react_on?(other)
      (ord - other.ord).abs == CODE_DELTA
    end

    def ieql?(other)
      ord == other.ord || react_on?(other)
    end
  end
end)

def reduce(input, ignore = nil)
  pos = 0

  loop do
    a, b = input[pos], input[pos + 1]

    if ignore
      next input.slice!(pos) if a&.ieql?(ignore)
      next input.slice!(pos + 1) if b&.ieql?(ignore)
    end

    break if b.nil?

    next pos += 1 unless a.react_on?(b)

    input.slice!(pos, 2)

    # move one step back
    # to handle posible double-collision:
    #   dabA|cCab -> dabA|ab -> dab|Aab
    pos -= 1 if pos > 0
  end
end

input.dup.yield_self do |inp|
  reduce(inp)
  p "Reduced size: #{inp.size}"
end

min_arg = nil
min = input.size

('a'..'z').each do |letter|
  input.dup.yield_self do |inp|
    reduce(inp, letter)

    if inp.size < min
      min = inp.size
      min_arg = letter
    end
  end
end

p "Most problematic: #{min_arg} (length: #{min})"
