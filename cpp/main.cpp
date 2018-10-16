#include <algorithm>
#include <chrono>
#include <iostream>
#include <ratio>
#include <string>
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

struct timeable {
    std::string name;
    void (*func)(int n, int k, int *result);
};

struct timeable totime[] = {
    {"donothing", donothing},
    {"cardchoose", cardchoose},
    {"fisheryates", fisheryates},
    {"floydf2", floydf2},
    {"rejectionsample", rejectionsample},
    {"", nullptr}
};

static void time_all(int n, int k) {
    std::chrono::seconds sec(1);
    struct timeable *tm;
    for (tm = totime; tm->func; tm++) {
        auto time_s = timefunc_for(sec, n, k, tm->func).count();
        std::cout << tm->name << " " << time_s << std::endl;
    }
}

int main() {
    for (int n = 10; n < 10000000; n *= 10) {
        for (int k = 8; k < n; k *= 10) {
            std::cout << "n, k = " << n << ", " << k << std::endl;
            time_all(n, k);
            std::cout << std::endl;
        }
    }
    return 0;
}
