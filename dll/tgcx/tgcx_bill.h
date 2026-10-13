// LAW-2 native Bill prototype: GUI writes only; recount uses an engine decision.
#pragma once
#include "tgcx_bill_catalog.h"

// from V2DLL: V2TechButton.cpp, GStr memory layout (borrowed inputs only).
struct tgcx_GameString {
    char data[16]; uint32_t length, capacity;
};
// from V2DLL: V2TechButton.cpp, GStr/MakeStr borrowed-string ABI.
static tgcx_GameString tgcx_BorrowString(const char* text) {
    tgcx_GameString s={};s.length=static_cast<uint32_t>(strlen(text));s.capacity=15;
    if(s.length<16)memcpy(s.data,text,s.length+1);
    else { *reinterpret_cast<const char**>(s.data)=text;s.capacity=s.length; }
    return s;
}
typedef uintptr_t(__fastcall* tgcx_GuiOne)(uintptr_t,void*,const void*);
typedef void(__fastcall* tgcx_GuiZero)(uintptr_t,void*);
typedef int(*tgcx_BillVarReader)(uintptr_t,const char*,bool*);

// from V2DLL: V2TechButton.cpp, MintReadCountryVar. Missing variable is zero.
static int tgcx_ReadBillVar(uintptr_t country,const char* key,bool* ok) {
    __try {
        uintptr_t vars=country+0x1DC;
        auto* table=*reinterpret_cast<uintptr_t**>(vars);
        if(reinterpret_cast<uintptr_t>(table)!=tgcx_base+0x9FBB60 || table[1]!=tgcx_base+0x241BA0) {
            *ok=false;return 0;
        }
        tgcx_GameString name=tgcx_BorrowString(key);
        uintptr_t node=reinterpret_cast<tgcx_GuiOne>(table[1])(vars,nullptr,&name);
        return node?*reinterpret_cast<int*>(node+0x1C):0;
    } __except(EXCEPTION_EXECUTE_HANDLER) { *ok=false;return 0; }
}
static tgcx_BillVarReader tgcx_bill_reader=tgcx_ReadBillVar;

struct tgcx_BillBloc { int seats,base,yes,no,abstain,bought,stance; bool promised; };
struct tgcx_BillModel {
    uintptr_t country;
    int id,yes,no,abstain,margin,quorum,favours,rate,spent;
    bool active,audience,valid;
    tgcx_BillBloc bloc[tgcx_ideology_count];
};
static int tgcx_BillValue(uintptr_t country,const char* key,bool* ok) {
    int raw=tgcx_bill_reader(country,key,ok);
    if(raw%1000) { *ok=false;return 0; }
    return raw/1000;
}
static bool tgcx_BillReadModel(uintptr_t country,tgcx_BillModel* out) {
    *out={};out->country=country;bool ok=country!=0;
    int house=tgcx_BillValue(country,"tgcx_house_total",&ok);
    int schema=tgcx_BillValue(country,"tgcx_gui_schema",&ok);
    out->active=schema==1?tgcx_BillValue(country,"tgcx_gui_active",&ok)==1:schema==0 && house==100;
    if(!ok || !out->active)return false;
    out->id=tgcx_BillValue(country,"tgcx_gui_bill_id",&ok);
    out->yes=tgcx_BillValue(country,"tgcx_yes",&ok);
    out->no=tgcx_BillValue(country,"tgcx_no",&ok);
    out->margin=tgcx_BillValue(country,"tgcx_margin",&ok);
    out->quorum=tgcx_BillValue(country,"tgcx_quorum",&ok);
    out->favours=tgcx_BillValue(country,"tgcx_favours",&ok);
    out->rate=tgcx_BillValue(country,"tgcx_rate",&ok);
    out->spent=tgcx_BillValue(country,"tgcx_spent",&ok);
    out->audience=tgcx_BillValue(country,"tgcx_gui_crown_audience",&ok)==1;
    int sum=0,aye=0,nay=0;
    for(unsigned i=0;i<tgcx_ideology_count;++i) {
        auto& b=out->bloc[i];char key[96];
#define TGCX_BILL_READ(field,prefix) sprintf_s(key,"tgcx_" prefix "_%s",tgcx_ideology_candidates[i]);b.field=tgcx_BillValue(country,key,&ok)
        TGCX_BILL_READ(seats,"seats");TGCX_BILL_READ(base,"base_yes");TGCX_BILL_READ(no,"left");
        TGCX_BILL_READ(stance,"gui_stance");
#undef TGCX_BILL_READ
        sprintf_s(key,"tgcx_buy_%s_total",tgcx_ideology_candidates[i]);b.bought=tgcx_BillValue(country,key,&ok);
        sprintf_s(key,"tgcx_buy_%s_yes",tgcx_ideology_candidates[i]);int bought_yes=tgcx_BillValue(country,key,&ok);
        sprintf_s(key,"tgcx_gui_promised_%s",tgcx_ideology_candidates[i]);b.promised=tgcx_BillValue(country,key,&ok)==1;
        if(b.seats<0 || b.seats>100 || b.seats%5 || b.base<0 || b.base>b.seats || b.base%5 ||
           b.no<0 || b.no>b.seats || b.no%5 || b.bought<0 || b.bought>100 || bought_yes<0 ||
           bought_yes>b.bought || b.stance<-2 || b.stance>2)ok=false;
        int opposed=(std::max)(0,b.seats-b.base);
        b.abstain=b.promised?0:(std::min)(b.bought,opposed)-(std::min)(bought_yes,opposed);
        b.yes=b.seats-b.no-b.abstain;
        if(b.yes<0 || b.yes>b.seats || b.abstain<0 || b.abstain>b.seats)ok=false;
        sum+=b.seats;aye+=b.yes;nay+=b.no;
    }
    out->abstain=100-out->yes-out->no;
    out->valid=ok && house==100 && sum==100 && aye==out->yes && nay==out->no &&
        out->yes>=0 && out->no>=0 && out->abstain>=0 && out->margin==out->yes-out->no &&
        out->quorum==out->yes+out->no && out->id>=0 && out->id<=static_cast<int>(_countof(tgcx_bill_catalog)) &&
        out->favours>=0 && out->favours<=100 && out->rate>=0 && out->rate<=8 && out->spent>=0;
    return out->valid;
}

static bool tgcx_bill_enabled=false,tgcx_bill_open=false,tgcx_bill_was_active=false;
static uintptr_t tgcx_bill_root=0,tgcx_bill_window=0;
static DWORD tgcx_bill_tick=0,tgcx_bill_thread=0;
static tgcx_BillModel tgcx_bill_model={};
static unsigned char tgcx_bill_glue[3][44]={};
static bool tgcx_bill_bound[3]={};
static uintptr_t tgcx_bill_decision_fn=0;
static unsigned int tgcx_bill_draws=0;

static uintptr_t tgcx_BillFind(uintptr_t host,unsigned slot,uintptr_t expected,const char* key) {
    auto* table=*reinterpret_cast<uintptr_t**>(host);
    if(table[slot]!=tgcx_base+expected)return 0;
    tgcx_GameString name=tgcx_BorrowString(key);
    return reinterpret_cast<tgcx_GuiOne>(table[slot])(host,nullptr,&name);
}
static void tgcx_BillVisibility(bool show) {
    if(!tgcx_bill_window)return;
    auto* table=*reinterpret_cast<uintptr_t**>(tgcx_bill_window);
    unsigned slot=show?8:9;
    if(table[slot]!=tgcx_base+(show?0x5E4450:0x5E4030))return;
    reinterpret_cast<tgcx_GuiZero>(table[slot])(tgcx_bill_window,nullptr);
}
static void tgcx_BillReset(bool destroyed) {
    __try { tgcx_BillVisibility(false); } __except(EXCEPTION_EXECUTE_HANDLER) {}
    tgcx_bill_open=false;tgcx_bill_was_active=false;tgcx_bill_model.valid=false;
    if(destroyed) { tgcx_bill_root=0;tgcx_bill_window=0;memset(tgcx_bill_bound,0,sizeof(tgcx_bill_bound)); }
}
// Native observer callbacks run in the game's GUI thread, preserving CPU state.
static void tgcx_BillClickImpl(int action) {
    if(!tgcx_bill_enabled || GetCurrentThreadId()!=tgcx_bill_thread || !tgcx_politics)return;
    __try {
        if(action==1) { tgcx_bill_open=false;tgcx_BillVisibility(false);return; }
        tgcx_Snapshot sample={};bool fault=false;tgcx_BillModel fresh={};
        if(!tgcx_ReadSnapshot(&sample,&fault) || !tgcx_BillReadModel(sample.country,&fresh))return;
        if(action==0) { tgcx_bill_open=true;tgcx_BillVisibility(true);return; }
        if(action!=2 || !tgcx_bill_decision_fn)return;
        // from V2DLL: V2TechButton.cpp, FireDecision fake element and native prefix.
        char text[]="POLITICSVIEW_DECISIONtgcx_count_votes";
        unsigned char element[0x30]={};tgcx_GameString name=tgcx_BorrowString(text);
        memcpy(element+0x14,&name,sizeof(name));
        reinterpret_cast<void(__stdcall*)(void*)>(tgcx_bill_decision_fn)(element);
        tgcx_bill_tick=0;
        tgcx_Log("Bill Count requested through native decision handler tag=%.4s",sample.country_tag);
    } __except(EXCEPTION_EXECUTE_HANDLER) { tgcx_Log("Bill click rejected: native GUI/command read fault"); }
}
static void __cdecl tgcx_BillClick(int action) { tgcx_CpuScope cpu;tgcx_BillClickImpl(action); }
// from V2DLL: V2TechButton.cpp, button observer thunk; CPU preservation extended.
#define TGCX_BILL_THUNK(name,action) \
__declspec(naked) static void name() { \
    __asm { pushfd } __asm { pushad } __asm { mov ebx,esp } \
    __asm { sub esp,528 } __asm { and esp,-16 } __asm { fxsave [esp] } \
    __asm { cld } __asm { push action } __asm { call tgcx_BillClick } __asm { add esp,4 } \
    __asm { fxrstor [esp] } __asm { mov esp,ebx } __asm { popad } __asm { popfd } __asm { ret } \
}
TGCX_BILL_THUNK(tgcx_BillOpenThunk,0)
TGCX_BILL_THUNK(tgcx_BillCloseThunk,1)
TGCX_BILL_THUNK(tgcx_BillCountThunk,2)
#undef TGCX_BILL_THUNK
// from V2DLL: V2TechButton.cpp, named-button/observable binding pattern.
// The Politics template/layout is independently verified for this prototype.
static void tgcx_BillBind(uintptr_t host,const char* name,unsigned action,uintptr_t callback) {
    if(tgcx_bill_bound[action])return;
    uintptr_t button=tgcx_BillFind(host,13,0x5E4C80,name);
    if(!button)return;
    uintptr_t observable=button+0x54;
    auto* vt=*reinterpret_cast<uintptr_t**>(observable);
    if(vt[1]<tgcx_base || vt[1]>=tgcx_base+tgcx_image_size)return;
    unsigned char* glue=tgcx_bill_glue[action];memset(glue,0,44);
    // Politics' 44-byte observer uses +20 for deletion notification; the
    // InGameIdler template reads +38 and is not interchangeable (P31).
    *reinterpret_cast<uintptr_t*>(glue)=tgcx_base+0xA0E48C;
    *reinterpret_cast<uintptr_t*>(glue+4)=tgcx_politics;
    *reinterpret_cast<uintptr_t*>(glue+8)=callback;
    reinterpret_cast<tgcx_GuiOne>(vt[1])(observable,nullptr,glue);
    tgcx_bill_bound[action]=true;
    tgcx_Log("Bill native button bound name=%s",name);
}
// from V2DLL: V2TechButton.cpp, BudgetUpdate string assign/ESI text setter ABI.
static void tgcx_BillText(const char* name,const char* text) {
    uintptr_t box=tgcx_BillFind(tgcx_bill_window,15,0x5E4D00,name);
    if(!box)return;
    tgcx_GameString str=tgcx_BorrowString(text);
    typedef void(__fastcall* Assign)(uintptr_t,void*,const void*,unsigned,unsigned);
    reinterpret_cast<Assign>(tgcx_base+0x8ED0)(box+0xDC,nullptr,&str,0,0xFFFFFFFFu);
    uintptr_t inner=*reinterpret_cast<uintptr_t*>(box+0xA0);
    if(!inner)return;
    uintptr_t fn=tgcx_base+0x65ECC0;const void* arg=&str;
    __asm { push esi }
    __asm { push 0 }
    __asm { push arg }
    __asm { mov esi,inner }
    __asm { call fn }
    __asm { pop esi }
}
static void tgcx_BillLabels(const tgcx_BillModel& model) {
    char text[256],key[64];
    if(model.id>0) {
        const auto& law=tgcx_bill_catalog[model.id-1];
        tgcx_BillText("tgcx_bill_law",law.title);
        sprintf_s(text,"%s -> %s",law.from,law.to);tgcx_BillText("tgcx_bill_transition",text);
    } else {
        tgcx_BillText("tgcx_bill_law","Active bill");
        tgcx_BillText("tgcx_bill_transition","Count once to identify the saved bill.");
    }
    sprintf_s(text,"%d Aye   %d Abstain   %d Nay",model.yes,model.abstain,model.no);tgcx_BillText("tgcx_bill_tallies",text);
    sprintf_s(text,"%d / 100",model.favours);tgcx_BillText("tgcx_bill_favours",text);
    sprintf_s(text,"+%d per month",model.rate);tgcx_BillText("tgcx_bill_rate",text);
    sprintf_s(text,"Spent on this bill: %d",model.spent);tgcx_BillText("tgcx_bill_spent",text);
    unsigned row=0;
    for(unsigned i=0;i<tgcx_ideology_count;++i) {
        const auto& b=model.bloc[i];if(!b.seats)continue;
        sprintf_s(key,"tgcx_bill_bloc_%u",row);sprintf_s(text,"%s (%d seats)",tgcx_bill_bloc_names[i],b.seats);tgcx_BillText(key,text);
        sprintf_s(key,"tgcx_bill_affinity_%u",row);
        if(!model.id)strcpy_s(text,"Count to refresh");
        else sprintf_s(text,"%s %s%d",b.promised?"Promised":b.stance>0?"For":b.stance<0?"Against":"Undecided",b.stance>0?"+":"",b.stance);
        tgcx_BillText(key,text);
        sprintf_s(key,"tgcx_bill_votes_%u",row);sprintf_s(text,"%d Aye / %d Abstain / %d Nay",b.yes,b.abstain,b.no);tgcx_BillText(key,text);
        sprintf_s(key,"tgcx_bill_native_%u",row);
        const char* ideal=model.id>0?tgcx_bill_catalog[model.id-1].ideal[i]:"Count to identify affinity";
        sprintf_s(text,"TGC: %d Aye, %d Nay | Bought: %d | Ideal: %s",b.base,b.seats-b.base,b.bought,ideal);tgcx_BillText(key,text);
        ++row;
    }
    for(;row<9;++row)for(const char* field:{"bloc","affinity","votes","native"}) {
        sprintf_s(key,"tgcx_bill_%s_%u",field,row);tgcx_BillText(key,"");
    }
    tgcx_BillText("tgcx_bill_crown",model.audience?"Audience granted for this bill.":"Audience not requested. Crown rules still apply.");
    if(model.quorum<50)sprintf_s(text,"No quorum: only %d seats vote.",model.quorum);
    else if(model.margin>0)sprintf_s(text,"Would pass by %d seats.",model.margin);
    else if(model.margin<0)sprintf_s(text,"Nay leads by %d seats.",-model.margin);
    else strcpy_s(text,"Aye and Nay are tied.");
    tgcx_BillText("tgcx_bill_summary",text);
}
static void tgcx_BillTick() {
    if(!tgcx_bill_enabled || !tgcx_politics)return;
    DWORD now=GetTickCount();if(tgcx_bill_tick && now-tgcx_bill_tick<250)return;tgcx_bill_tick=now;
    __try {
        tgcx_GuiRect gui={};if(!tgcx_ReadPoliticsRect(&gui))return;
        uintptr_t root=*reinterpret_cast<uintptr_t*>(tgcx_politics+0x4C);
        if(root!=tgcx_bill_root) {
            tgcx_bill_root=root;tgcx_bill_window=0;memset(tgcx_bill_bound,0,sizeof(tgcx_bill_bound));
            tgcx_bill_thread=GetCurrentThreadId();tgcx_bill_was_active=false;
        }
        if(!tgcx_bill_window) {
            // A DLL with a script-only submod should not ask the native
            // window lookup for a missing child (that prints a game warning).
            if(!tgcx_BillFind(root,13,0x5E4C80,"tgcx_bill_open"))return;
            tgcx_bill_window=tgcx_BillFind(root,27,0x5E4A20,"tgcx_bill_window");
            if(!tgcx_bill_window)return;
            typedef void(__fastcall* Position)(uintptr_t,void*,uint32_t);
            uint32_t position=static_cast<uint16_t>(-416)|(static_cast<uint32_t>(static_cast<uint16_t>(-340))<<16);
            reinterpret_cast<Position>(tgcx_base+0x5E3BA0)(tgcx_bill_window,nullptr,position);
            tgcx_BillVisibility(false);tgcx_Log("Bill child window found root=%08X child=%08X",root,tgcx_bill_window);
        }
        tgcx_BillBind(root,"tgcx_bill_open",0,reinterpret_cast<uintptr_t>(&tgcx_BillOpenThunk));
        tgcx_BillBind(tgcx_bill_window,"tgcx_bill_close",1,reinterpret_cast<uintptr_t>(&tgcx_BillCloseThunk));
        tgcx_BillBind(tgcx_bill_window,"tgcx_bill_count",2,reinterpret_cast<uintptr_t>(&tgcx_BillCountThunk));
        tgcx_Snapshot sample={};bool fault=false;tgcx_BillModel model={};
        bool valid=tgcx_ReadSnapshot(&sample,&fault) && tgcx_BillReadModel(sample.country,&model);
        if(!valid) { tgcx_bill_model.valid=false;tgcx_bill_open=false;tgcx_bill_was_active=false;tgcx_BillVisibility(false);return; }
        bool changed=!tgcx_bill_model.valid || memcmp(&tgcx_bill_model,&model,sizeof(model))!=0;
        tgcx_bill_model=model;
        if(!tgcx_bill_was_active)tgcx_bill_open=true;tgcx_bill_was_active=true;
        if(changed)tgcx_BillLabels(model);
        tgcx_BillVisibility(tgcx_bill_open && gui.width>=900 && gui.height>=700);
    } __except(EXCEPTION_EXECUTE_HANDLER) {
        __try { tgcx_BillVisibility(false); } __except(EXCEPTION_EXECUTE_HANDLER) {}
        tgcx_bill_model.valid=false;tgcx_bill_enabled=false;tgcx_Log("Bill prototype disabled after native GUI read fault; Decisions remain available");
    }
}

static HRESULT tgcx_DrawBillChart(IDirect3DDevice9* device,const tgcx_BillModel& model,const tgcx_Snapshot& sample,const tgcx_GuiRect& gui) {
    if(!device || !model.valid || gui.width<900 || gui.height<700)return S_FALSE;
    unsigned seats[tgcx_ideology_count]={};for(unsigned i=0;i<tgcx_ideology_count;++i)seats[i]=static_cast<unsigned>(model.bloc[i].seats);
    tgcx_Seat dots[100];tgcx_SeatGeometry(seats,dots);
    IDirect3DSurface9* target=nullptr;IDirect3DSurface9* back=nullptr;
    HRESULT result=device->GetRenderTarget(0,&target);
    if(SUCCEEDED(result))result=device->GetBackBuffer(0,0,D3DBACKBUFFER_TYPE_MONO,&back);
    bool valid=target && back && target==back;D3DSURFACE_DESC desc={};
    if(valid)result=back->GetDesc(&desc);if(target)target->Release();if(back)back->Release();
    if(FAILED(result))return result;if(!valid || desc.Width!=static_cast<unsigned>(gui.width) || desc.Height!=static_cast<unsigned>(gui.height))return S_FALSE;
    IDirect3DStateBlock9* state=nullptr;result=device->CreateStateBlock(D3DSBT_ALL,&state);if(FAILED(result))return result;
    result=state->Capture();if(FAILED(result)){state->Release();return result;}
    int left=gui.width/2-416,top=gui.height/2-340;
    RECT clip={left+22,top+173,left+320,top+311};D3DVIEWPORT9 viewport={0,0,static_cast<DWORD>(gui.width),static_cast<DWORD>(gui.height),0,1};
#define TGCX_BILL_SET(call) if(SUCCEEDED(result))result=(call)
    TGCX_BILL_SET(device->SetViewport(&viewport));TGCX_BILL_SET(device->SetScissorRect(&clip));TGCX_BILL_SET(device->SetRenderState(D3DRS_SCISSORTESTENABLE,TRUE));
    TGCX_BILL_SET(device->SetVertexShader(nullptr));TGCX_BILL_SET(device->SetPixelShader(nullptr));TGCX_BILL_SET(device->SetFVF(tgcx_fvf));TGCX_BILL_SET(device->SetTexture(0,nullptr));
    TGCX_BILL_SET(device->SetRenderState(D3DRS_ZENABLE,FALSE));TGCX_BILL_SET(device->SetRenderState(D3DRS_ZWRITEENABLE,FALSE));TGCX_BILL_SET(device->SetRenderState(D3DRS_CULLMODE,D3DCULL_NONE));
    TGCX_BILL_SET(device->SetRenderState(D3DRS_LIGHTING,FALSE));TGCX_BILL_SET(device->SetRenderState(D3DRS_FOGENABLE,FALSE));TGCX_BILL_SET(device->SetRenderState(D3DRS_STENCILENABLE,FALSE));
    TGCX_BILL_SET(device->SetRenderState(D3DRS_ALPHATESTENABLE,FALSE));TGCX_BILL_SET(device->SetRenderState(D3DRS_ALPHABLENDENABLE,TRUE));TGCX_BILL_SET(device->SetRenderState(D3DRS_SRCBLEND,D3DBLEND_SRCALPHA));TGCX_BILL_SET(device->SetRenderState(D3DRS_DESTBLEND,D3DBLEND_INVSRCALPHA));
    TGCX_BILL_SET(device->SetRenderState(D3DRS_BLENDOP,D3DBLENDOP_ADD));TGCX_BILL_SET(device->SetRenderState(D3DRS_SEPARATEALPHABLENDENABLE,FALSE));TGCX_BILL_SET(device->SetRenderState(D3DRS_SRGBWRITEENABLE,FALSE));TGCX_BILL_SET(device->SetRenderState(D3DRS_COLORWRITEENABLE,15));TGCX_BILL_SET(device->SetRenderState(D3DRS_FILLMODE,D3DFILL_SOLID));
    TGCX_BILL_SET(device->SetTextureStageState(0,D3DTSS_COLOROP,D3DTOP_SELECTARG1));TGCX_BILL_SET(device->SetTextureStageState(0,D3DTSS_COLORARG1,D3DTA_DIFFUSE));TGCX_BILL_SET(device->SetTextureStageState(0,D3DTSS_ALPHAOP,D3DTOP_SELECTARG1));TGCX_BILL_SET(device->SetTextureStageState(0,D3DTSS_ALPHAARG1,D3DTA_DIFFUSE));
    TGCX_BILL_SET(device->SetTextureStageState(0,D3DTSS_TEXTURETRANSFORMFLAGS,D3DTTFF_DISABLE));TGCX_BILL_SET(device->SetTextureStageState(0,D3DTSS_TEXCOORDINDEX,0));TGCX_BILL_SET(device->SetTextureStageState(0,D3DTSS_RESULTARG,D3DTA_CURRENT));TGCX_BILL_SET(device->SetTextureStageState(1,D3DTSS_COLOROP,D3DTOP_DISABLE));
    TGCX_BILL_SET(tgcx_SoftPanel(device,static_cast<float>(left+23),static_cast<float>(top+174),296,136,0,0xFFCCC9BE));
    float cosine[tgcx_dot_segments+1],sine[tgcx_dot_segments+1];for(unsigned i=0;i<=tgcx_dot_segments;++i){float a=6.28318530718f*static_cast<float>(i%tgcx_dot_segments)/tgcx_dot_segments;cosine[i]=cosf(a);sine[i]=sinf(a);}
    unsigned used[tgcx_ideology_count]={};
    // Geometry's angular sweep is separate from storage order. Each bloc still
    // has exactly its cached Aye/abstain/Nay seats; no new vote is calculated.
    for(unsigned i=0;i<100;++i) {
        unsigned b=dots[i].ideology,n=used[b]++;const auto& bloc=model.bloc[b];
        DWORD color=n<static_cast<unsigned>(bloc.yes)?0xFF83C53B:n<static_cast<unsigned>(bloc.yes+bloc.abstain)?0xFF908D80:0xFFC52E28;
        float x=left+171+(dots[i].x-99)*2.0f,y=top+299+(dots[i].y-183)*2.0f;
        auto* v=tgcx_seat_vertices+i*2*tgcx_disc_vertices;
        tgcx_AppendDisc(v,cosine,sine,x,y,5.0f,0.7f,sample.colors[b]);tgcx_AppendDisc(v+tgcx_disc_vertices,cosine,sine,x,y,3.0f,0.5f,color);
    }
    TGCX_BILL_SET(device->DrawPrimitiveUP(D3DPT_TRIANGLELIST,100*2*tgcx_disc_vertices/3,tgcx_seat_vertices,sizeof(tgcx_Vertex)));
#undef TGCX_BILL_SET
    HRESULT restore=state->Apply();state->Release();return FAILED(restore)?restore:result;
}
static void tgcx_BillAfterEntity(uintptr_t renderer,uintptr_t entity) {
    if(!tgcx_bill_enabled || !tgcx_bill_open || !tgcx_bill_model.valid || !tgcx_bill_window)return;
    __try {
        if(!*reinterpret_cast<unsigned char*>(tgcx_bill_window+0x67))return;
        uintptr_t marker=tgcx_GuiEntity(reinterpret_cast<char*>(tgcx_bill_window),renderer,"tgcx_bill_chart_overlay",0xA3FF08);
        if(marker!=entity)return;
        tgcx_GuiRect gui={};tgcx_Snapshot sample={};bool fault=false;
        if(!tgcx_ReadPoliticsRect(&gui) || !tgcx_ReadSnapshot(&sample,&fault) || sample.country!=tgcx_bill_model.country)return;
        auto* device=*reinterpret_cast<IDirect3DDevice9**>(renderer+0x64);
        if(device && tgcx_DrawBillChart(device,tgcx_bill_model,sample,gui)==S_OK && ++tgcx_bill_draws==1)
            tgcx_Log("Bill first ordered projected-seat draw Aye=%d abstain=%d Nay=%d",tgcx_bill_model.yes,tgcx_bill_model.abstain,tgcx_bill_model.no);
    } __except(EXCEPTION_EXECUTE_HANDLER) { tgcx_bill_model.valid=false;tgcx_Log("Bill chart rejected: native marker read fault"); }
}
