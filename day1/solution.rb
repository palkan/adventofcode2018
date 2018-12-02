# frozen_string_literal: true

class DayOne
  def calculate_final(file)
    acc = 0

    file.each_line do |line|
      acc += line.chomp.to_i
    end

    p "Final: #{acc}"
  end

  def calculate_freq(file)
    require "set"

    acc = 0
    freqs = Set.new([0])

    lines = file.each_line.with_object([]) do |line, acc|
      acc << line.chomp.to_i
    end

    lines.cycle.each do |val|
      acc += val

      if freqs.include?(acc)
        p "Repeated: #{acc}"
        break
      end

      freqs << acc
    end
  end
end

command = ARGV[0]
file = File.open(ARGV[1])

DayOne.new.public_send("calculate_#{command}", file)
