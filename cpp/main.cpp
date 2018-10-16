#include <algorithm>
#include <chrono>
#include <iostream>
#include <ratio>
#include <vector>

#include "testfunc.h"

static std::chrono::duration<double> timefunc(int iters, int n, int k,
    void (*func)(int n, int k, int *result)) {
    auto result = std::vector<int>(n);
    auto start = std::chrono::system_clock::now();
    for (int i = 0; i < iters; i++) {
        func(n, k, &result[0]);
    }
    auto end = std::chrono::system_clock::now();
    return end - start;
}

static int choose_iters(
    std::chrono::duration<double> totake,
    int n, int k,
    void (*func)(int n, int k, int *result)) {
    int iters = 1;
    for (;;) {
        auto t = timefunc(iters, n, k, func);
        if (t * 10 >= totake) {
            return std::max(1, static_cast<int>(totake * iters / t));
        }
        iters *= 10;
    }
}

static std::chrono::duration<double> timefunc_for(
    std::chrono::duration<double> totake,
    int n, int k,
    void (*func)(int n, int k, int *result)) {
    int iters = choose_iters(totake, n, k, func);
    auto t = timefunc(iters, n, k, func);
    return t / iters;
}

int main() {
    std::cout << "Hello, World!" << std::endl;
    std::chrono::seconds sec(1);
    auto time_s = timefunc_for(sec, 100, 10, donothing).count();
    std::cout << time_s << std::endl;
    return 0;
}
