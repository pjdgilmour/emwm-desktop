/* SPDX-License-Identifier: GPL-2.0-or-later
 * PCManFM 1.4.0 desktop.c with EMWM patches 0002 and 0004. Scalar
 * adapters for load_items/save_item_pos, on_snap_to_grid and move_item
 * allow rust-anything to run the original math without a GTK display.
 * Copyright 2010-2012 Hong Jen Yee, 2012-2017 Andriy Grytsenko, 2024 Ingo Brückl.
 */
#include <math.h>

int anchor_x(int x, int old_width, int was_right, int origin,
             int width, int cell, int right)
{
    if (was_right && old_width > 0)
        x += width - old_width;
    if (!!was_right != !!right)
        x = 2 * origin + width - x - cell;
    return x;
}

int snap_axis(int value, int origin, int cell)
{
    /* GTK cell dimensions are positive after allocation. */
    if (cell <= 0) return value;
    double x = (double)(value - origin) / cell;
    return origin + ((x > 0.0) ? floor(x + 0.5) : ceil(x - 0.5)) * cell;
}

int drag_axis(int value, int origin, int width, int margin, int item)
{
    if (value > origin + width - margin - item)
        value = origin + width - margin - item;
    if (value < origin + margin)
        value = origin + margin;
    return value;
}
