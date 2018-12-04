# frozen_string_literal: true

require "set"

class DayTwo
  def calculate_checksum(file)
    twos = 0
    threes = 0

    file.each_line do |line|
      matches = line.chomp.chars.
        sort.join('').to_enum(:scan, /([a-z])\1+/i).
        map { Regexp.last_match }

      counts = Set.new(matches.map(&:to_s).map(&:size))

      twos += 1 if counts.include?(2)
      threes += 1 if counts.include?(3)
    end

    # More simpler solution:
    #
    # file.each_line do |line|
    #   line.chomp.chars.each_with_object(Hash.new {|h,k| h[k] = 0}) do |char, acc|
    #     acc[char] += 1
    #   end.values.yield_self do |counts|
    #     twos += 1 if counts.include?(2)
    #     threes += 1 if counts.include?(3)
    #   end
    # end

    p "Checksum: #{twos * threes}"
  end

  using(Module.new do
    refine String do
      def each_candidate
        0.upto(size) do |i|
          charz = chars
          charz[i] = "_"
          yield charz.join
        end
      end
    end
  end)

  def calculate_pair(file)
    store = {}

    file.each_line do |line|
      line.chomp!
      line.each_candidate do |candidate|
        if store.key?(candidate)
          p "Pair for #{candidate}: #{line} and #{store[candidate]}"
          break
        end

        store[candidate] = line
      end
    end
  end
end

command = ARGV[0]
file = File.open(ARGV[1])

DayTwo.new.public_send("calculate_#{command}", file)
