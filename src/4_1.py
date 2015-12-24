from sys import stdin
from hashlib import md5
from itertools import count


def main():
    initial = stdin.read()
    print(mine(initial.encode('utf-8')))


def mine(initial):
    for number in count(1):
        digest = md5(initial + str(number).encode('utf-8')).hexdigest()
        if frozenset(digest[:5]) == {'0'}:
            return number


if __name__ == '__main__':
    main()
