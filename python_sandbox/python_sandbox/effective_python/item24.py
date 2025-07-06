# Things to Remember
#
# Python only supports a single constructor per class, the __init__ method.
#
# Use @classmethod to define alternative constructors for your classes. They are like
# "factory methods" in Java.
#
# Use class method polymorphism to provide "generic" ways to build and connect concrete subclasses.


import os
from threading import Thread


class InputData(object):
    def read(self):
        raise NotImplementedError

    @classmethod
    def generate_inputs(cls, config):
        raise NotImplementedError


class PathInputData(InputData):
    def __init__(self, path):
        super().__init__()
        self.path = path

    def read(self):
        return open(self.path).read()

    @classmethod
    def generate_inputs(cls, config):
        data_dir = config["data_dir"]
        for name in os.listdir(data_dir):
            yield cls(os.path.join(data_dir, name))


class Worker(object):
    def __init__(self, input_data):
        self.input_data = input_data
        self.result = None

    def map(self):
        raise NotImplementedError

    def reduce(self, other):
        raise NotImplementedError

    @classmethod
    def create_workers(cls, input_class, config):
        workers = []
        for input_data in input_class.generate_inputs(config):
            workers.append(cls(input_data))
        return workers


class LineCountWorker(Worker):
    def map(self):
        data = self.input_data.read()
        self.result = data.count("\n")

    def reduce(self, other):
        self.result += other.result


def generate_inputs(data_dir):
    for name in os.listdir(data_dir):
        yield PathInputData(os.path.join(data_dir, name))


def create_workers(input_list):
    return [LineCountWorker(input_data) for input_data in input_list]


def execute(workers):
    threads = [Thread(target=w.map) for w in workers]
    for thread in threads:
        thread.start()
    for thread in threads:
        thread.join()

    first, rest = workers[0], workers[1:]
    for worker in rest:
        first.reduce(worker)
    return first.result


def map_reduce(worker_class, input_class, config):
    """
    The beauty if map_reduce is that it's generic. You can pass in different
    worker_class and input_class, and it will glue them together!
    """
    workers = worker_class.create_workers(input_class, config)
    return execute(workers)


def main():
    current_dir = os.path.dirname(os.path.abspath(__file__))
    config = {"data_dir": current_dir}
    result = map_reduce(LineCountWorker, PathInputData, config)
    print("There are %s lines" % result)


if __name__ == "__main__":
    main()
