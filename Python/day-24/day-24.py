from z3 import *

with open("input.txt", 'r') as f :
    inputLines = f.read().strip().split("\n")

x, y, z = Ints('x y z')
vx, vy, vz = Ints('vx vy vz')

s = Solver()

for i, line in enumerate(inputLines) :
    pos, vel = line.split(" @ ")
    spx, spy, spz = [int(v.strip()) for v in pos.split(", ")]
    svx, svy, svz = [int(v.strip()) for v in vel.split(", ")]

    s.add( (spx - x) * (vy - svy) == (spy - y) * (vx - svx) )
    s.add( (spx - x) * (vz - svz) == (spz - z) * (vx - svx) )
    s.add( (spy - y) * (vz - svz) == (spz - z) * (vy - svy) )

s.check()
m = s.model()
#print(m)

x = m.eval(x).as_long()
y = m.eval(y).as_long()
z = m.eval(z).as_long()

sum = x + y + z

print(f"Part 2. Sum (x + y + z) = {sum}")
