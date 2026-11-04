// SPDX-License-Identifier: GPL-3.0-only
#include "effects.h"
#include <cstdlib>
#include <iostream>
static void check(bool value) { if (!value) { std::cerr << "invalid animation geometry\n"; std::exit(1); } }
int main() {
    Rect a{10,20,800,600}, b{300,700,48,48};
    check(ease(-1)==0 && ease(2)==1);
    check(interpolate(a,b,ease(0)).w==800 && interpolate(a,b,ease(1)).x==300);
    for (auto e : {Effect::Zoom, Effect::Twist, Effect::Flip}) {
        for (int i=0;i<=100;i++) {
            double t=i/100.; auto r=interpolate(a,b,ease(t));
            check(r.w>0 && r.h>0);
            for (auto p : outline(r,e,t)) check(std::isfinite(p.x) && std::isfinite(p.y));
        }
        auto end=outline(b,e,1), expected=rectangle(b);
        for (int i=0;i<4;i++) check(std::abs(end[i].x-expected[i].x)<.001 && std::abs(end[i].y-expected[i].y)<.001);
    }
}
