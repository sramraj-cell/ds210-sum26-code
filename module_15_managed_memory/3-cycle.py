from helper import address

import sys
import gc


x = [1, 2, 3]
x.append(x)
x.append(x)
print(x)

print(sys.getrefcount(x))
del x

count = gc.collect()
print(count)
