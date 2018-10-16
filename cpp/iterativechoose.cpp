#include <algorithm>
#include <random>
#include <vector>

#include "testfunc.h"

static std::mt19937_64 generator;

void iterativechoose(int n, int k, int *result) {
    int ix = 0;
    for (int i = 0; i < n; i++) {
        std::uniform_int_distribution<int> distribution(0, n - i - 1);
        int r = distribution(generator);
        if (r < k - ix) {
            result[ix++] = i;
        }
    }
}
