#!/usr/bin/env ruby

blocks_txt = File.open("Blocks.txt")
blocks_rs = File.open("blocks.rs", 'w')

blocks_rs.puts "const UNICODE_BLOCKS: [Block; 300] = ["

blocks_txt.each_line do |line|
  line = line.chomp
  if line[0] != '#' && line != ''
    if matches = /([A-F0-9]+)\.\.([A-F0-9]+); (.+)/.match(line)
      blocks_rs.puts "    Block { start: 0x#{matches[1]}, end: 0x#{matches[2]}, name: \"#{matches[3]}\" },"
    else
      puts "error: can't match #{line.inspect}"
    end
  end
end

blocks_rs.puts "];"
