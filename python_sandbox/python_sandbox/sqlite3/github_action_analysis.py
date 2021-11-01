import json
import sqlite3


def get_workflow_runs(json_file):
    """
    Read json file and extract relevant fields
    """
    with open(json_file) as fd:
        data = json.load(fd)
        print("count: %s" % data["total_count"])


def persist_data(conn, entries):
    cursor = conn.cursor()
    cursor.execute("INSERT INTO workflow_runs VALUES ('bayone', 'deploy', 1, 'main', 'completed', 10, 'success')")
    conn.commit()


def main():
    print('Hello, world!')
    json_file = "/Users/jduan/tmp/github_runs.json"
    db_file = "/Users/jduan/tmp/github_action_analysis.db"
    conn = sqlite3.connect(db_file)
    get_workflow_runs(json_file)


if __name__ == '__main__':
    main()
