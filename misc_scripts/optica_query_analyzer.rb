#!/usr/bin/env ruby
#
# This script takes the output of "optica_query.rb" and analyzes the result.
require 'set'

expected_titanic_version = '1.1.25'

version_to_role = Hash.new {|hsh, key| hsh[key] = Set.new}
ARGF.each do |line|
  host, role, version_str, rest = line.split(',')
  # Ignore ".pre" versions
  version = version_str.split.reject {|v| v =~ /pre/}[0]
  version_to_role[version].add("#{role},#{host}")
end

version_to_role.each do |version, roles|
  if version != expected_titanic_version
    puts "version: #{version}, total roles: #{roles.size}"
    roles.sort.each {|role| puts role}
    puts
  end
end
