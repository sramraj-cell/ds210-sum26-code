from helper import address


v = ["hello", "goodbye"]
x = v[0]

print(address(v[0]))
print(address(x))

for i in range(10):
    v.append("this is a long string")

print('is x dangling?', x)

print(address(v[0]))
print(address(x))