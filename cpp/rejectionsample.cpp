#include <algorithm>
#include <random>
#include <unordered_set>

#include "testfunc.h"

static std::mt19937_64 generator;

void rejectionsample(int n, int k, int *result) {
    std::unordered_set<int> done;
    std::uniform_int_distribution<int> distribution(0, n - 1);
    for (int i = 0; i < k;) {
        int r = distribution(generator);
        if (done.find(r) == done.end()) {
            done.insert(r);
            result[i++] = r;
        }
    }
}
