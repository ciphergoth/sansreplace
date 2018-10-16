#include <algorithm>
#include <random>

#include "testfunc.h"

static std::mt19937_64 generator;

void cardchoose(int n, int k, int *result) {
    auto t = n - k + 1;
    for (int i = 0; i < k; i++) {
        std::uniform_int_distribution<int> distribution(0, t + i - 1);
        int r = distribution(generator);
        if (r < t) {
            result[i] = r;
        } else {
            result[i] = result[r - t];
        }
    }
    std::sort(result, result + k);
#if 0
    for (int i = 0; i < k; i++) {
        result[i] += i;
    }
#else
    for (int i = 0; i < k; i++) {
        std::uniform_int_distribution<int> distribution(0, i);
        int r = distribution(generator);
        auto t = result[i] + i;
        result[i] = result[r];
        result[r] = t;
    }
#endif
}
