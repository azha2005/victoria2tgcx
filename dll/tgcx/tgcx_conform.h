// TPL-1 Conform to template (mockup board 6-conform; D12, D13). Pure planning
// and session tracking: nothing here reads or writes the game. Every change it
// asks for is an engine command (R5): CCreateUnitCommand + CTransferSubUnitCommand
// detach extras, TPL-0's CConstructUnitCommand builds, CMoveCommand (safe path)
// then CMergeCommand bring the new stacks. Offsets and ABIs: docs/symbols.md "TPL-1".
#pragma once
#include "tgcx_templates.h"

// A registry id: type < 0x1269 lives in the low table, otherwise the high one.
struct tgcx_CfId { uint32_t type,id; };
static bool tgcx_CfSame(const tgcx_CfId& a,const tgcx_CfId& b) { return a.type==b.type && a.id==b.id; }
static bool tgcx_CfNone(const tgcx_CfId& a) { return !a.type && !a.id; }

constexpr unsigned tgcx_cf_regiments=128,tgcx_cf_idle=256,tgcx_cf_tasks=256;
constexpr uint32_t tgcx_cf_build_days=730;   // longest wait for a construction never seen

// ---------------------------------------------------------------- planning (pure)

// unit is the unit type index (+0x24); -1 when the catalogue cannot name it.
struct tgcx_CfRegiment { tgcx_CfId id;int unit; };
// A finished one-regiment stack the player owns (D13: gathered after a reload).
// cost orders the choice, lower first; the caller supplies it.
struct tgcx_CfIdle { tgcx_CfId stack;int unit,province,cost; };
enum tgcx_CfRefusal { TGCX_CF_OK,TGCX_CF_MATCHES,TGCX_CF_NOTHING_KEPT,TGCX_CF_TOO_LARGE };
struct tgcx_CfPlan {
    tgcx_CfRefusal refusal;
    unsigned detach_count;tgcx_CfRegiment detach[tgcx_cf_regiments];   // leave into one new stack
    unsigned gather_count;tgcx_CfIdle gather[tgcx_cf_regiments];        // march and join
    int build[tgcx_tpl_units];unsigned build_total;                     // to TPL-0's recruit planner
};

// Extras of each type are the last ones of that type in the stack's order, so
// the first regiments stay. Regiments the catalogue cannot name are kept and
// never counted against the template. Missing regiments come first from
// pending ones (already on their way this session), then from idle stacks
// (cheapest first), then are built. A plan that would empty the stack is
// refused: nothing is disbanded, and an empty stack is not a conform.
static void tgcx_CfMakePlan(const tgcx_CfRegiment* regs,unsigned reg_count,const int* want,const int* pending,
                            const tgcx_CfIdle* idle,unsigned idle_count,tgcx_CfPlan* out) {
    out->refusal=TGCX_CF_OK;out->detach_count=out->gather_count=out->build_total=0;
    memset(out->build,0,sizeof(out->build));
    unsigned total=0;
    for(unsigned u=0;u<tgcx_tpl_units;++u)total+=want[u]>0?want[u]:0;
    if(reg_count>tgcx_cf_regiments || total>tgcx_cf_regiments) { out->refusal=TGCX_CF_TOO_LARGE;return; }
    int have[tgcx_tpl_units]={};
    for(unsigned i=0;i<reg_count;++i)if(regs[i].unit>=0 && regs[i].unit<static_cast<int>(tgcx_tpl_units))++have[regs[i].unit];
    int keep[tgcx_tpl_units];
    for(unsigned u=0;u<tgcx_tpl_units;++u)keep[u]=(std::min)(have[u],want[u]>0?want[u]:0);
    unsigned kept=0;int seen[tgcx_tpl_units]={};
    for(unsigned i=0;i<reg_count;++i) {
        int u=regs[i].unit;
        if(u<0 || u>=static_cast<int>(tgcx_tpl_units) || seen[u]++<keep[u]) { ++kept;continue; }
        out->detach[out->detach_count++]=regs[i];
    }
    if(!kept && reg_count) { out->refusal=TGCX_CF_NOTHING_KEPT;out->detach_count=0;return; }
    bool used[tgcx_cf_idle]={};
    for(unsigned u=0;u<tgcx_tpl_units;++u) {
        int missing=(want[u]>0?want[u]:0)-have[u]-(pending?pending[u]:0);
        while(missing>0) {
            int best=-1;
            for(unsigned k=0;k<idle_count && k<tgcx_cf_idle;++k)
                if(!used[k] && idle[k].unit==static_cast<int>(u) && (best<0 || idle[k].cost<idle[best].cost))best=static_cast<int>(k);
            if(best<0)break;
            used[best]=true;out->gather[out->gather_count++]=idle[best];--missing;
        }
        if(missing>0) { out->build[u]=missing;out->build_total+=missing; }
    }
    if(!out->detach_count && !out->gather_count && !out->build_total)out->refusal=TGCX_CF_MATCHES;
}

// ---------------------------------------------------------------- session tracking (pure)

// JOIN: a stack (one being built, or an idle one) marches to the target and
// merges into it. FILL: a stack created for the extras receives them.
enum tgcx_CfKind { TGCX_CF_JOIN,TGCX_CF_FILL };
struct tgcx_CfTask {
    bool used;tgcx_CfKind kind;
    tgcx_CfId target,stack,construction;   // construction: JOIN of a regiment still being built
    int unit,origin;unsigned regiments;tgcx_CfId regiment[tgcx_cf_regiments];   // JOIN origin: recruitment province; FILL: extras
    bool appeared,merge_sent,construction_seen;int moved_to;uint32_t merge_date,created_date;unsigned misses;
    bool transfer_sent;uint32_t transfer_date;unsigned retries;   // FILL: TPL-4 checks the extras arrived
    unsigned record;   // 1-based saved order (TPL-7), 0 for none
};
// What the native layer reads each game day; ids only, never pointers kept.
struct tgcx_CfSeen {
    bool target_alive;int target_province;
    bool stack_alive;int stack_province;
    bool construction_alive;
};
enum tgcx_CfAction { TGCX_CF_WAIT,TGCX_CF_MOVE,TGCX_CF_MERGE,TGCX_CF_TRANSFER,TGCX_CF_DONE,
                     TGCX_CF_TARGET_GONE,TGCX_CF_CANCELLED,TGCX_CF_VERIFY };

// A stack that does not appear while its construction lives is still being
// built. Two consecutive days with neither, once the construction was seen
// (or for a gathered stack, which has none), means it was cancelled; one day
// of grace covers a read between the two. The construction's own identity is
// inferred (symbols.md), so one never seen only times out. A FILL stack is
// given a week to appear. A JOIN re-sends Move only when the target changes
// province, so a user's own orders to the new stack are not fought; it merges
// when both are in one province, at most one Merge per game day.
static tgcx_CfAction tgcx_CfStep(tgcx_CfTask* t,const tgcx_CfSeen& s,uint32_t date,int* province) {
    if(!t->used)return TGCX_CF_DONE;
    if(!s.stack_alive) {
        if(t->kind==TGCX_CF_FILL && t->appeared) { t->used=false;return TGCX_CF_VERIFY; }
        if(t->appeared) { t->used=false;return TGCX_CF_DONE; }   // merged, or gone after it existed
        if(t->kind==TGCX_CF_FILL) {
            if(date>=t->created_date+7) { t->used=false;return TGCX_CF_CANCELLED; }
            return TGCX_CF_WAIT;
        }
        if(s.construction_alive) { t->construction_seen=true;t->misses=0;return TGCX_CF_WAIT; }
        if(t->construction_seen || tgcx_CfNone(t->construction)) {
            if(++t->misses>=2) { t->used=false;return TGCX_CF_CANCELLED; }
            return TGCX_CF_WAIT;
        }
        if(date>=t->created_date+tgcx_cf_build_days) { t->used=false;return TGCX_CF_CANCELLED; }
        return TGCX_CF_WAIT;
    }
    t->appeared=true;t->misses=0;
    if(!s.target_alive) { t->used=false;return TGCX_CF_TARGET_GONE; }
    // Create and Transfers are queued together. The daily driver verifies the
    // result; TRANSFER is also used for a retry into a surviving stack.
    if(t->kind==TGCX_CF_FILL) {
        if(!t->transfer_sent) { t->transfer_sent=true;t->transfer_date=date;return TGCX_CF_TRANSFER; }
        if(date==t->transfer_date)return TGCX_CF_WAIT;
        t->used=false;return TGCX_CF_VERIFY;
    }
    if(s.stack_province==s.target_province) {
        if(t->merge_sent && t->merge_date==date)return TGCX_CF_WAIT;
        t->merge_sent=true;t->merge_date=date;return TGCX_CF_MERGE;
    }
    if(t->moved_to!=s.target_province) { t->moved_to=s.target_province;*province=s.target_province;return TGCX_CF_MOVE; }
    return TGCX_CF_WAIT;
}

struct tgcx_CfTracker { tgcx_CfTask task[tgcx_cf_tasks]; };
static tgcx_CfTask* tgcx_CfAdd(tgcx_CfTracker* k,tgcx_CfKind kind,const tgcx_CfId& target,const tgcx_CfId& stack,
                              const tgcx_CfId& construction,int unit,uint32_t date) {
    for(auto& t:k->task)if(t.used && tgcx_CfSame(t.stack,stack))return nullptr;   // one task per stack
    for(auto& t:k->task)if(!t.used) {
        memset(&t,0,sizeof(t));t.used=true;t.kind=kind;t.target=target;t.stack=stack;t.construction=construction;
        t.unit=unit;t.origin=-1;t.moved_to=-1;t.created_date=date;return &t;
    }
    return nullptr;
}
// Regiments of each type already coming to this target (planner input).
static void tgcx_CfPending(const tgcx_CfTracker& k,const tgcx_CfId& target,int* pending) {
    memset(pending,0,sizeof(int)*tgcx_tpl_units);
    for(const auto& t:k.task)
        if(t.used && t.kind==TGCX_CF_JOIN && tgcx_CfSame(t.target,target) && t.unit>=0 && t.unit<static_cast<int>(tgcx_tpl_units))
            ++pending[t.unit];
}

// ---------------------------------------------------------------- saved orders (pure; TPL-7, D13 revised)

// A JOIN of a regiment being built, kept in a local file so a reload resumes
// it. The save stores a construction's own id and its future stack id
// ("unit"), so both survive. An order applies to a loaded game of the same
// tag whose date lies from its creation up to (not including) its end: a
// save made before the order, or after it finished, never resumes it.
constexpr unsigned tgcx_cf_orders_max=512;
struct tgcx_CfOrder { uint32_t tag,created,done;tgcx_CfId target,stack,construction;int unit;bool active; };
struct tgcx_CfOrders { unsigned count;tgcx_CfOrder item[tgcx_cf_orders_max]; };
static bool tgcx_CfOrderApplies(const tgcx_CfOrder& r,uint32_t tag,uint32_t date) {
    return r.tag==tag && r.created<=date && (!r.done || date<r.done);
}
// Full: the oldest finished order goes first, else the oldest open one.
// *dropped: the index removed (later ones moved down one), or the maximum.
static unsigned tgcx_CfOrderAdd(tgcx_CfOrders* k,const tgcx_CfOrder& r,unsigned* dropped=nullptr) {
    if(dropped)*dropped=tgcx_cf_orders_max;
    if(k->count>=tgcx_cf_orders_max) {
        unsigned drop=0;bool done=false;
        for(unsigned i=0;i<k->count;++i) {
            const auto& c=k->item[i];bool d=c.done!=0;
            if((d && !done) || (d==done && c.created<k->item[drop].created)) { drop=i;done=d; }
        }
        for(unsigned i=drop+1;i<k->count;++i)k->item[i-1]=k->item[i];
        --k->count;if(dropped)*dropped=drop;
    }
    k->item[k->count]=r;k->item[k->count].active=false;
    return k->count++;
}
static bool tgcx_CfOrderLine(const tgcx_CfOrder& r,char* line,size_t size) {
    return sprintf_s(line,size,"order tag=%08X created=%u done=%u target=%u/%u stack=%u/%u construction=%u/%u unit=%d\r\n",
        r.tag,r.created,r.done,r.target.type,r.target.id,r.stack.type,r.stack.id,r.construction.type,r.construction.id,r.unit)>0;
}
static size_t tgcx_CfOrdersWrite(const tgcx_CfOrders& k,char* out,size_t size) {
    size_t used=0;char line[192];
    const char* header="tgcx_conform_orders 1\r\n";size_t n=strlen(header);
    if(n>=size)return 0;memcpy(out,header,n);used=n;
    for(unsigned i=0;i<k.count;++i) {
        if(!tgcx_CfOrderLine(k.item[i],line,sizeof(line)))return 0;
        n=strlen(line);if(used+n>=size)return 0;memcpy(out+used,line,n);used+=n;
    }
    out[used]=0;return used;
}
// Strict: every line must print back byte for byte; anything else refuses
// the whole file (nothing is resumed from a damaged one).
static bool tgcx_CfOrdersParse(const char* text,tgcx_CfOrders* out) {
    out->count=0;
    const char* header="tgcx_conform_orders 1\r\n";
    if(strncmp(text,header,strlen(header)))return false;
    for(const char* at=text+strlen(header);*at;) {
        const char* end=strstr(at,"\r\n");if(!end || end-at>=190 || out->count>=tgcx_cf_orders_max)return false;
        char line[192];memcpy(line,at,end-at);line[end-at]=0;
        tgcx_CfOrder r={};
        if(sscanf_s(line,"order tag=%8X created=%u done=%u target=%u/%u stack=%u/%u construction=%u/%u unit=%d",
            &r.tag,&r.created,&r.done,&r.target.type,&r.target.id,&r.stack.type,&r.stack.id,&r.construction.type,&r.construction.id,&r.unit)!=10)return false;
        char back[192];
        if(!tgcx_CfOrderLine(r,back,sizeof(back)) || strncmp(back,at,end-at+2) || back[end-at+2])return false;
        if(tgcx_CfNone(r.target) || tgcx_CfNone(r.stack) || r.unit<0 || r.unit>=static_cast<int>(tgcx_tpl_units))return false;
        out->item[out->count++]=r;at=end+2;
    }
    return true;
}
