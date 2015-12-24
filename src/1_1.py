from sys import stdin

print(sum({'(': 1, ')': -1}[v] for v in stdin.read()))
