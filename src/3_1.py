from collections import namedtuple
from sys import stdin

Point = namedtuple('Point', ('x', 'y'))


def main():
    current = Point(0, 0)
    points = {}
    for direction in stdin.read():
        points[current] = True
        current = move(current, direction)
    print(len(points))


def move(current, direction):
    return {
        '<': lambda: add_x(current, -1),
        '>': lambda: add_x(current, 1),
        'v': lambda: add_y(current, 1),
        '^': lambda: add_y(current, -1),
    }[direction]()


def add_x(current, value):
    return Point(x=current.x + value, y=current.y)


def add_y(current, value):
    return Point(x=current.x, y=current.y + value)


if __name__ == '__main__':
    main()
