/* SPDX-License-Identifier: GPL-3.0-only
 * Isolated Xvfb fixture. Only RandR monitor discovery is replaced. */
#include <X11/Xlib.h>
#include <X11/extensions/Xrandr.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
static int count=2, first=0, origin=0;
static void read_layout(void) {
    int primary;
    FILE *f=fopen(getenv("EMWM_TEST_TOPOLOGY"),"r");
    first=origin=0;
    if(f){ if(fscanf(f,"%d %d %d %d",&count,&primary,&first,&origin)<2)count=2; fclose(f); }
}
XRRScreenResources *XRRGetScreenResourcesCurrent(Display *d, Window w) {
    read_layout();
    XRRScreenResources *r=calloc(1,sizeof(*r)+2*sizeof(RROutput));
    r->noutput=count; r->outputs=(RROutput *)(r+1);
    for(int i=0;i<count;i++)r->outputs[i]=first+i+1;
    return r;
}
void XRRFreeScreenResources(XRRScreenResources *r) { free(r); }
XRROutputInfo *XRRGetOutputInfo(Display *d,XRRScreenResources *r,RROutput id) {
    XRROutputInfo *v=calloc(1,sizeof(*v)+16);
    v->connection=RR_Connected;v->crtc=id;v->name=(char *)(v+1);
    strcpy(v->name,id==1?"TEST-A":"TEST-B");v->nameLen=strlen(v->name);return v;
}
void XRRFreeOutputInfo(XRROutputInfo *v) {free(v);}
XRRCrtcInfo *XRRGetCrtcInfo(Display *d,XRRScreenResources *r,RRCrtc id) {
    XRRCrtcInfo *v=calloc(1,sizeof(*v));
    v->x=origin+((int)id-first-1)*800;v->width=800;v->height=600;v->rotation=RR_Rotate_0;return v;
}
void XRRFreeCrtcInfo(XRRCrtcInfo *v) {free(v);}
XRRMonitorInfo *XRRGetMonitors(Display *d,Window w,Bool active,int *n) {
    read_layout();*n=count;
    XRRMonitorInfo *v=calloc(count,sizeof(*v));
    for(int i=0;i<count;i++) {
        v[i].x=origin+i*800;v[i].width=800;v[i].height=600;v[i].primary=(i==count-1);
    }
    return v;
}
void XRRFreeMonitors(XRRMonitorInfo *v) {free(v);}
