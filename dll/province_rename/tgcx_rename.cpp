// Standalone province rename plugin. Simulation changes use the engine command.
#define WIN32_LEAN_AND_MEAN
#define NOMINMAX
#include <windows.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <math.h>
#include "tgcx_rename_model.h"
#include "tgcx_rename_signatures.h"
#include "../tgcx/tgcx_sha256.h"
#include "../tgcx/tgcx_packages.h"

static_assert(sizeof(void*)==4,"Victoria II requires x86");
static uintptr_t tgcx_rename_base=0;
static HANDLE tgcx_rename_log=INVALID_HANDLE_VALUE;
static void tgcx_RenameLog(const char* fmt,...) {
    if(tgcx_rename_log==INVALID_HANDLE_VALUE)return;
    char line[768];va_list args;va_start(args,fmt);
    int size=_vsnprintf_s(line,sizeof(line)-2,_TRUNCATE,fmt,args);va_end(args);
    if(size<0)size=static_cast<int>(strlen(line));line[size++]='\r';line[size++]='\n';
    DWORD written=0;WriteFile(tgcx_rename_log,line,static_cast<DWORD>(size),&written,nullptr);
}
template<class T> static T tgcx_RenameRead(uintptr_t p) {return *reinterpret_cast<T*>(p);}
typedef uintptr_t(__fastcall* tgcx_RenameGet)(uintptr_t,void*);
typedef uintptr_t(__fastcall* tgcx_RenameOne)(uintptr_t,void*,const void*);
typedef void(__fastcall* tgcx_RenameZero)(uintptr_t,void*);

static bool tgcx_RenameStringRead(uintptr_t p,char* out,unsigned capacity) {
    const auto* s=reinterpret_cast<const tgcx_RenameString*>(p);
    if(s->length>=capacity || s->capacity<s->length || s->capacity>1048576)return false;
    const char* text=s->capacity>15?*reinterpret_cast<const char* const*>(s->data):s->data;
    if(!text || text[s->length])return false;
    memcpy(out,text,s->length);out[s->length]=0;return true;
}

static bool tgcx_RenameSnapshot(uintptr_t view,tgcx_RenameTarget* out) {
    *out={};
    if(!view || tgcx_RenameRead<uintptr_t>(view)!=tgcx_rename_base+0xA13768)return false;
    uintptr_t world=tgcx_RenameRead<uintptr_t>(tgcx_rename_base+0xE588E8);
    uintptr_t province=tgcx_RenameRead<uintptr_t>(view+0x24);
    uintptr_t manager=tgcx_RenameRead<uintptr_t>(tgcx_rename_base+0xE587E4);
    if(!world || !province || !manager)return false;
    int id=tgcx_RenameRead<int>(province+0x58),player=tgcx_RenameRead<int>(world+0xB60);
    uintptr_t first=tgcx_RenameRead<uintptr_t>(world+0xACC),last=tgcx_RenameRead<uintptr_t>(world+0xAD0);
    if(!first || last<first || (last-first)%4 || (last-first)/4>65536 || id<=0 || static_cast<unsigned>(id)>=(last-first)/4 ||
       tgcx_RenameRead<uintptr_t>(first+id*4)!=province)return false;
    if(player<=0 || player>=tgcx_RenameRead<int>(manager) || tgcx_RenameRead<int>(province+0x12C)!=player)return false;
    uintptr_t countries=tgcx_RenameRead<uintptr_t>(manager+4);
    uintptr_t country=countries?tgcx_RenameRead<uintptr_t>(countries+player*4):0;
    if(!country || tgcx_RenameRead<int>(country+0x20)!=player ||
       memcmp(reinterpret_cast<void*>(country+0x1C),reinterpret_cast<void*>(world+0xB5C),3))return false;
    uintptr_t map=tgcx_RenameRead<uintptr_t>(province+0x5C);
    if(!map || !tgcx_RenameRead<unsigned char>(map+0x2A))return false;
    *out={world,province,id,player,tgcx_RenameRead<uint32_t>(country+0x1C)};return true;
}

static uintptr_t tgcx_RenameFind(uintptr_t host,unsigned slot,uint32_t expected,const char* key) {
    if(!host)return 0;auto* table=tgcx_RenameRead<uintptr_t*>(host);
    if(table[slot]!=tgcx_rename_base+expected)return 0;
    auto name=tgcx_RenameBorrow(key);
    return reinterpret_cast<tgcx_RenameOne>(table[slot])(host,nullptr,&name);
}
static void tgcx_RenameVisible(uintptr_t window,bool show) {
    if(!window)return;auto* vt=tgcx_RenameRead<uintptr_t*>(window);
    if(show) {
        if(tgcx_RenameRead<unsigned char>(window+0x67))return;
        uintptr_t gui=window+0x18;auto* secondary=tgcx_RenameRead<uintptr_t*>(gui);
        if(secondary[13]==tgcx_rename_base+0x5E4490)
            reinterpret_cast<tgcx_RenameZero>(secondary[13])(gui,nullptr);
    } else if(vt[9]==tgcx_rename_base+0x5E4030)
        reinterpret_cast<tgcx_RenameZero>(vt[9])(window,nullptr);
}
static uintptr_t tgcx_RenameContext(uintptr_t root) {
    uintptr_t context=tgcx_RenameRead<uintptr_t>(root+0x4C);
    if(!context || tgcx_RenameRead<uintptr_t>(context)!=tgcx_rename_base+0xA08A44 ||
       tgcx_RenameRead<uintptr_t>(context+0x30)!=tgcx_RenameRead<uintptr_t>(root+0x90))return 0;
    return context;
}
static uintptr_t tgcx_RenameLookup(uintptr_t registry,const char* key) {
    auto name=tgcx_RenameBorrow(key);
    auto* reg=tgcx_RenameRead<uintptr_t*>(registry);
    if(reg[1]<tgcx_rename_base || reg[1]>=tgcx_rename_base+tgcx_rename_image_size)return 0;
    return reinterpret_cast<tgcx_RenameOne>(reg[1])(registry,nullptr,&name);
}
static uintptr_t tgcx_RenameDefinition(uintptr_t context,const char* key) {
    return tgcx_RenameLookup(context+0x34,key);
}
static bool tgcx_RenameSchema(uintptr_t root) {
    uintptr_t context=tgcx_RenameContext(root);if(!context)return false;
    // P59: the native button builder calls slot 11 on a name-resolved type.
    // A label with the same name can resolve to a destructor thunk instead.
    const char* keys[]={"tgcx_rename_open","tgcx_rename_apply","tgcx_rename_cancel"};
    const char* roots[]={"tgcx_rename_entry","tgcx_rename_window","tgcx_rename_window"};
    for(unsigned i=0;i<_countof(keys);++i) {
        // Match FUN_009CFF20: buttons are in CWindowType+0x150, not the
        // CEU3Gui+0x34 registry which resolves the owning window definition.
        uintptr_t definition=tgcx_RenameDefinition(context,roots[i]);
        uintptr_t type=definition?tgcx_RenameLookup(definition+0x150,keys[i]):0;
        uintptr_t table=type?tgcx_RenameRead<uintptr_t>(type):0;
        if(table!=tgcx_rename_base+0xA3CFBC || tgcx_RenameRead<uintptr_t>(table+11*4)!=tgcx_rename_base+0x5F0310) {
            tgcx_RenameLog("Rename schema refused before construction: %s/%s type=%08X table=%08X; expected native CButtonType",roots[i],keys[i],type,table);return false;
        }
    }
    return true;
}
static uintptr_t tgcx_RenameMake(uintptr_t root,const char* key) {
    uintptr_t context=tgcx_RenameContext(root);if(!context || !tgcx_RenameDefinition(context,key))return 0;
    auto name=tgcx_RenameBorrow(key);
    auto* vt=tgcx_RenameRead<uintptr_t*>(context);
    if(vt[11]!=tgcx_rename_base+0x5A38F0)return 0;
    typedef uintptr_t(__fastcall* Make)(uintptr_t,void*,const void*,uintptr_t);
    uintptr_t window=reinterpret_cast<Make>(vt[11])(context,nullptr,&name,0);
    if(window && tgcx_RenameRead<uintptr_t>(window)!=tgcx_rename_base+0xA3C0D8)return 0;
    tgcx_RenameLog("Rename native root constructed: %s=%08X",key,window);
    tgcx_RenameVisible(window,false);return window;
}
static void tgcx_RenameRelease(uintptr_t window) {
    if(!window)return;tgcx_RenameVisible(window,false);auto* vt=tgcx_RenameRead<uintptr_t*>(window);
    if(vt[20]==tgcx_rename_base+0x5E3D60)reinterpret_cast<tgcx_RenameZero>(vt[20])(window,nullptr);
}
static bool tgcx_RenamePlace(uintptr_t window,int x,int y,bool promote) {
    if(!window || x<0 || y<0 || x>32767 || y>32767)return false;
    auto* vt=tgcx_RenameRead<uintptr_t*>(window);
    if(vt[6]!=tgcx_rename_base+0x5E3BA0 || vt[11]!=tgcx_rename_base+0x5E2FC0)return false;
    if(promote)reinterpret_cast<tgcx_RenameZero>(vt[11])(window,nullptr);
    typedef void(__fastcall* Position)(uintptr_t,void*,uint32_t);
    reinterpret_cast<Position>(vt[6])(window,nullptr,static_cast<unsigned>(x)|(static_cast<unsigned>(y)<<16));return true;
}
static bool tgcx_RenameInnerScreen(uintptr_t inner,uintptr_t renderer,int* x,int* y,int* width,int* height) {
    if(!inner || !renderer)return false;float sx=0,sy=0;
    for(unsigned n=0;n<8;++n) {
        uintptr_t table=tgcx_RenameRead<uintptr_t>(inner);
        if(table<tgcx_rename_base || table>=tgcx_rename_base+tgcx_rename_image_size)return false;
        sx+=tgcx_RenameRead<float>(inner+0x84);sy-=tgcx_RenameRead<float>(inner+0x88);
        uintptr_t parent=tgcx_RenameRead<uintptr_t>(inner+0xE4);
        if(parent==renderer+0x6B9E0) {
            *width=tgcx_RenameRead<int>(renderer+0x6B9BC);*height=tgcx_RenameRead<int>(renderer+0x6B9C0);
            if(!isfinite(sx) || !isfinite(sy) || sx<0 || sy<0 || sx>32767 || sy>32767 ||
               *width<640 || *height<480 || *width>32767 || *height>32767)return false;
            *x=static_cast<int>(sx);*y=static_cast<int>(sy);return true;
        }
        if(!parent)return false;inner=parent;
    }
    return false;
}
static bool tgcx_RenameScreen(uintptr_t root,int* x,int* y,int* width,int* height) {
    return tgcx_RenameInnerScreen(tgcx_RenameRead<uintptr_t>(root+0x494),
                                tgcx_RenameRead<uintptr_t>(root+0x90),x,y,width,height);
}
static bool tgcx_RenameFontMetrics(uintptr_t font,const char* shown,int* width,int* height) {
    if(!font || !shown)return false;
    auto* vt=tgcx_RenameRead<uintptr_t*>(font);
    if(vt[10]!=tgcx_rename_base+0x624E20 || vt[11]!=tgcx_rename_base+0x624F90)return false;
    typedef int(__fastcall* Measure)(uintptr_t,void*,const char*,int);
    *width=reinterpret_cast<Measure>(vt[10])(font,nullptr,shown,static_cast<int>(strlen(shown)));
    *height=static_cast<int>(reinterpret_cast<tgcx_RenameGet>(vt[11])(font,nullptr));
    return *width>=0 && *width<=4096 && *height>0 && *height<=128;
}
static bool tgcx_RenameNameMetrics(uintptr_t inner,int* width,int* height) {
    if(!inner)return false;char shown[1024];
    if(!tgcx_RenameStringRead(inner+0x150,shown,sizeof(shown)))return false;
    return tgcx_RenameFontMetrics(tgcx_RenameRead<uintptr_t>(inner+0x1BC),shown,width,height);
}
static bool tgcx_RenameAnchor(uintptr_t root,tgcx_RenamePlacement* out,bool* measured=nullptr,const char** failure=nullptr) {
    auto refuse=[failure](const char* stage){if(failure)*failure=stage;return false;};
    int hx=0,hy=0,w=0,h=0,nx=0,ny=0,nw=0,nh=0,tw=0,th=0;
    if(!tgcx_RenameScreen(root,&hx,&hy,&w,&h))return refuse("host render chain");
    // P44: province_name belongs to this direct native child, not the root.
    uintptr_t header=tgcx_RenameFind(root,27,0x5E4A20,"province_view_header");
    uintptr_t name=tgcx_RenameFind(header?header:root,15,0x5E4D00,"province_name");
    uintptr_t inner=name?tgcx_RenameRead<uintptr_t>(name+0xA0):0;
    if(!inner || !tgcx_RenameInnerScreen(inner,
        tgcx_RenameRead<uintptr_t>(root+0x90),&nx,&ny,&nw,&nh) || nw!=w || nh!=h)return refuse("province_name lookup/render chain");
    auto* text_table=tgcx_RenameRead<uintptr_t*>(inner);
    if(text_table[10]!=tgcx_rename_base+0x65EB00)return refuse("native text draw type");
    // P61: native text Draw uses these local offsets in addition to the
    // graphical parent matrix. They are not part of inner+84/-88.
    nx+=tgcx_RenameRead<int>(inner+0x188);ny+=tgcx_RenameRead<int>(inner+0x18C);
    // Native CFixedWindow constructor stores the actual size here. Definition
    // +A0 is an X position in the live CWindowType, not its width (P61).
    int host_width=tgcx_RenameRead<int>(root+0x4A0);
    bool exact=tgcx_RenameNameMetrics(inner,&tw,&th);
    if(!exact){tw=host_width-(nx-hx)-tgcx_rename_entry_width-16;th=20;}
    if(measured)*measured=exact;
    if(!tgcx_RenameLayout(hx,hy,host_width,nx,ny,tw,th,w,h,out))return refuse("measured geometry bounds");
    return true;
}
static void tgcx_RenameText(uintptr_t window,const char* key,const char* text) {
    uintptr_t box=tgcx_RenameFind(window,15,0x5E4D00,key);if(!box)return;
    auto value=tgcx_RenameBorrow(text);
    typedef void(__fastcall* Assign)(uintptr_t,void*,const void*,unsigned,unsigned);
    reinterpret_cast<Assign>(tgcx_rename_base+0x8ED0)(box+0xDC,nullptr,&value,0,0xFFFFFFFFu);
    uintptr_t inner=tgcx_RenameRead<uintptr_t>(box+0xA0);if(!inner)return;
    uintptr_t fn=tgcx_rename_base+0x65ECC0;const void* arg=&value;
    // from V2DLL: V2TechButton.cpp, native text rendering ESI calling convention.
    __asm {push esi} __asm {push 0} __asm {push arg} __asm {mov esi,inner} __asm {call fn} __asm {pop esi}
}

static uintptr_t tgcx_RenameQueue() {
    uintptr_t world=tgcx_RenameRead<uintptr_t>(tgcx_rename_base+0xE588E8);
    uintptr_t front=world?tgcx_RenameRead<uintptr_t>(world+0xB24):0;if(!front)return 0;
    uintptr_t session=reinterpret_cast<tgcx_RenameGet>(tgcx_RenameRead<uintptr_t*>(front)[17])(front,nullptr);
    return session?tgcx_RenameRead<uintptr_t>(session+0x38):0;
}
static void* tgcx_RenameCommand(int id,const char* name) {
    void* memory=reinterpret_cast<void*(__cdecl*)(size_t)>(tgcx_rename_base+0x6AE9AF)(0x5C);
    if(!memory)return nullptr;
    void* command=reinterpret_cast<void*(__stdcall*)(void*)>(tgcx_rename_base+0x17FF20)(memory);
    auto value=tgcx_RenameBorrow(name);
    typedef void(__fastcall* Assign)(uintptr_t,void*,const void*,unsigned,unsigned);
    reinterpret_cast<Assign>(tgcx_rename_base+0x8ED0)(reinterpret_cast<uintptr_t>(command)+0x3C,nullptr,&value,0,0xFFFFFFFFu);
    *reinterpret_cast<int*>(static_cast<char*>(command)+0x58)=id;return command;
}
static bool tgcx_RenameSend(uintptr_t view,const tgcx_RenameTarget& locked,const char* name) {
    tgcx_RenameTarget now={};char checked[81];
    if(!tgcx_RenameSnapshot(view,&now) || !tgcx_RenameSame(locked,now) ||
       !tgcx_RenameName(name,static_cast<unsigned>(strlen(name)),checked))return false;
    uintptr_t queue=tgcx_RenameQueue();if(!queue)return false;
    void* command=tgcx_RenameCommand(now.id,checked);if(!command)return false;
    typedef void(__fastcall* Post)(uintptr_t,void*,void*);
    reinterpret_cast<Post>(tgcx_RenameRead<uintptr_t*>(queue)[6])(queue,nullptr,command);
    tgcx_RenameLog("Rename queued: province=%d player=%d name=%s command=0x707",now.id,now.player,checked);return true;
}

struct tgcx_RenamePanel {
    uintptr_t view,root,entry,window,edit;DWORD thread;UINT_PTR timer;
    tgcx_RenameTarget target,queued_target;bool open,bound[3],fault,promote,pending,placed,anchor_reported,centered;
    char queued_name[81];uint32_t glue[3][11]; // Native observer: 44 bytes, DWORD alignment.
};
static tgcx_RenamePanel tgcx_rename_panel={};
static uintptr_t tgcx_rename_update_original=0,tgcx_rename_destroy_original=0,tgcx_rename_edit_original=0;

static bool tgcx_RenameInputInsets(uintptr_t edit,int* x,int* y) {
    auto& p=tgcx_rename_panel;
    if(!edit || edit!=p.edit || p.thread!=GetCurrentThreadId() ||
       tgcx_RenameRead<uintptr_t>(edit)!=tgcx_rename_base+0xA39878 ||
       tgcx_RenameRead<uintptr_t>(edit+0x54)!=tgcx_rename_base+0xA39938 ||
       tgcx_RenameRead<unsigned short>(edit+0x160)!=296 ||
       tgcx_RenameRead<unsigned short>(edit+0x162)!=30)return false;
    uintptr_t render=tgcx_RenameRead<uintptr_t>(edit+0x110);
    if(!render || tgcx_RenameRead<uintptr_t>(render)!=tgcx_rename_base+0xA3BB58)return false;
    char value[1024];int width=0,height=0;
    if(!tgcx_RenameStringRead(edit+0x54+0x18,value,sizeof(value)) ||
       !tgcx_RenameFontMetrics(tgcx_RenameRead<uintptr_t>(edit+0x54+0x78),value,&width,&height))return false;
    *x=(296-width)/2;*y=(30-height)/2;
    if(*x<8)*x=8;if(*y<2)*y=2;if(*y>10)*y=10;
    return true;
}
static void tgcx_RenameInputPosition(uintptr_t edit,int x,int y) {
    *reinterpret_cast<int*>(edit+0x150)=x;*reinterpret_cast<int*>(edit+0x154)=y;
    int width=0,height=0;
    if(tgcx_RenameFontMetrics(tgcx_RenameRead<uintptr_t>(edit+0xCC),"",&width,&height)) {
        // Native cursor is 32x32, visible ink (7,6)..(8,22). Native caret
        // measurement also adds a 3px gap. These caret-only fields leave
        // one pixel after the text and centre the 16px line in its font row.
        *reinterpret_cast<int*>(edit+0x184)=1-7-3;
        *reinterpret_cast<int*>(edit+0x188)=(height-16)/2-6;
    }
    // Native placement adds these insets to the control origin. The text
    // texture must have zero padding, or placement applies the offset twice.
    typedef void(__fastcall* Position)(uintptr_t,void*,uint32_t);
    reinterpret_cast<Position>(tgcx_rename_base+0x5B5A40)(edit,nullptr,tgcx_RenameRead<uint32_t>(edit+0x15C));
}
static void __cdecl tgcx_RenameEditRefresh(uintptr_t edit) {
    DWORD error=GetLastError();auto& p=tgcx_rename_panel;int x=0,y=0;bool centered=false;
    __try {
        centered=tgcx_RenameInputInsets(edit,&x,&y);
        if(centered) {
            *reinterpret_cast<int*>(edit+0x150)=0;*reinterpret_cast<int*>(edit+0x154)=0;
        }
    } __except(EXCEPTION_EXECUTE_HANDLER) {tgcx_RenameLog("Rename input centering refused expired control");}
    reinterpret_cast<void(__stdcall*)(uintptr_t)>(tgcx_rename_edit_original)(edit);
    __try {
        if(centered) {
            tgcx_RenameInputPosition(edit,x,y);
            if(!p.centered) {
                p.centered=true;tgcx_RenameLog("Rename centered input enabled; single native text/caret origin");
            }
        }
    } __except(EXCEPTION_EXECUTE_HANDLER) {tgcx_RenameLog("Rename centered input placement refused expired control");}
    SetLastError(error);
}
// Complete raster refresh and native placement synchronously on the UI thread;
// no intermediate frame sees the temporary zero texture padding.
__declspec(naked) static void tgcx_RenameEditRefreshHook() {
    __asm {pushfd} __asm {pushad} __asm {mov eax,[esp+40]} __asm {mov ebx,esp}
    __asm {sub esp,528} __asm {and esp,-16} __asm {fxsave [esp]} __asm {cld}
    __asm {push eax} __asm {call tgcx_RenameEditRefresh} __asm {add esp,4}
    __asm {fxrstor [esp]} __asm {mov esp,ebx} __asm {popad} __asm {popfd}
    __asm {ret 4}
}

static void tgcx_RenameClose() {
    tgcx_rename_panel.open=false;tgcx_rename_panel.target={};tgcx_RenameVisible(tgcx_rename_panel.window,false);
}
static void __cdecl tgcx_RenameClick(unsigned action) {
    auto& p=tgcx_rename_panel;
    __try {
        if(p.fault || p.thread!=GetCurrentThreadId() || !p.root || !tgcx_RenameRead<unsigned char>(p.root+0x67))return;
        if(!p.view || tgcx_RenameRead<uintptr_t>(p.view)!=tgcx_rename_base+0xA13768 ||
           tgcx_RenameRead<uintptr_t>(p.view+0x20)!=p.root ||
           tgcx_RenameRead<uintptr_t>(p.root)!=tgcx_rename_base+0xA3C0D8) {tgcx_RenameClose();return;}
        if(action==2){tgcx_RenameClose();return;}
        tgcx_RenameTarget now={};
        if(!tgcx_RenameSnapshot(p.view,&now)){tgcx_RenameClose();return;}
        if(action==0) {
            if(p.open){tgcx_RenameClose();return;}
            char current[512];uintptr_t map=tgcx_RenameRead<uintptr_t>(now.province+0x5C);
            if(!tgcx_RenameStringRead(map+0xE4,current,sizeof(current)))return;
            p.edit=tgcx_RenameFind(p.window,16,0x5E4D20,"tgcx_rename_name");if(!p.edit)return;
            uintptr_t buffer=p.edit+0x54;auto* vt=tgcx_RenameRead<uintptr_t*>(buffer);
            if(vt[16]!=tgcx_rename_base+0x5F5290 || vt[19]!=tgcx_rename_base+0x5F5950)return;
            auto text=tgcx_RenameBorrow(current);
            reinterpret_cast<tgcx_RenameOne>(vt[16])(buffer,nullptr,&text);
            reinterpret_cast<void(__stdcall*)(uintptr_t)>(tgcx_rename_base+0x5B6400)(p.edit);
            tgcx_RenameText(p.window,"tgcx_rename_current",current);
            tgcx_RenameText(p.window,"tgcx_rename_status","Up to 80 characters.");
            p.target=now;p.open=true;
            auto* table=tgcx_RenameRead<uintptr_t*>(p.window);
            reinterpret_cast<tgcx_RenameZero>(table[11])(p.window,nullptr);
            tgcx_RenameVisible(p.window,true);
            tgcx_RenameLog("Rename editor opened: province=%d",now.id);return;
        }
        if(action==1 && p.open) {
            if(!tgcx_RenameSame(p.target,now)){tgcx_RenameClose();return;}
            char input[4096],name[81];
            if(!p.edit || !tgcx_RenameStringRead(p.edit+0x54+0x18,input,sizeof(input)) ||
               !tgcx_RenameName(input,static_cast<unsigned>(strlen(input)),name)) {
                tgcx_RenameText(p.window,"tgcx_rename_status","Use 1-80 characters; no quotes, backslash or controls.");return;
            }
            if(tgcx_RenameSend(p.view,p.target,name)) {
                p.queued_target=p.target;strcpy_s(p.queued_name,name);p.pending=true;tgcx_RenameClose();
            }
            else tgcx_RenameText(p.window,"tgcx_rename_status","Rename unavailable. Select your province again.");
        }
    } __except(EXCEPTION_EXECUTE_HANDLER) {
        p.open=false;p.fault=true;tgcx_RenameLog("Rename disabled after a guarded native UI/command fault");
    }
}
// from V2DLL: V2TechButton.cpp observer thunk; preserve flags, registers and FP/SSE.
#define TGCX_RENAME_THUNK(n) \
__declspec(naked) static void tgcx_RenameThunk##n() { \
    __asm {pushfd} __asm {pushad} __asm {mov ebx,esp} \
    __asm {sub esp,528} __asm {and esp,-16} __asm {fxsave [esp]} \
    __asm {cld} __asm {push n} __asm {call tgcx_RenameClick} __asm {add esp,4} \
    __asm {fxrstor [esp]} __asm {mov esp,ebx} __asm {popad} __asm {popfd} __asm {ret} \
}
TGCX_RENAME_THUNK(0) TGCX_RENAME_THUNK(1) TGCX_RENAME_THUNK(2)
#undef TGCX_RENAME_THUNK
static void (*const tgcx_rename_thunks[])()={tgcx_RenameThunk0,tgcx_RenameThunk1,tgcx_RenameThunk2};

static bool tgcx_RenameBind(unsigned action) {
    auto& p=tgcx_rename_panel;if(p.bound[action])return true;
    const char* names[]={"tgcx_rename_open","tgcx_rename_apply","tgcx_rename_cancel"};
    uintptr_t button=tgcx_RenameFind(action?p.window:p.entry,13,0x5E4C80,names[action]);if(!button)return false;
    uintptr_t observable=button+0x54;auto* vt=tgcx_RenameRead<uintptr_t*>(observable);
    if(vt[1]<tgcx_rename_base || vt[1]>=tgcx_rename_base+tgcx_rename_image_size)return false;
    auto* glue=reinterpret_cast<unsigned char*>(p.glue[action]);memset(glue,0,44);
    *reinterpret_cast<uintptr_t*>(glue)=tgcx_rename_base+0xA0E48C;
    *reinterpret_cast<uintptr_t*>(glue+4)=reinterpret_cast<uintptr_t>(&p);
    *reinterpret_cast<uintptr_t*>(glue+8)=reinterpret_cast<uintptr_t>(tgcx_rename_thunks[action]);
    reinterpret_cast<tgcx_RenameOne>(vt[1])(observable,nullptr,glue);p.bound[action]=true;return true;
}
static void tgcx_RenameConfirm(const tgcx_RenameTarget& target) {
    auto& p=tgcx_rename_panel;if(!p.pending)return;
    if(!tgcx_RenameSame(p.queued_target,target)){p.pending=false;return;}
    char current[512];uintptr_t map=tgcx_RenameRead<uintptr_t>(target.province+0x5C);
    if(!tgcx_RenameStringRead(map+0xE4,current,sizeof(current)) || strcmp(current,p.queued_name))return;
    // The command updates map labels but does not rebuild the cached province
    // header. Refresh GUI only after observing the engine's applied name.
    p.pending=false;
    if(tgcx_rename_update_original)reinterpret_cast<void(__stdcall*)(uintptr_t)>(tgcx_rename_update_original)(p.view);
    p.promote=true;tgcx_RenameLog("Rename observed applied: province=%d name=%s; native header refreshed",target.id,current);
}
static void CALLBACK tgcx_RenameTimer(HWND,UINT,UINT_PTR,DWORD) {
    auto& p=tgcx_rename_panel;
    __try {
        if(!p.view || p.thread!=GetCurrentThreadId())return;
        if(p.fault){tgcx_RenameVisible(p.entry,false);tgcx_RenameVisible(p.window,false);return;}
        if(tgcx_RenameRead<uintptr_t>(p.view)!=tgcx_rename_base+0xA13768) {
            tgcx_RenameClose();tgcx_RenameVisible(p.entry,false);return;
        }
        uintptr_t root=tgcx_RenameRead<uintptr_t>(p.view+0x20);
        if(!root || tgcx_RenameRead<uintptr_t>(root)!=tgcx_rename_base+0xA3C0D8 ||
           !tgcx_RenameRead<unsigned char>(root+0x67)) {
            tgcx_RenameClose();tgcx_RenameVisible(p.entry,false);return;
        }
        if(p.root && root!=p.root) {
            tgcx_RenameClose();tgcx_RenameRelease(p.entry);tgcx_RenameRelease(p.window);
            p.entry=p.window=p.edit=0;p.placed=p.anchor_reported=p.centered=false;memset(p.bound,0,sizeof(p.bound));
        }
        p.root=root;tgcx_RenameTarget target={};
        if(!tgcx_RenameSnapshot(p.view,&target)) {tgcx_RenameClose();tgcx_RenameVisible(p.entry,false);return;}
        tgcx_RenameConfirm(target);
        if(p.open && !tgcx_RenameSame(p.target,target))tgcx_RenameClose();
        if((!p.entry || !p.window) && !tgcx_RenameSchema(root)) {p.fault=true;return;}
        if(!p.entry)p.entry=tgcx_RenameMake(root,"tgcx_rename_entry");
        if(!p.window)p.window=tgcx_RenameMake(root,"tgcx_rename_window");
        if(!p.entry || !p.window || !tgcx_RenameBind(0) || !tgcx_RenameBind(1) || !tgcx_RenameBind(2)) {
            p.fault=true;tgcx_RenameLog("Rename GUI definitions/binding unavailable; controls disabled");return;
        }
        tgcx_RenamePlacement placement={};bool measured=false;
        const char* failure="unknown";
        if(!tgcx_RenameAnchor(root,&placement,&measured,&failure)) {
            if(!p.anchor_reported){tgcx_RenameLog("Rename anchor refused: stage=%s root=%08X width=%d",failure,root,tgcx_RenameRead<int>(root+0x4A0));p.anchor_reported=true;}
            tgcx_RenameClose();tgcx_RenameVisible(p.entry,false);return;
        }
        if(!tgcx_RenamePlace(p.entry,placement.entry_x,placement.entry_y,p.promote || !tgcx_RenameRead<unsigned char>(p.entry+0x67)))return;
        tgcx_RenameVisible(p.entry,true);
        if(!p.placed) {tgcx_RenameLog("Universal Rename ABC icon at %d,%d; editor at %d,%d; native province_name anchor, %s",placement.entry_x,placement.entry_y,placement.panel_x,placement.panel_y,measured?"measured font width":"header right-edge fallback");p.placed=true;}
        if(tgcx_RenamePlace(p.window,placement.panel_x,placement.panel_y,p.open && p.promote))tgcx_RenameVisible(p.window,p.open);
        p.promote=false;
    } __except(EXCEPTION_EXECUTE_HANDLER) {
        p.fault=true;tgcx_RenameLog("Rename timer disabled after expired native view");
    }
}
static void __cdecl tgcx_RenameObserve(uintptr_t view) {
    DWORD error=GetLastError();auto& p=tgcx_rename_panel;
    p.promote=true;
    if(!p.view) {
        p.view=view;p.thread=GetCurrentThreadId();p.timer=SetTimer(nullptr,0,100,tgcx_RenameTimer);
        if(!p.timer){p.fault=true;tgcx_RenameLog("Rename UI timer creation failed");}
        else tgcx_RenameLog("Province view observed; UI timer on thread %lu",p.thread);
    }
    SetLastError(error);
}
static void __cdecl tgcx_RenameDestroy(uintptr_t view) {
    DWORD error=GetLastError();auto& p=tgcx_rename_panel;
    if(p.view==view) {
        if(p.timer)KillTimer(nullptr,p.timer);
        __try {tgcx_RenameRelease(p.entry);tgcx_RenameRelease(p.window);}
        __except(EXCEPTION_EXECUTE_HANDLER) {tgcx_RenameLog("Expired rename GUI discarded on native view destruction");}
        p={};
    }
    SetLastError(error);
}
__declspec(naked) static void tgcx_RenameUpdateHook() {
    __asm {pushfd} __asm {pushad} __asm {mov eax,[esp+40]} __asm {mov ebx,esp}
    __asm {sub esp,528} __asm {and esp,-16} __asm {fxsave [esp]} __asm {cld}
    __asm {push eax} __asm {call tgcx_RenameObserve} __asm {add esp,4}
    __asm {fxrstor [esp]} __asm {mov esp,ebx} __asm {popad} __asm {popfd}
    __asm {jmp tgcx_rename_update_original}
}
__declspec(naked) static void tgcx_RenameDestroyHook() {
    __asm {pushfd} __asm {pushad} __asm {mov ebx,esp}
    __asm {sub esp,528} __asm {and esp,-16} __asm {fxsave [esp]} __asm {cld}
    __asm {push ecx} __asm {call tgcx_RenameDestroy} __asm {add esp,4}
    __asm {fxrstor [esp]} __asm {mov esp,ebx} __asm {popad} __asm {popfd}
    __asm {jmp tgcx_rename_destroy_original}
}

// from V2DLL: V2ModAPI.h, API v1 layout (cdad55c7, 2026-10-20).
struct V2ModAPI {
    uint32_t size,apiVersion,gameBase;const char* modFolder;
    void(__cdecl* Log)(const char*,...);
    int(__cdecl* PatchSlot)(uint32_t,int,void*,void**);
    int(__cdecl* StealToTrampoline)(uint32_t,unsigned,unsigned char*,int,const unsigned char*,void*,void**,const char*);
};

static bool tgcx_RenameHash(const wchar_t* path) {
    HANDLE file=CreateFileW(path,GENERIC_READ,FILE_SHARE_READ,nullptr,OPEN_EXISTING,0,nullptr);
    if(file==INVALID_HANDLE_VALUE)return false;tgcx_SHA256 hash;unsigned char data[8192];DWORD read=0;bool ok=true;
    for(;;) {if(!ReadFile(file,data,sizeof(data),&read,nullptr)){ok=false;break;}if(!read)break;hash.update(data,read);}
    CloseHandle(file);char hex[65]={};if(ok)hash.finish(hex);return ok && !strcmp(hex,tgcx_rename_sha);
}
static bool tgcx_RenameGuards() {
    for(const auto& s:tgcx_rename_signatures) {
        unsigned char bytes[32];memcpy(bytes,s.bytes,s.size);
        for(unsigned n=0;n<s.count;++n) {
            auto* value=reinterpret_cast<uint32_t*>(bytes+s.reloc[n]);
            *value+=static_cast<uint32_t>(tgcx_rename_base)-0x400000;
        }
        if(memcmp(reinterpret_cast<void*>(tgcx_rename_base+s.rva),bytes,s.size)) {
            tgcx_RenameLog("BAD RVA %06X %s; no rename hooks installed",s.rva,s.name);return false;
        }
    }
    tgcx_RenameLog("%u native signatures OK",static_cast<unsigned>(_countof(tgcx_rename_signatures)));return true;
}

#ifndef TGCX_RENAME_TEST
static LONG tgcx_rename_initialized=0;
#pragma comment(linker,"/export:V2Mod_Init=_V2Mod_Init")
extern "C" __declspec(dllexport) int __cdecl V2Mod_Init(const V2ModAPI* api) {
    if(!api || api->size<sizeof(V2ModAPI) || api->apiVersion!=1 || !api->Log ||
       !api->PatchSlot || !api->StealToTrampoline || api->gameBase!=reinterpret_cast<uintptr_t>(GetModuleHandleW(nullptr)))return 0;
    LONG previous=InterlockedCompareExchange(&tgcx_rename_initialized,1,0);
    if(previous)return previous==2 || previous==3?1:0;
    wchar_t exe[MAX_PATH],game[MAX_PATH],path[MAX_PATH];
    if(!GetModuleFileNameW(nullptr,exe,MAX_PATH))return 0;
    wcscpy_s(game,exe);wchar_t* slash=wcsrchr(game,L'\\');if(!slash)return 0;*slash=0;
    tgcx_ModList mods={};tgcx_ReadMods(game,GetCommandLineA(),&mods);
    if(!tgcx_Ticked(mods,"rename")){api->Log("TGCX Province Rename: package not selected; inactive");InterlockedExchange(&tgcx_rename_initialized,3);return 1;}
    swprintf_s(path,L"%s\\Logs",game);CreateDirectoryW(path,nullptr);
    swprintf_s(path,L"%s\\Logs\\tgcx_province_rename.log",game);
    tgcx_rename_log=CreateFileW(path,GENERIC_WRITE,FILE_SHARE_READ,nullptr,CREATE_ALWAYS,0,nullptr);
    tgcx_RenameLog("TGCX Province Rename 0.2.8 Universal loaded_by=V2DLL plugin API v1; engine command 0x707");
    if(!tgcx_RenameHash(exe)){tgcx_RenameLog("BAD sha256; unsupported executable, no game memory read or patched");return 0;}
    tgcx_rename_base=api->gameBase;if(!tgcx_RenameGuards())return 0;
    auto* trampoline=static_cast<unsigned char*>(VirtualAlloc(nullptr,64,MEM_COMMIT|MEM_RESERVE,PAGE_EXECUTE_READWRITE));
    if(!trampoline)return 0;
    void* old=nullptr;
    if(!api->PatchSlot(0xA13768,0,reinterpret_cast<void*>(tgcx_RenameDestroyHook),&old))return 0;
    tgcx_rename_destroy_original=reinterpret_cast<uintptr_t>(old);
    const unsigned char expect[]={0x55,0x8B,0xEC,0x83,0xE4,0xF8};void* original=nullptr;
    if(!api->StealToTrampoline(0x340050,6,trampoline,32,expect,reinterpret_cast<void*>(tgcx_RenameUpdateHook),&original,"TGCX Province Rename")) {
        void* discarded=nullptr;api->PatchSlot(0xA13768,0,old,&discarded);VirtualFree(trampoline,0,MEM_RELEASE);return 0;
    }
    tgcx_rename_update_original=reinterpret_cast<uintptr_t>(original);
    original=nullptr;
    if(!api->StealToTrampoline(0x5B6400,6,trampoline+32,32,expect,reinterpret_cast<void*>(tgcx_RenameEditRefreshHook),&original,"TGCX Rename centered input")) {
        // Existing rename hooks remain safe; no edit entry is recorded yet.
        // Keep the native entry usable, and log that this optional GUI patch failed.
        tgcx_RenameLog("Centered-input hook refused; native input remains unchanged");
    } else tgcx_rename_edit_original=reinterpret_cast<uintptr_t>(original);
    InterlockedExchange(&tgcx_rename_initialized,2);
    tgcx_RenameLog("Rename hooks installed; GUI created after province selection on its UI thread");
    api->Log("TGCX Province Rename 0.2.8 Universal: initialized; native command path enabled");return 1;
}
BOOL WINAPI DllMain(HINSTANCE,DWORD,LPVOID) {return TRUE;}
#endif
