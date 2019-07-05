import json
import redis

rds = redis.Redis(host="localhost", port=6379, db=0)
bad_items = set()
redis_key = "resque:queue:notifications"
pr_numbers = {
    564695,
    590740,
    1153425,
    614328,
    1065882,
    1153128,
    1158572,
    1130644,
}
start = 0
end = rds.llen(redis_key)
print("queue length: %s" % end)

items = rds.lrange(redis_key, start, end)
for item in items:
    event = json.loads(item)
    if (event['class'] == 'GitHub::Jobs::CreateSubscribedIssueEvents'
            and event['args'][0] in pr_numbers
            and event['meta']['context']['job'] == "GitHub::jobs::RetryJobs"):
        bad_items.add(item)

for bad_item in sorted(bad_items):
    print("bad item: %s" % bad_item)
    # rds.lrem(redis_key, 0, bad_item)
