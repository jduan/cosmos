# Things to Remember
#
# Use @property to give existing instance attributes new functionality.
#
# Make incremental progress toward better data models by using @property.
#
# Consider refactoring a class and all call sites when you find yourself using @property too
# heavily. In the example below, if "fill" and "deduct" had been implemented as instance
# methods, you wouldn't need to make quota a @property setter.

from datetime import datetime, timedelta


class Bucket:
    def __init__(self, period):
        self.period_delta = timedelta(seconds=period)
        self.reset_time = datetime.now()
        self.max_quota = 0
        self.quota_consumed = 0

    @property
    def quota(self):
        return self.max_quota - self.quota_consumed

    @quota.setter
    def quota(self, amount):
        delta = self.max_quota - amount
        if amount == 0:
            # quota being reset for a new period
            self.quota_consumed = 0
            self.max_quota = 0
        elif delta < 0:
            # quota being filled for the new period
            assert self.quota_consumed == 0
            self.max_quota = amount
        else:
            # quota being consumed during the period
            assert self.max_quota >= self.quota_consumed
            self.quota_consumed += delta

    def __repr__(self):
        return "Bucket(max_quota=%d, quota_consumed=%d)" % (
            self.max_quota,
            self.quota_consumed,
        )


def fill(bucket, amount):
    now = datetime.now()
    if now - bucket.reset_time > bucket.period_delta:
        bucket.quota = 0
        bucket.reset_time = now
    bucket.quota += amount


def deduct(bucket, amount):
    now = datetime.now()
    if now - bucket.reset_time > bucket.period_delta:
        return False
    if bucket.quota - amount < 0:
        return False
    bucket.quota -= amount
    return True


def main():
    bucket = Bucket(60)
    print("Initial", bucket)
    fill(bucket, 100)
    print("Filled", bucket)
    if deduct(bucket, 99):
        print("Had 99 quota")
    else:
        print("Not enough for 99 quota")
    print("Now", bucket)

    if deduct(bucket, 3):
        print("Had 3 quota")
    else:
        print("Not enough for 3 quota")
    print("Still", bucket)


if __name__ == "__main__":
    main()
