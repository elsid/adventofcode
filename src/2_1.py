import re

from collections import namedtuple
from sys import stdin


def main():
    presents = (parse(line) for line in stdin)
    squares = (square(present) for present in presents)
    print(sum(squares))


Preset = namedtuple('Present', ('length', 'width', 'height'))
PRESENT = re.compile(r'^(?P<length>\d+)x(?P<width>\d+)x(?P<height>\d+)$')


def parse(line):
    match = PRESENT.match(line)
    return Preset(**{k: int(v) for k, v in match.groupdict().items()})


def square(present: Preset):
    sides = (
        present.length * present.width,
        present.width * present.height,
        present.height * present.length,
    )
    return min(sides) + sum(2 * v for v in sides)


if __name__ == '__main__':
    main()
