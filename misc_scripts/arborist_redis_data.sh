#!/usr/bin/env bash

port=6861 # prod
# port=9315 # staging

get_redis_data() {
  redis_database=$1
  echo -e "\n========== Checking Redis Database $redis_database ==========\n"
  echo -e "\n==========> 1. treehouse_builds_seen\n"
  redis-cli -p $port -n "$redis_database" zrange treehouse_builds_seen -5 -1
  echo -e "\n==========> 2. treehouse_builds\n"
  redis-cli -p $port -n "$redis_database" zrange treehouse_builds -2 -1 | jq
  echo -e "\n==========> 3. treehouse_builds_consumed\n"
  redis-cli -p $port -n "$redis_database" zrange treehouse_builds_consumed -2 -1 | jq

  today=$(date +"%Y_%m_%d")
  echo -e "\n==========> 4. treehouse_$today\n"
  redis-cli -p $port -n "$redis_database" hgetall "treehouse_$today"
}

get_redis_data 0
get_redis_data 1
