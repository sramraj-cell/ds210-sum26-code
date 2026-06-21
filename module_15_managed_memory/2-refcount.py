import sys

from helper import address

# x and y both refer to the same list!
x = [1, 2, 3]
y = x

# so, if x is modified, y is affected!
x.append(4)

# and vice versa!
y.append(5)

# We can see both modifications!
print(x)
print(y)

# We can also prove this by looking at the addresses of x and y!
print(address(x))
print(address(y))

# Python keeps track of how many active references there are to a piece of data.
print(sys.getrefcount(y))

# Deleting a variable (which is a reference) simply reduces the referece
# count by 1!
del y
print(sys.getrefcount(x))

# When the refcount hits 0, Python deletes the data!
del x
