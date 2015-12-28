import re

from sys import stdin


def main():
    variables = {}

    def get_value(value):
        if isinstance(value, int):
            return value
        else:
            return get_variable_value(value)

    cache = {}

    @cached(cache)
    def get_variable_value(name):
        return variables[name]()

    for line in stdin:
        target, function = parse(get_value, line)
        variables[target] = function
    a = get_value('a')
    variables['b'] = lambda: a
    cache.clear()
    print(get_value('a'))


def cached(cache):
    def wrap(function):
        def wrapped(*args):
            result = cache.get(args)
            if result is not None:
                return result
            result = function(*args)
            cache[args] = result
            return result
        return wrapped
    return wrap


ASSIGN = re.compile(r'^(?P<value>\w+) -> (?P<target>\w+)$')
UNARY = re.compile(r'^(?P<operation>NOT) (?P<value>\w+) -> (?P<target>\w+)$')
BINARY = re.compile(r'^(?P<lhs>\w+) (?P<operation>AND|OR|LSHIFT|RSHIFT)'
                    r' (?P<rhs>\w+) -> (?P<target>\w+)$')


def parse(get_value, line):
    for r, f in ((ASSIGN, on_assign), (UNARY, on_unary), (BINARY, on_binary)):
        match = r.match(line)
        if match:
            return f(get_value, match)


def on_assign(get_value, match):
    value = get_int_or_str(match.group('value'))
    target = match.group('target')
    return target, lambda: get_value(value)


def on_unary(get_value, match):
    value = get_int_or_str(match.group('value'))
    operation = dict(
        NOT=lambda x: ~x,
    )[match.group('operation')]
    target = match.group('target')
    return target, lambda: operation(get_value(value)) % 65536


def on_binary(get_value, match):
    lhs = get_int_or_str(match.group('lhs'))
    rhs = get_int_or_str(match.group('rhs'))
    operation = dict(
        AND=lambda a, b: a & b,
        OR=lambda a, b: a | b,
        LSHIFT=lambda a, b: a << b,
        RSHIFT=lambda a, b: a >> b,
    )[match.group('operation')]
    target = match.group('target')
    return target, lambda: operation(get_value(lhs), get_value(rhs)) % 65536


def get_int_or_str(value):
    return int(value) if value.isdigit() else value


if __name__ == '__main__':
    main()
