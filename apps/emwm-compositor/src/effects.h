// SPDX-License-Identifier: GPL-2.0-or-later
// Outline geometry adapted from Window Maker src/actions.c:
// Copyright (c) 1997-2003 Alfredo K. Kojima
// Copyright (c) 1998-2003 Dan Pascu
// Copyright (c) 2014-2026 Window Maker Team
// Adaptations for EMWM Desktop: time-based interpolation and pure geometry.
// Distributed without warranty; see COPYING.WindowMaker. The combined
// compositor is distributed under GPL-3.0-only.
#pragma once
#include <algorithm>
#include <array>
#include <cmath>

struct Rect { double x=0, y=0, w=1, h=1; };
struct Point { double x, y; };
using Quad = std::array<Point, 4>;
enum class Effect { Disabled, Zoom, Twist, Flip };
inline double ease(double t) {
    t = std::clamp(t, 0.0, 1.0);
    return t*t*(3-2*t);
}
inline Rect interpolate(Rect a, Rect b, double t) {
    return {a.x+(b.x-a.x)*t, a.y+(b.y-a.y)*t,
            a.w+(b.w-a.w)*t, a.h+(b.h-a.h)*t};
}
inline Quad rectangle(Rect r) {
    return {{{r.x,r.y}, {r.x+r.w,r.y}, {r.x+r.w,r.y+r.h}, {r.x,r.y+r.h}}};
}
// The outline styles follow Window Maker's animateResizeZoom/Twist/Flip
// (actions.c). Timing and rendering are independent and never grab the server.
inline Quad outline(Rect r, Effect effect, double progress) {
    constexpr double pi = 3.14159265358979323846;
    auto q = rectangle(r);
    double angle = 2*pi*progress;
    if (effect == Effect::Twist) {
        for (auto &p : q) {
            double x=p.x-r.x-r.w/2, y=p.y-r.y-r.h/2;
            p = {r.x+r.w/2 + x*cos(angle)-y*sin(angle),
                 r.y+r.h/2 + x*sin(angle)+y*cos(angle)};
        }
    } else if (effect == Effect::Flip) {
        double dx = r.w*.1*sin(angle), h=r.h*.5*cos(angle), cy=r.y+r.h/2;
        q = {{{r.x+dx,cy-h}, {r.x+r.w-dx,cy-h},
              {r.x+r.w+dx,cy+h}, {r.x-dx,cy+h}}};
    }
    return q;
}
