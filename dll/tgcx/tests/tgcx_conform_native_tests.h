// Isolated TPL-1 native-layer tests on synthetic engine-shaped memory: the
// registry walk, the stack read, each command constructor's argument order
// against fakes, the id under the lock, the daily driver and one Conform with
// stub commands. No game process or state mutation.
#pragma once
#include "../tgcx_conform_native.h"

// ---- fake constructors (stdcall, as the engine's) record their arguments
static uintptr_t tgcx_cf_t_args[10];static unsigned tgcx_cf_t_argc;
static unsigned char tgcx_cf_t_memory[8][0x100];static unsigned tgcx_cf_t_allocs;static uint32_t tgcx_cf_t_size;
static void* __cdecl tgcx_CfFakeNew(size_t size) { tgcx_cf_t_size=static_cast<uint32_t>(size);return tgcx_cf_t_allocs<8?tgcx_cf_t_memory[tgcx_cf_t_allocs++]:nullptr; }
static void tgcx_CfRecord(std::initializer_list<uintptr_t> a) { tgcx_cf_t_argc=0;for(auto v:a)tgcx_cf_t_args[tgcx_cf_t_argc++]=v; }
static void* __stdcall tgcx_CfFakeMove(void* m,uintptr_t s,int p,int a,int b,int c,int d) {
    tgcx_CfRecord({reinterpret_cast<uintptr_t>(m),s,static_cast<uintptr_t>(p),static_cast<uintptr_t>(a),static_cast<uintptr_t>(b),static_cast<uintptr_t>(c),static_cast<uintptr_t>(d)});
    *reinterpret_cast<uintptr_t*>(m)=0x1001;return m;
}
static uintptr_t tgcx_cf_t_merge[4];
static void* __stdcall tgcx_CfFakeMerge(void* m,tgcx_CfList* l) {
    tgcx_cf_t_merge[0]=l->count;tgcx_cf_t_merge[1]=l->head?l->head->item:0;
    tgcx_cf_t_merge[2]=l->head && l->head->next?l->head->next->item:0;tgcx_cf_t_merge[3]=l->tail==l->head->next && !l->tail->next;
    *reinterpret_cast<uintptr_t*>(m)=0x1002;return m;
}
static void* __stdcall tgcx_CfFakeTransfer(void* m,uint32_t type,uint32_t id,uintptr_t regiment) {
    tgcx_CfRecord({reinterpret_cast<uintptr_t>(m),type,id,regiment});*reinterpret_cast<uintptr_t*>(m)=0x1003;return m;
}
static void* __stdcall tgcx_CfFakeCreate(void* m,int p,int c,uint32_t t1,uint32_t i1,uint32_t t2,uint32_t i2,uintptr_t x,int kind,int split) {
    tgcx_CfRecord({reinterpret_cast<uintptr_t>(m),static_cast<uintptr_t>(p),static_cast<uintptr_t>(c),t1,i1,t2,i2,x,
                   static_cast<uintptr_t>(kind),static_cast<uintptr_t>(split)});
    *reinterpret_cast<uintptr_t*>(m)=0x1004;return m;
}
static uintptr_t tgcx_cf_t_lock_eax;
__declspec(naked) static void tgcx_CfFakeLock() {   // EAX lock byte; takes it, returns 0 in AL to store back
    __asm { mov tgcx_cf_t_lock_eax,eax } __asm { mov byte ptr [eax],1 } __asm { mov eax,0x77777700 } __asm { ret }
}

// ---- stubs for the driver and Conform
static unsigned tgcx_cf_s_posts,tgcx_cf_s_moves,tgcx_cf_s_merges,tgcx_cf_s_transfers,tgcx_cf_s_creates;
static int tgcx_cf_s_move_province;static uintptr_t tgcx_cf_s_merge_target,tgcx_cf_s_merge_other,tgcx_cf_s_transfer_regiment;
static tgcx_CfId tgcx_cf_s_transfer_target;
static unsigned char tgcx_cf_s_command[16][16];static unsigned tgcx_cf_s_made;
static void* tgcx_CfStubCommand(uintptr_t vtable) { auto* c=tgcx_cf_s_command[tgcx_cf_s_made++%16];*reinterpret_cast<uintptr_t*>(c)=vtable;return c; }
static void* tgcx_CfStubMove(uintptr_t,int p) { ++tgcx_cf_s_moves;tgcx_cf_s_move_province=p;return tgcx_CfStubCommand(0x2001); }
static void* tgcx_CfStubMerge(uintptr_t t,uintptr_t o) { ++tgcx_cf_s_merges;tgcx_cf_s_merge_target=t;tgcx_cf_s_merge_other=o;return tgcx_CfStubCommand(0x2002); }
static void* tgcx_CfStubTransfer(const tgcx_CfId& t,uintptr_t r) { ++tgcx_cf_s_transfers;tgcx_cf_s_transfer_target=t;tgcx_cf_s_transfer_regiment=r;return tgcx_CfStubCommand(0x2003); }
static void* tgcx_CfStubCreate(uintptr_t,const tgcx_CfId&) { ++tgcx_cf_s_creates;return tgcx_CfStubCommand(0x2004); }
static bool tgcx_CfStubNewId(const tgcx_CfGlobals&,tgcx_CfId* out) { *out={0x1269,900};return true; }
static bool tgcx_CfStubLand(uintptr_t) { return true; }
static uintptr_t tgcx_cf_s_post_order[256];
static bool tgcx_CfStubPost(void* command) {
    if(tgcx_cf_s_posts<256)tgcx_cf_s_post_order[tgcx_cf_s_posts]=*reinterpret_cast<uintptr_t*>(command);
    ++tgcx_cf_s_posts;return true;
}
static int tgcx_cf_s_recruited[tgcx_tpl_units];
static uintptr_t tgcx_cf_s_recruit_target;
static unsigned tgcx_cf_s_culture;
static tgcx_TplResult tgcx_CfStubRecruit(uintptr_t target,const int* counts,tgcx_TplQueued queued,void* context,unsigned culture) {
    tgcx_cf_s_recruit_target=target;
    tgcx_cf_s_culture=culture;
    tgcx_TplResult r={true,0,0,0};
    for(unsigned u=0;u<tgcx_tpl_units;++u)for(int k=0;k<counts[u];++k) {
        uint32_t cons[2]={0x20,500+r.queued},stack[2]={0x1269,600+r.queued};
        tgcx_cf_s_recruited[u]++;queued(context,static_cast<int>(u),33,cons,stack);++r.queued;
    }
    return r;
}
static int tgcx_cf_s_unreachable=-1;
static bool tgcx_CfStubReach(uintptr_t,int province) {return province!=tgcx_cf_s_unreachable;}

// ---- synthetic registry: one bucket table per range, nodes {entry, next}
struct tgcx_CfFakeTable { uint32_t unused,count;uintptr_t buckets; };
static uintptr_t tgcx_cf_f_buckets_low[7],tgcx_cf_f_buckets_high[7];
static tgcx_CfFakeTable tgcx_cf_f_low={0,7,0},tgcx_cf_f_high={0,7,0};
static uintptr_t tgcx_cf_f_nodes[32][2];static unsigned tgcx_cf_f_node_count;
static void tgcx_CfFakeRegister(uintptr_t entry) {
    uint32_t type=*reinterpret_cast<uint32_t*>(entry+8),id=*reinterpret_cast<uint32_t*>(entry+0xC);
    uintptr_t* buckets=type>0x1268?tgcx_cf_f_buckets_high:tgcx_cf_f_buckets_low;
    uint32_t b=((type<<16)+id)%7;
    auto* node=tgcx_cf_f_nodes[tgcx_cf_f_node_count++];node[0]=entry;node[1]=buckets[b];buckets[b]=reinterpret_cast<uintptr_t>(node);
}
static void tgcx_CfFakeReset() {
    memset(tgcx_cf_f_buckets_low,0,sizeof(tgcx_cf_f_buckets_low));memset(tgcx_cf_f_buckets_high,0,sizeof(tgcx_cf_f_buckets_high));
    tgcx_cf_f_node_count=0;tgcx_cf_f_low.buckets=reinterpret_cast<uintptr_t>(tgcx_cf_f_buckets_low);
    tgcx_cf_f_high.buckets=reinterpret_cast<uintptr_t>(tgcx_cf_f_buckets_high);
}
// A stack: vtable +0, ids +0x10/+0x14, list +0x38, count +0x40, tag +0xC0, country +0xC4, province object +0xDC (+0x58 id).
static unsigned char tgcx_cf_f_stack[6][0x100];static unsigned char tgcx_cf_f_regiment[8][0x40];
static uintptr_t tgcx_cf_f_list[8][3];static unsigned char tgcx_cf_f_province[6][0x60];
template<class T> static void tgcx_CfPut(void* at,size_t off,T v) { *reinterpret_cast<T*>(static_cast<unsigned char*>(at)+off)=v; }
static uintptr_t tgcx_CfMakeStack(unsigned i,uint32_t id,int province,int country) {
    auto* s=tgcx_cf_f_stack[i];memset(s,0,0x100);
    tgcx_CfPut<uintptr_t>(s,0,0x7AB0);tgcx_CfPut<uint32_t>(s,0x10,0x1269);tgcx_CfPut<uint32_t>(s,0x14,id);
    tgcx_CfPut<uint32_t>(s,0xC0,0x415246);tgcx_CfPut<int>(s,0xC4,country);   // "FRA"
    tgcx_CfPut<int>(tgcx_cf_f_province[i],0x58,province);tgcx_CfPut<uintptr_t>(s,0xDC,reinterpret_cast<uintptr_t>(tgcx_cf_f_province[i]));
    tgcx_CfFakeRegister(reinterpret_cast<uintptr_t>(s)+8);return reinterpret_cast<uintptr_t>(s);
}
static uintptr_t tgcx_cf_f_units[tgcx_tpl_units];
static void tgcx_CfAddRegiment(uintptr_t stack,unsigned slot,unsigned r,uint32_t id,unsigned unit) {
    auto* g=tgcx_cf_f_regiment[r];memset(g,0,0x40);
    tgcx_CfPut<uint32_t>(g,8,0x30);tgcx_CfPut<uint32_t>(g,0xC,id);tgcx_CfPut<uintptr_t>(g,0x38,tgcx_cf_f_units[unit]);
    tgcx_CfFakeRegister(reinterpret_cast<uintptr_t>(g));
    auto* node=tgcx_cf_f_list[slot];node[0]=reinterpret_cast<uintptr_t>(g);node[1]=0;node[2]=0;
    uintptr_t* tail=reinterpret_cast<uintptr_t*>(stack+0x38);
    while(*tail)tail=reinterpret_cast<uintptr_t*>(*tail+8);
    *tail=reinterpret_cast<uintptr_t>(node);++*reinterpret_cast<int*>(stack+0x40);
}

static void tgcx_ConformNativeTests() {
    (void)&tgcx_InstallConform;   // live install: covered by the guard table, not run here
    static unsigned char world[0xC00]={};tgcx_CfPut<int>(world,0xB60,7);tgcx_CfPut<uint32_t>(world,0xB0C,100);
    static uint32_t counter=41;static uint16_t id_base=1;static unsigned char lock=0;
    tgcx_CfGlobals g={reinterpret_cast<uintptr_t>(world),reinterpret_cast<uintptr_t>(&tgcx_cf_f_low),reinterpret_cast<uintptr_t>(&tgcx_cf_f_high),
        reinterpret_cast<uintptr_t>(&counter),reinterpret_cast<uintptr_t>(&id_base),reinterpret_cast<uintptr_t>(&lock),0x2001,0x2002,0x2003,0x2004};
    static unsigned char unit_types[6][0x44];
    for(unsigned u=0;u<6;++u)tgcx_cf_f_units[u]=reinterpret_cast<uintptr_t>(unit_types[u]);

    // ---- registry and stack read
    tgcx_CfFakeReset();
    uintptr_t target=tgcx_CfMakeStack(0,50,10,7);
    tgcx_CfAddRegiment(target,0,0,1,3);tgcx_CfAddRegiment(target,1,1,2,3);tgcx_CfAddRegiment(target,2,2,3,5);
    uintptr_t enemy=tgcx_CfMakeStack(1,51,10,8);(void)enemy;
    tgcx_Expect(tgcx_CfLookup(g,{0x1269,50})==target+8,"conform registry finds the stack entry (+8)");
    tgcx_Expect(tgcx_CfLookup(g,{0x30,2})==reinterpret_cast<uintptr_t>(tgcx_cf_f_regiment[1]),"conform registry finds a regiment in the low table");
    tgcx_Expect(!tgcx_CfLookup(g,{0x1269,99}) && !tgcx_CfLookup(g,{0,0}),"conform registry misses cleanly");
    tgcx_Expect(tgcx_CfStackOf(g,{0x1269,50},0x7AB0)==target,"conform resolves the player's stack");
    tgcx_Expect(!tgcx_CfStackOf(g,{0x1269,51},0x7AB0) && !tgcx_CfStackOf(g,{0x1269,50},0x7AB1),"conform ignores other owners and classes");
    static tgcx_CfStack read;
    tgcx_Expect(tgcx_CfReadStack(target,tgcx_cf_f_units,&read) && read.count==3 && read.province==10 && read.id.id==50,"conform reads the stack");
    tgcx_Expect(read.regiment[0].unit==3 && read.regiment[2].unit==5 && read.regiment[2].id.id==3,"conform maps regiments to unit indices");
    tgcx_Expect(tgcx_CfReadStack(target,nullptr,&read) && read.regiment[0].unit==-1,"conform leaves unknown types unnamed");
    tgcx_Expect(tgcx_CfStackHas(target,reinterpret_cast<uintptr_t>(tgcx_cf_f_regiment[2])) &&
        !tgcx_CfStackHas(target,reinterpret_cast<uintptr_t>(tgcx_cf_f_regiment[5])),"conform membership test");

    // ---- constructor ABIs against fakes (P31)
    uintptr_t saved_new=tgcx_tpl_new_fn;tgcx_tpl_new_fn=reinterpret_cast<uintptr_t>(&tgcx_CfFakeNew);
    tgcx_cf_move_fn=reinterpret_cast<uintptr_t>(&tgcx_CfFakeMove);tgcx_cf_merge_fn=reinterpret_cast<uintptr_t>(&tgcx_CfFakeMerge);
    tgcx_cf_transfer_fn=reinterpret_cast<uintptr_t>(&tgcx_CfFakeTransfer);tgcx_cf_create_fn=reinterpret_cast<uintptr_t>(&tgcx_CfFakeCreate);
    tgcx_cf_lock_fn=reinterpret_cast<uintptr_t>(&tgcx_CfFakeLock);
    void* c=tgcx_CfMoveNative(target,42);
    tgcx_Expect(c && tgcx_cf_t_size==0x90 && tgcx_cf_t_args[1]==target && tgcx_cf_t_args[2]==42,"conform Move size, stack, province");
    tgcx_Expect(tgcx_cf_t_args[3]==1 && tgcx_cf_t_args[4]==1 && tgcx_cf_t_args[5]==1 && tgcx_cf_t_args[6]==0,"conform Move flags as the rally point");
    c=tgcx_CfMergeNative(target,0x4444);
    tgcx_Expect(c && tgcx_cf_t_size==0x4C && tgcx_cf_t_merge[0]==2 && tgcx_cf_t_merge[1]==target && tgcx_cf_t_merge[2]==0x4444 && tgcx_cf_t_merge[3],
        "conform Merge size as native callers (0x4C), list: target first, linked, count 2");
    c=tgcx_CfTransferNative({0x1269,900},0x5555);
    tgcx_Expect(c && tgcx_cf_t_size==0x6C && tgcx_cf_t_args[1]==0x1269 && tgcx_cf_t_args[2]==900 && tgcx_cf_t_args[3]==0x5555,
        "conform Transfer: target stack id, then the regiment");
    c=tgcx_CfCreateNative(target,{0x1269,900});
    tgcx_Expect(c && tgcx_cf_t_size==0x60 && tgcx_cf_t_args[1]==0x415246 && tgcx_cf_t_args[2]==7,"conform Create tag (+0xC0) and country");
    tgcx_Expect(tgcx_cf_t_args[3]==0x1269 && tgcx_cf_t_args[4]==900 && tgcx_cf_t_args[5]==0x1269 && tgcx_cf_t_args[6]==900,"conform Create id twice");
    tgcx_Expect(tgcx_cf_t_args[7]==10 && tgcx_cf_t_args[8]==0 && tgcx_cf_t_args[9]==0,"conform Create province id +0xDC->+0x58, land kind, no split");
    tgcx_CfId fresh={};
    tgcx_Expect(tgcx_CfNewIdNative(g,&fresh) && fresh.id==42 && counter==42 && fresh.type==0x1269,"conform new id: ++counter, word + 0x1268");
    tgcx_Expect(tgcx_cf_t_lock_eax==reinterpret_cast<uintptr_t>(&lock) && lock==0,"conform new id: lock in EAX, released with AL");
    tgcx_tpl_new_fn=saved_new;

    // ---- the daily driver: march, then merge
    tgcx_CfNative stub={tgcx_CfStubMove,tgcx_CfStubMerge,tgcx_CfStubTransfer,tgcx_CfStubCreate,tgcx_CfStubNewId,tgcx_CfStubLand,
        tgcx_CfStubPost,tgcx_CfStubRecruit,tgcx_CfStubReach};
    // No real orders file in tests: the store is marked read and unusable.
    tgcx_cf_orders_loaded=true;tgcx_cf_orders_ok=false;tgcx_cf_orders.count=0;
    memset(&tgcx_cf_tracker,0,sizeof(tgcx_cf_tracker));tgcx_cf_world=g.world;tgcx_cf_player=7;tgcx_cf_step=0;tgcx_cf_date=99;
    tgcx_cf_stack_vtable=0x7AB0;
    uintptr_t built=tgcx_CfMakeStack(2,60,20,7);
    tgcx_CfAdd(&tgcx_cf_tracker,TGCX_CF_JOIN,{0x1269,50},{0x1269,60},{},3,99);
    tgcx_CfDaily(g,stub);
    tgcx_Expect(tgcx_cf_s_moves==1 && tgcx_cf_s_move_province==10 && tgcx_cf_s_posts==1,"conform driver marches to the target");
    tgcx_CfDaily(g,stub);
    tgcx_Expect(tgcx_cf_s_moves==1,"conform driver steps once per game day");
    tgcx_CfPut<int>(tgcx_cf_f_province[2],0x58,10);tgcx_CfPut<uint32_t>(world,0xB0C,101);
    tgcx_CfDaily(g,stub);
    tgcx_Expect(tgcx_cf_s_merges==1 && tgcx_cf_s_merge_target==target && tgcx_cf_s_merge_other==built,"conform driver merges into the target");
    tgcx_CfPut<uint32_t>(world,0xB0C,50);tgcx_CfDaily(g,stub);
    tgcx_Expect(!tgcx_cf_tracker.task[0].used,"conform driver forgets session tasks on a load (only saved orders resume, TPL-7)");

    // ---- one Conform: 2 infantry(3) + 1 cavalry(5) to 2 infantry + 2 artillery(4)
    memset(&tgcx_cf_tracker,0,sizeof(tgcx_cf_tracker));tgcx_cf_date=50;
    tgcx_cf_s_posts=tgcx_cf_s_creates=tgcx_cf_s_transfers=0;
    // A finished one-regiment artillery stack nearby is not taken: Conform builds (user, 2026-10-17).
    uintptr_t idle=tgcx_CfMakeStack(3,70,11,7);tgcx_CfAddRegiment(idle,3,3,7,4);
    int want[tgcx_tpl_units]={};want[3]=2;want[4]=2;
    static tgcx_CfPlan plan;
    tgcx_CfResult r=tgcx_CfConformWith(g,stub,target,tgcx_cf_f_units,want,&plan,TGCX_CULTURE_PRIMARY);
    tgcx_Expect(tgcx_cf_s_culture==TGCX_CULTURE_PRIMARY,"Conform forwards the selected culture mode to actual missing-unit recruitment");
    tgcx_Expect(r.read && r.refusal==TGCX_CF_OK && r.detached==1 && !r.gathered && r.queued==2 && !r.failed,"conform plans detach and build, gathers nothing");
    tgcx_Expect(tgcx_cf_s_creates==1 && tgcx_cf_s_recruited[4]==2,"conform creates one stack and builds both artillery");
    tgcx_Expect(tgcx_cf_s_recruit_target==target,"conform recruitment receives the actual target army for its land scope");
    tgcx_Expect(tgcx_cf_s_transfers==1 && tgcx_cf_s_post_order[0]==g.vt_create && tgcx_cf_s_post_order[1]==g.vt_transfer,
        "conform queues Create then Transfer together before the new empty stack can disappear");
    tgcx_Expect(tgcx_cf_s_transfer_target.id==900 && tgcx_cf_s_transfer_regiment==reinterpret_cast<uintptr_t>(tgcx_cf_f_regiment[2]),
        "conform initial batch names the new stack and the extra regiment");
    int pending[tgcx_tpl_units];tgcx_CfPending(tgcx_cf_tracker,{0x1269,50},pending);
    tgcx_Expect(pending[4]==2,"conform tracks both built artillery");
    r=tgcx_CfConformWith(g,stub,target,tgcx_cf_f_units,want,&plan);
    tgcx_Expect(r.refusal==TGCX_CF_OK && !r.queued && !r.gathered && !r.detached && !r.failed && tgcx_cf_s_creates==1,
        "conform again counts pending regiments and leaving extras, sends nothing more");
    // No simulated execution: the still-present extra is detected, not claimed
    // as detached. The retry remains pending only while its destination exists.
    uintptr_t extras=tgcx_CfMakeStack(4,900,10,7);
    tgcx_CfPut<uint32_t>(world,0xB0C,51);
    tgcx_CfDaily(g,stub);
    tgcx_Expect(tgcx_cf_s_transfers==1 && tgcx_cf_tracker.task[0].used && tgcx_cf_tracker.task[0].retries==1,
        "conform verifies the initial batch and schedules a refused transfer for retry");
    tgcx_CfPut<uintptr_t>(reinterpret_cast<void*>(extras),0,0);tgcx_CfPut<uint32_t>(world,0xB0C,52);
    tgcx_CfDaily(g,stub);
    tgcx_Expect(!tgcx_cf_tracker.task[0].used && tgcx_cf_s_transfers==1,
        "conform verifies a vanished empty stack without reading null or retrying into it");
    tgcx_Expect(tgcx_CfConformWith(g,stub,enemy,tgcx_cf_f_units,want,&plan).read==false,"conform refuses another country's stack");
    // Already queued overseas builds must not make a later preview claim a
    // full army. Stop only our tracking; no cancel/disband/move is posted.
    unsigned posts_before=tgcx_cf_s_posts;tgcx_cf_s_unreachable=33;
    tgcx_CfPruneRoutes(g,stub,{0x1269,50},target);
    tgcx_CfPending(tgcx_cf_tracker,{0x1269,50},pending);
    tgcx_Expect(!pending[4] && tgcx_cf_s_posts==posts_before,
        "Conform removes stranded origins from pending counts without changing their units/constructions");
    tgcx_cf_s_unreachable=20;
    tgcx_CfPut<int>(tgcx_cf_f_province[2],0x58,20);
    auto* stranded=tgcx_CfAdd(&tgcx_cf_tracker,TGCX_CF_JOIN,{0x1269,50},{0x1269,60},{},3,52);
    tgcx_CfPut<uint32_t>(world,0xB0C,53);tgcx_CfDaily(g,stub);
    tgcx_Expect(stranded && !stranded->used && tgcx_cf_s_posts==posts_before,
        "Conform daily driver does not order an unreachable reinforcement to march");
    tgcx_cf_s_unreachable=-1;

    // ---- TPL-7 saved orders (D13 revised): the file format
    static tgcx_CfOrders orders;static char text[8192];orders.count=0;
    tgcx_CfOrder a={0x415246,99,0,{0x1269,50},{0x1269,600},{0x20,500},4,false};
    tgcx_CfOrderAdd(&orders,a);a.done=120;a.stack.id=601;a.construction.id=501;a.unit=3;tgcx_CfOrderAdd(&orders,a);
    size_t used=tgcx_CfOrdersWrite(orders,text,sizeof(text));
    static tgcx_CfOrders back;
    tgcx_Expect(used && tgcx_CfOrdersParse(text,&back) && back.count==2 && back.item[1].done==120 && back.item[1].stack.id==601 &&
        back.item[0].construction.type==0x20 && back.item[0].construction.id==500 && back.item[0].unit==4 && back.item[0].tag==0x415246,
        "saved orders: write and parse round trip");
    char* unit=strstr(text,"unit=4");if(unit)unit[5]='x';
    tgcx_Expect(unit && !tgcx_CfOrdersParse(text,&back) && !back.count,"saved orders: a damaged line refuses the whole file");
    tgcx_Expect(!tgcx_CfOrdersParse("tgcx_conform_orders 2\r\n",&back) && tgcx_CfOrdersParse("tgcx_conform_orders 1\r\n",&back) && !back.count,
        "saved orders: header version checked, empty file accepted");
    tgcx_Expect(tgcx_CfOrderApplies(orders.item[1],0x415246,99) && tgcx_CfOrderApplies(orders.item[1],0x415246,119) &&
        !tgcx_CfOrderApplies(orders.item[1],0x415246,120) && !tgcx_CfOrderApplies(orders.item[1],0x415246,98) &&
        !tgcx_CfOrderApplies(orders.item[0],0x474E45,110),"saved orders: only the same tag, from creation up to the end date");
    {
        static tgcx_CfOrders full;full.count=0;unsigned dropped=0;
        for(unsigned i=0;i<tgcx_cf_orders_max;++i) { tgcx_CfOrder o=a;o.done=0;o.created=1000+i;tgcx_CfOrderAdd(&full,o); }
        full.item[7].done=1100;
        tgcx_CfOrder o=a;o.done=0;o.created=5000;unsigned at=tgcx_CfOrderAdd(&full,o,&dropped);
        tgcx_Expect(full.count==tgcx_cf_orders_max && dropped==7 && at==tgcx_cf_orders_max-1 && full.item[at].created==5000,
            "saved orders: a full file drops the oldest finished order first");
        at=tgcx_CfOrderAdd(&full,o,&dropped);
        tgcx_Expect(dropped==0 && full.item[0].created==1001,"saved orders: with none finished, the oldest open order goes");
    }

    // ---- TPL-7: a load resumes the saved orders that apply
    // Order 1: still being built. Order 2: finished at a rally point (province 20).
    memset(&tgcx_cf_tracker,0,sizeof(tgcx_cf_tracker));tgcx_cf_stack_vtable=0;
    tgcx_CfPut<int>(tgcx_cf_f_province[0],0x58,10);
    tgcx_CfPut<uintptr_t>(reinterpret_cast<void*>(tgcx_cf_f_stack[0]),0,0x7AB0);
    uintptr_t rallied=tgcx_CfMakeStack(5,601,20,7);(void)rallied;
    tgcx_cf_orders=orders;tgcx_cf_orders_ok=true;tgcx_cf_orders_dirty=false;tgcx_cf_orders_path[0]=0;   // no file: Save is a no-op
    tgcx_cf_date=200;tgcx_CfPut<uint32_t>(world,0xB0C,110);
    unsigned moves_before=tgcx_cf_s_moves;
    tgcx_CfDaily(g,stub);
    tgcx_Expect(tgcx_cf_tracker.task[0].used && tgcx_cf_tracker.task[0].record==1 && tgcx_cf_tracker.task[0].construction.id==500 &&
        tgcx_cf_tracker.task[1].used && tgcx_cf_tracker.task[1].record==2 && tgcx_cf_orders.item[0].active && tgcx_cf_orders.item[1].active,
        "a load resumes every saved order of the player's tag in its date window");
    tgcx_Expect(tgcx_cf_stack_vtable==0x7AB0 && tgcx_cf_s_moves==moves_before,"resume takes the land-stack class from the target; sends nothing that day");
    tgcx_CfPut<uint32_t>(world,0xB0C,111);tgcx_CfDaily(g,stub);
    tgcx_Expect(tgcx_cf_s_moves==moves_before+1 && tgcx_cf_s_move_province==10,"a resumed regiment at the rally point marches to the conformed stack");
    tgcx_CfPut<uint32_t>(world,0xB0C,112);tgcx_CfDaily(g,stub);
    tgcx_Expect(!tgcx_cf_tracker.task[0].used && !tgcx_cf_orders.item[0].active && tgcx_cf_orders.item[0].done==112 && tgcx_cf_orders_dirty,
        "a resumed order whose construction is gone ends, and its record is closed that day");
    tgcx_Expect(tgcx_cf_tracker.task[1].used && tgcx_cf_orders.item[1].active,"the other resumed order continues");
    tgcx_CfPut<uint32_t>(world,0xB0C,112+46);tgcx_CfDaily(g,stub);
    tgcx_Expect(!tgcx_cf_tracker.task[1].used && !tgcx_cf_orders.item[1].active,
        "a 46-day jump at one-day steps is a load: nothing in its window resumes");
    tgcx_CfPut<uint32_t>(world,0xB0C,105);tgcx_CfPut<uint32_t>(reinterpret_cast<void*>(tgcx_cf_f_stack[0]),0xC0,0x474E45);
    tgcx_CfDaily(g,stub);
    tgcx_Expect(!tgcx_cf_tracker.task[0].used && !tgcx_cf_tracker.task[1].used,"orders of another tag are not resumed");
    tgcx_CfPut<uint32_t>(reinterpret_cast<void*>(tgcx_cf_f_stack[0]),0xC0,0x415246);
    // Conform records the orders of the regiments it builds.
    memset(&tgcx_cf_tracker,0,sizeof(tgcx_cf_tracker));tgcx_cf_orders.count=0;tgcx_cf_date=105;
    want[4]=3;
    r=tgcx_CfConformWith(g,stub,target,tgcx_cf_f_units,want,&plan);
    tgcx_Expect(r.queued && tgcx_cf_orders.count==r.queued && tgcx_cf_orders.item[0].tag==0x415246 && tgcx_cf_orders.item[0].created==105 &&
        tgcx_cf_orders.item[0].target.id==50 && tgcx_cf_orders.item[0].construction.id==500 && tgcx_cf_orders.item[0].stack.id==600 &&
        tgcx_cf_orders.item[0].active,"Conform saves an order per regiment it builds: tag, date, stack and construction ids");
    bool linked=true;for(const auto& t:tgcx_cf_tracker.task)if(t.used && t.kind==TGCX_CF_JOIN && !t.record)linked=false;
    tgcx_Expect(linked,"every built regiment's task is linked to its saved order");
    tgcx_cf_orders_ok=false;tgcx_cf_orders.count=0;tgcx_cf_orders_dirty=false;
    tgcx_cf_stack_vtable=0;memset(&tgcx_cf_tracker,0,sizeof(tgcx_cf_tracker));tgcx_cf_world=0;tgcx_cf_date=0;tgcx_cf_player=-1;tgcx_cf_step=0;
    printf("Conform native: registry walk, stack read, command ABIs, id lock, driver and one Conform checked\n");
}
