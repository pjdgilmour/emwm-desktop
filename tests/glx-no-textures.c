/* SPDX-License-Identifier: GPL-3.0-only
 * Fault injection: a GLX driver with contexts but no bindable pixmap textures.
 * Exercises failure while initializing shadows, before the first paint. */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <GL/glx.h>
#include <GL/glxext.h>
int glXGetFBConfigAttrib(Display *d, GLXFBConfig c, int attribute, int *value) {
    if (attribute == GLX_BIND_TO_TEXTURE_RGB_EXT || attribute == GLX_BIND_TO_TEXTURE_RGBA_EXT) {
        *value = 0;
        return Success;
    }
    typedef int (*Getter)(Display *, GLXFBConfig, int, int *);
    Getter real = (Getter)dlsym(RTLD_NEXT, "glXGetFBConfigAttrib");
    return real(d, c, attribute, value);
}
