import matplotlib
matplotlib.use("pgf")

import matplotlib.pyplot as plt
import numpy as np


def avg_dict(dict):
    for (size, lst) in dict.items():
        # dict[size] = sum(lst) / len(lst)
        dict[size] = int(sum(lst) / len(lst))

    sizes, avg = zip(*sorted(dict.items()))
    return (sizes, avg)


quicksort_iter_median = {
    100: [15, 5, 7, 7, 4, 7, 6, 6, 6, 13],
    1000: [50, 60, 47, 48, 47, 47, 60, 73, 46, 89],
    10000: [841, 900, 836, 805, 775, 788, 910, 790, 880, 878],
    100000: [40718, 41328, 40137, 40340, 40268, 40070, 40408, 40151, 40116, 40208],
    1000000: [3465285, 3621021, 3691337, 3475514, 3675360, 3605695, 3680831, 3532472, 3362648, 3367648],
}
quicksort_iter_median = avg_dict(quicksort_iter_median)
print("quicksort_iter_median", *zip(*quicksort_iter_median))

quicksort_iter_random = {
    100: [13, 21, 9, 9, 15, 9, 9, 9, 9, 9],
    1000: [88, 94, 55, 53, 53, 54, 53, 56, 56, 65],
    100000: [54031, 51902, 53415, 52364, 52987, 51732, 51870, 49637, 50531, 53342],
    10000: [1024, 1139, 1115, 1189, 1164, 1150, 1121, 1159, 1172, 1125],
    1000000: [4624650, 4780009, 4693737, 4524178, 4918928, 4940209, 4872351, 4689612, 4423382, 4353560],
}
quicksort_iter_random = avg_dict(quicksort_iter_random)
print("quicksort_iter_random", *zip(*quicksort_iter_random))

quicksort_iter_first = {
    100: [5, 5, 7, 7, 7, 6, 6, 6, 6, 9],
    100000: [40568, 40127, 40154, 39936, 40523, 40489, 39833, 39849, 41187, 39780],
    10000: [802, 886, 834, 782, 811, 780, 782, 787, 785, 781],
    1000000: [3477445, 3625413, 3569782, 3465620, 3635657, 3722658, 3702980, 3519468, 3362265, 3367837],
    1000: [77, 57, 46, 47, 46, 46, 47, 49, 45, 59]
}
quicksort_iter_first = avg_dict(quicksort_iter_first)
print("quicksort_iter_first", *zip(*quicksort_iter_first))

quicksort_recur_median = {
    1000000: [3470505, 3619783, 3551294, 3457960, 3631751, 3651735, 3690957, 3516832, 3364776, 3363512],
    100: [5, 21, 7, 7, 13, 7, 12, 7, 7, 20],
    1000: [46, 44, 44, 45, 44, 47, 45, 44, 43, 45],
    10000: [813, 913, 953, 817, 802, 818, 818, 819, 858, 821],
    100000: [40109, 40542, 40180, 39990, 39805, 40222, 40051, 40161, 40792, 40746],
}
quicksort_recur_median = avg_dict(quicksort_recur_median)
print("quicksort_recur_median", *zip(*quicksort_recur_median))

quicksort_recur_random = {
    100: [9, 24, 9, 8, 9, 8, 9, 9, 14, 9],
    1000000: [4670407, 4850676, 4840055, 4632988, 4776003, 4699205, 4893634, 4702358, 4421663, 4402728],
    1000: [53, 49, 51, 52, 54, 54, 51, 51, 50, 54],
    100000: [49672, 51293, 52843, 50181, 50992, 50829, 52166, 51935, 50923, 50241],
    10000: [1075, 1074, 1065, 1294, 1248, 1114, 1148, 1082, 1181, 1110]
}
quicksort_recur_random = avg_dict(quicksort_recur_random)
print("quicksort_recur_random", *zip(*quicksort_recur_random))

quicksort_recur_first = {
    1000: [66, 58, 47, 50, 48, 47, 47, 74, 46, 46],
    100: [23, 8, 14, 8, 14, 13, 7, 6, 6, 17],
    1000000: [5227339, 5450626, 5416035, 5226109, 5469028, 5477962, 5596628, 5305985, 5061921, 5069208],
    10000: [1082, 955, 982, 1046, 991, 1067, 989, 1057, 1000, 966],
    100000: [57078, 57036, 57605, 57116, 56775, 56853, 56750, 56617, 56997, 57093]
}
quicksort_recur_first = avg_dict(quicksort_recur_first)
print("quicksort_recur_first", *zip(*quicksort_recur_first))

insertion_iter = {
    10000: [37089, 36545, 36555, 37101, 36830, 36791, 36993, 36938, 36787, 36899],
    1000000: [367342613, 373736886, 363999797, 369020516, 369867043, 379637928, 372415143, 352491997, 344477930, 344628647],
    1000: [357, 390, 366, 365, 359, 362, 358, 384, 388, 360],
    100: [13, 25, 9, 10, 9, 9, 9, 15, 20, 9],
    100000: [3561327, 3563515, 3558224, 3554701, 3552193, 3554660, 3548272, 3548997, 3596756, 3554774]
}
insertion_iter = avg_dict(insertion_iter)
print("insertion_iter", *zip(*insertion_iter))

insertion_recur = {
    1000000: [375032117, 373578084, 370137292, 372846169, 372916966, 375875795, 363052674, 344744233, 344715423, 344552942],
    10000: [36637, 36539, 36912, 36447, 37279, 36278, 36753, 36676, 36970, 36607],
    1000: [519, 495, 364, 377, 363, 401, 362, 364, 375, 361],
    100000: [3558215, 3556506, 3555398, 3547279, 3543047, 3560617, 3545120, 3553899, 3550975, 3549052],
    100: [85, 13, 10, 10, 9, 10, 10, 10, 7, 27]
}
insertion_recur = avg_dict(insertion_recur)
print("insertion_recur", *zip(*insertion_recur))

binary_search = {
    100: [4, 5, 4, 3, 6, 4, 4, 4, 4, 5],
    100000: [5, 4, 4, 4, 4, 4, 4, 3, 4, 4],
    1000: [4, 4, 5, 3, 5, 3, 7, 6, 3, 5],
    1000000: [5, 5, 3, 7, 4, 6, 5, 7, 4, 4],
    10000: [4, 4, 4, 3, 4, 4, 29, 3, 3, 5]
}
binary_search = avg_dict(binary_search)
print("binary_search", *zip(*binary_search))

t = np.arange(100, 1000000, 100)
nlogn = np.log2(t) * t
quadratic = t * t * 0.001

plt.plot(t, t, "--", label="linear")
plt.plot(t, nlogn, "--", label="nlogn")
plt.plot(t, quadratic, "--", label="quadratic")

plt.plot(*quicksort_iter_median, marker=".", label="quicksort iterable median pivot")
plt.plot(*quicksort_iter_first, marker=".", label="quicksort iterable first")
plt.plot(*quicksort_iter_random, marker=".", label="quicksort iterable random")

plt.plot(*quicksort_recur_median, marker=".", label="quicksort recursive median pivot")
plt.plot(*quicksort_recur_first, marker=".", label="quicksort recursive first")
plt.plot(*quicksort_recur_random, marker=".", label="quicksort recursive random")


plt.plot(*insertion_iter, marker=".", label="insertion sort iterable")
plt.plot(*insertion_recur, marker=".", label="insertion sort recursive")

ax = plt.gca()
ax.ticklabel_format(useOffset=False, style='plain')
# ax.set_xticks([10, 100, 1000, 10000, 100000, 1000000])
# ax.set_xscale('log')
ax.set_yscale('log')

plt.legend()
# plt.show()
# plt.savefig("images/graph.pgf")

# plt.plot(t, quadratic, label="quadratic")
#
# plt.plot(*insertion_iter, label="insertion sort iterable")
# plt.plot(*insertion_recur, label="insertion sort recursive")
#
# ax = plt.gca()
# # ax.set_xscale('log')
# # ax.set_yscale('log')
#
# plt.legend()
# plt.show()
