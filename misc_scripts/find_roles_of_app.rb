#!/usr/bin/env ruby

require 'json'
require 'json5'
require 'net/http'
require 'yaml'

app_config_file = ARGV[0]
data = File.read(app_config_file)

ext = File.extname(app_config_file)
if ext == '.json5'
  config = JSON5.parse(data)
elsif ext == '.yaml' || ext == '.yml'
  config = YAML.safe_load(data)
else
  raise "Invalid file format"
end

def role_valid?(role)
  uri = URI("http://optica.d.musta.ch?role=^#{role}$")
  response = Net::HTTP.get_response(uri)
  j = JSON.parse(response.body)
  j['nodes'].size > 0
end

all_roles = []
config['targets'].each do |target, values|
  puts "=========#{target}============"
  roles = values['roles'].keys.select do |role|
    role_valid?(role)
  end
  puts roles.join(',')
  all_roles.concat(roles)
end

puts "========= ALL ROLES ============"
puts all_roles.join(',')
