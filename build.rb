#!/usr/bin/env ruby

blocks_txt = File.open("data/Blocks.txt")
blocks_rs = File.open("data/blocks.rs", 'w')
blocks = []

blocks_txt.each_line do |line|
  line = line.chomp
  if line[0] != '#' && line != ''
    if matches = /([A-F0-9]+)\.\.([A-F0-9]+); (.+)/.match(line)
      blocks << "    Block { start: 0x#{matches[1]}, end: 0x#{matches[2]}, name: \"#{matches[3]}\" },"
    else
      puts "error: can't match #{line.inspect}"
    end
  end
end

blocks_rs.puts "["
blocks.each do |block|
  blocks_rs.puts block
end
blocks_rs.puts "];"
