#!/usr/bin/env ruby
# Usage:
# ./check_github_repos.rb <path_to_github_repo_parent_dir>
Dir.chdir(ARGV[0])
Dir.entries('.').each do |repo|
  # Ignore "." and ".."
  next if repo == "." || repo == ".."
  # Only consider directories
  next if File.file?(repo)
  puts "=====#{repo}====="
  Dir.chdir repo
  if File.exists?('.git')
    output = `git status`
    if output =~ /On branch master/ && output =~ /Your branch is up to date with 'origin\/master'/
      puts "On master branch and up to date"
    else
      puts "!!! (#{repo}) Not on master or not up to date !!!"
    end
  else
    puts "Not a git repo"
  end
  Dir.chdir ".."
  puts
end
