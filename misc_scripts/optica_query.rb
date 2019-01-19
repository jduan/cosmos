#!/usr/bin/env ruby

require 'json'
# filepath to the output of querying optica for all hosts
#
# wget http://optica.d.musta.ch/ -O ~/tmp/optica_output.txt
# ruby optica_query.rb ~/tmp/optica_output.txt  > /tmp/out 2> /tmp/err
#
# number of hosts that don't have titanic installed
# cut -d, -f3 /tmp/out |  rg '^$' | wc
#
# hosts that have titanic installed
# cut -d, -f3 /tmp/out |  rg -v '^$'  | wc
#
# find statistics of titanic versions
# cut -d, -f3 /tmp/out |  rg -v '^$' | cut -d' ' -f1 | sort | uniq -c
#
optica_data = ARGV[0]
File.open(optica_data) do |fd|
  j = JSON.load(fd)
  nodes = j['nodes']
  nodes.each do |key, value|
    begin
      recipes = value['recipes']
      chef_role = value['role']
      hostname = value['hostname']
      titanic_versions = value['titanic_versions'] || []
      ruby_versions = []
      if recipes
        ruby_versions = recipes.select do |recipe|
          recipe =~ /^ruby/
        end
      end
      puts "#{hostname},#{chef_role},#{titanic_versions.join(' ')},#{ruby_versions.join(' ')}"
    rescue => e
      STDERR.puts "error processing node: #{value}"
      STDERR.puts e
    end
  end
end
