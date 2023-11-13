# Quick Sort

1. swap pivot element with last element
2. LP = 0, RP = len - 2 (one before the last element)
3. while LP <= RP && arr[LP] < pivot: LP++
4. while RP >= pivot: RP --

(LP and RP have stopped now) LP is point at large element and RP is pointing at small element

5. if LP < RP: swap LP and RP elements
6. stop when LP and RP pass each other
7. swap pivot with lp

every element before LP is now small

for n < 10 || 20 use insertion sort
