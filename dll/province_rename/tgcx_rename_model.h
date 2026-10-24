#pragma once
#include <stdint.h>
#include <string.h>

constexpr unsigned tgcx_rename_max=80;
constexpr int tgcx_rename_panel_width=332,tgcx_rename_panel_height=224;
constexpr int tgcx_rename_entry_width=24,tgcx_rename_entry_height=24;
struct tgcx_RenamePlacement {int entry_x,entry_y,panel_x,panel_y;};
static bool tgcx_RenameLayout(int host_x,int host_y,int host_width,int name_x,int name_y,int name_width,int name_height,
                             int screen_width,int screen_height,tgcx_RenamePlacement* out) {
    if(host_width<160 || host_width>2048 || screen_width<tgcx_rename_panel_width ||
       screen_height<tgcx_rename_panel_height || name_x<0 || name_y<0 || name_width<0 ||
       name_width>4096 || name_height<1 || name_height>128 || name_y>screen_height)return false;
    int ex=name_x+name_width+10,ey=name_y+(name_height-tgcx_rename_entry_height)/2-2;
    int right=host_x+host_width-tgcx_rename_entry_width-8;
    if(ex>right)ex=right;if(ex<name_x || ex+tgcx_rename_entry_width>screen_width)return false;
    if(ey<0)ey=0;if(ey+tgcx_rename_entry_height>screen_height)ey=screen_height-tgcx_rename_entry_height;
    int px=host_x+host_width+20,py=host_y;
    if(px+tgcx_rename_panel_width>screen_width) {
        if(host_y>=tgcx_rename_panel_height+8){px=host_x;py=host_y-tgcx_rename_panel_height-8;}
        else px=screen_width-tgcx_rename_panel_width;
    }
    if(px<0)px=0;if(px+tgcx_rename_panel_width>screen_width)px=screen_width-tgcx_rename_panel_width;
    if(py<0)py=0;if(py+tgcx_rename_panel_height>screen_height)py=screen_height-tgcx_rename_panel_height;
    *out={ex,ey,px,py};return true;
}
// The executable stores Western single-byte text. Reject save/format controls;
// names are passed as data in CChangeProvinceNameCommand, never as script.
static bool tgcx_RenameName(const char* input,unsigned size,char (&out)[81]) {
    out[0]=0;
    if(!input || !size || size>tgcx_rename_max)return false;
    unsigned first=0,last=size;
    while(first<last && input[first]==' ')++first;
    while(last>first && input[last-1]==' ')--last;
    if(first==last)return false;
    for(unsigned i=first;i<last;++i) {
        unsigned char c=static_cast<unsigned char>(input[i]);
        if(c<32 || c==127 || c=='"' || c=='\\' || c==0xA7 ||
           c==0x81 || c==0x8D || c==0x8F || c==0x90 || c==0x9D)return false;
    }
    memcpy(out,input+first,last-first);out[last-first]=0;return true;
}

struct tgcx_RenameTarget {uintptr_t world,province;int id,player;uint32_t tag;};
static bool tgcx_RenameSame(const tgcx_RenameTarget& a,const tgcx_RenameTarget& b) {
    return a.world && a.world==b.world && a.province==b.province && a.id==b.id && a.player==b.player && a.tag==b.tag;
}

// 24-byte string payload plus spare storage matching the interval between
// command+0x3C and +0x58. The spare word's meaning is not identified.
// Borrowed inputs never go to a game destructor.
struct tgcx_RenameString {char data[16];uint32_t length,capacity,reserved;};
static tgcx_RenameString tgcx_RenameBorrow(const char* value) {
    tgcx_RenameString s={};s.length=static_cast<uint32_t>(strlen(value));s.capacity=15;
    if(s.length<16)memcpy(s.data,value,s.length+1);
    else {*reinterpret_cast<const char**>(s.data)=value;s.capacity=s.length;}
    return s;
}
