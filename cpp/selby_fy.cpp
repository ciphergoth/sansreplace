#include <algorithm>
#include <random>
#include <unordered_map>

#include "testfunc.h"

static std::mt19937_64 generator;

void selby_fy(int n, int k, int *result) {
    std::unordered_map<int, int> options;
    std::uniform_int_distribution<int> distribution(0, n - 1);
    for (int i = 0; i < k; i++) {
        std::uniform_int_distribution<int> distribution(i, n - 1);
        int r = distribution(generator);
        auto fr = options.find(r);
        if (fr == options.end()) {
            result[i] = r;
        } else {
            result[i] = fr->second;
        }
        auto fi = options.find(i);
        if (fi == options.end()) {
            options[r] = i;
        } else {
            options[r] = fi->second;
        }
    }
}
