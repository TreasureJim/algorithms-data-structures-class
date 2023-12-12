import math

def logn(start, end, startTimeReal, endTimeReal):
    ratio = end / start
    c = startTimeReal / (math.log2(start))
    theory = c * math.log2(ratio * start)

    print(f"c: {c}, start real: {startTimeReal}, theory: {int(theory)}, actual: {endTimeReal}")

def nlogn(start, end, startTimeReal, endTimeReal):
    ratio = end / start
    c = startTimeReal / (start * math.log2(start))
    theory = c * ratio * start * math.log2(ratio * start)

    print(f"c: {c}, start real: {startTimeReal}, theory: {int(theory)}, actual: {endTimeReal}")

def quad(start, end, startTimeReal, endTimeReal):
    ratio = end / start
    c = startTimeReal / (start ** 2)
    theory = c * (ratio * start)**2

    print(f"c: {c}, start real: {startTimeReal}, theory: {int(theory)}, actual: {endTimeReal}")

print("qs first recur")
nlogn(100000, 1000000, 7829, 448092)

print("qs median recur")
nlogn(100000, 1000000, 7314, 339602)

print("qs median recur")
nlogn(100000, 1000000, 8893, 429718)

print("insertion")
quad(100000, 1000000, 3200947, 444065625)

print("binary")
quad(100000, 1000000, 25, 158)

