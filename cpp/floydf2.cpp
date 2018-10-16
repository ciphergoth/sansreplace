#include <algorithm>
#include <random>
#include <unordered_set>

#include "testfunc.h"

static std::mt19937_64 generator;

void floydf2(int n, int k, int *result) {
    std::unordered_set<int> done;
    for (int i = 0; i < k; i++) {
        std::uniform_int_distribution<int> distribution(0, n - i - 1);
        int r = distribution(generator);
        if (done.find(r) == done.end()) {
            done.insert(r);
            result[i] = r;
        } else {
            result[i] = n - i - 1;
        }
    }
}
