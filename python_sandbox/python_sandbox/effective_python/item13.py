import json


def main():
    data = '{"name": "jingjing", "age": 99}'
    try:
        result_dict = json.loads(data)
    except ValueError as e:
        raise RuntimeError("Failed to pass json data") from e
    else:
        # When the try block doesn't raise an exception, the else block will run.
        # The else block helps you minimize the amount of code in the try block
        # and improves readability.
        print("name is", result_dict['name'])
    finally:
        print("The finally block always runs!")


if __name__ == '__main__':
    main()
