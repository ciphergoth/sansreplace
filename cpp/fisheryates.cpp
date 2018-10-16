#include <algorithm>
#include <random>
#include <vector>

#include "testfunc.h"

static std::mt19937_64 generator;

void fisheryates(int n, int k, int *result) {
    std::vector<int> options(n);
    for (int i = 0; i < n; i++) {
        options[i] = i;
    }
    for (int i = 0; i < k; i++) {
        std::uniform_int_distribution<int> distribution(i, n - 1);
        int r = distribution(generator);
        result[i] = options[r];
        options[r] = options[i];
    }
}
