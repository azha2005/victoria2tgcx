// TPL-5/TPL-6: read-only recruitment preferences. No simulation state writes.
// Field evidence and guards: docs/report-army-filters-research-1019.md.
#pragma once

enum tgcx_CultureMode { TGCX_CULTURE_ALL, TGCX_CULTURE_ACCEPTED, TGCX_CULTURE_PRIMARY };
struct tgcx_ArmyFilter { unsigned culture;uintptr_t continent; };
static bool tgcx_army_filters_enabled=false;
constexpr unsigned tgcx_filter_cultures_max=1024,tgcx_continents_max=128;
struct tgcx_CultureSnapshot { uintptr_t primary,accepted[tgcx_filter_cultures_max];unsigned count; };

static bool tgcx_ReadCulturesImpl(uintptr_t country,tgcx_CultureSnapshot* out) {
    if(!country)return false;
    out->primary=tgcx_TplRead<uintptr_t>(country+0xE64);out->count=0;
    unsigned count=tgcx_TplRead<unsigned>(country+0xE5C);
    if(!out->primary || count>tgcx_filter_cultures_max)return false;
    uintptr_t node=tgcx_TplRead<uintptr_t>(country+0xE54),last=0;
    for(unsigned i=0;i<count;++i) {
        if(!node || tgcx_TplRead<uintptr_t>(node+4)!=last)return false;
        uintptr_t value=tgcx_TplRead<uintptr_t>(node);if(!value)return false;
        out->accepted[out->count++]=value;last=node;node=tgcx_TplRead<uintptr_t>(node+8);
    }
    if(node || last!=tgcx_TplRead<uintptr_t>(country+0xE58))return false;
    std::sort(out->accepted,out->accepted+out->count);
    return true;
}
static bool tgcx_ReadCultures(uintptr_t country,tgcx_CultureSnapshot* out) {
    __try {return tgcx_ReadCulturesImpl(country,out);} __except(EXCEPTION_EXECUTE_HANDLER) {return false;}
}
static bool tgcx_ArmyApply(uintptr_t country,tgcx_TplPop* pops,unsigned count,const tgcx_ArmyFilter& filter) {
    if(filter.culture>TGCX_CULTURE_PRIMARY || count>tgcx_tpl_pops)return false;
    if(!filter.culture && !filter.continent)return true;
    if(!tgcx_army_filters_enabled)return false;
    tgcx_CultureSnapshot cultures={};
    if(filter.culture && !tgcx_ReadCultures(country,&cultures))return false;
    for(unsigned i=0;i<count;++i) {
        if(filter.culture && !pops[i].culture)return false;
        bool match=!filter.culture || pops[i].culture==cultures.primary ||
            (filter.culture==TGCX_CULTURE_ACCEPTED && std::binary_search(cultures.accepted,cultures.accepted+cultures.count,pops[i].culture));
        if(!match || (filter.continent && pops[i].continent!=filter.continent))pops[i].free=0;
    }
    return true;
}

struct tgcx_Continent { uintptr_t object;char key[64],label[128]; };
struct tgcx_ContinentCatalogue {
    uintptr_t world,manager,begin,end;unsigned count;bool valid;tgcx_Continent item[tgcx_continents_max];
};
static tgcx_ContinentCatalogue tgcx_continents={};
static bool tgcx_ContinentsReadImpl(uintptr_t world,uintptr_t manager,uintptr_t expected_vt,tgcx_ContinentCatalogue* out) {
    if(!world || !manager) {out->valid=false;return false;}
    uintptr_t begin=tgcx_TplRead<uintptr_t>(manager+0x2A78),end=tgcx_TplRead<uintptr_t>(manager+0x2A7C);
    if(!begin || end<=begin || (end-begin)%4 || (end-begin)/4>tgcx_continents_max) {out->valid=false;return false;}
    if(out->valid && out->world==world && out->manager==manager && out->begin==begin && out->end==end)return true;
    out->valid=false;out->count=0;
    for(uintptr_t at=begin;at<end;at+=4) {
        tgcx_Continent& item=out->item[out->count];item.object=tgcx_TplRead<uintptr_t>(at);
        if(!item.object || tgcx_TplRead<uintptr_t>(item.object)!=expected_vt ||
           !tgcx_ReadGameString(item.object+0x40,item.key,sizeof(item.key)) || !item.key[0] ||
           !tgcx_ReadGameString(item.object+0x5C,item.label,sizeof(item.label)))return false;
        if(!item.label[0])strcpy_s(item.label,item.key);
        for(unsigned i=0;i<out->count;++i)if(!strcmp(out->item[i].key,item.key))return false;
        ++out->count;
    }
    std::sort(out->item,out->item+out->count,[](const tgcx_Continent& a,const tgcx_Continent& b){return strcmp(a.key,b.key)<0;});
    out->world=world;out->manager=manager;out->begin=begin;out->end=end;out->valid=true;return true;
}
static bool tgcx_ContinentsRead(uintptr_t world,uintptr_t manager,uintptr_t expected_vt,tgcx_ContinentCatalogue* out) {
    __try {return tgcx_ContinentsReadImpl(world,manager,expected_vt,out);}
    __except(EXCEPTION_EXECUTE_HANDLER) {out->valid=false;return false;}
}
static bool tgcx_ContinentsLive(uintptr_t world) {
    if(!tgcx_army_filters_enabled)return false;
    __try {return tgcx_ContinentsRead(world,tgcx_TplRead<uintptr_t>(tgcx_base+0xE5870C),tgcx_base+0x9FC1DC,&tgcx_continents);}
    __except(EXCEPTION_EXECUTE_HANDLER) {tgcx_continents.valid=false;return false;}
}
static const tgcx_Continent* tgcx_ContinentByKey(const tgcx_ContinentCatalogue& catalogue,const char* key) {
    if(!catalogue.valid || !key || !key[0])return nullptr;
    for(unsigned i=0;i<catalogue.count;++i)if(!strcmp(catalogue.item[i].key,key))return &catalogue.item[i];
    return nullptr;
}
// The builder offers only continents with a province the country owns
// (country +0x9D8/+0x9DC province ids, g_world+0xACC province table). Only
// the selector's choices narrow; the filter itself is unchanged.
static bool tgcx_ContinentsPresentImpl(uintptr_t world,uintptr_t country,const tgcx_ContinentCatalogue& catalogue,bool* present) {
    memset(present,0,sizeof(bool)*tgcx_continents_max);
    if(!world || !country || !catalogue.valid)return false;
    uintptr_t provinces=tgcx_TplRead<uintptr_t>(world+0xACC);
    uintptr_t owned=tgcx_TplRead<uintptr_t>(country+0x9D8),owned_end=tgcx_TplRead<uintptr_t>(country+0x9DC);
    if(!provinces || owned_end<owned || (owned_end-owned)%4 || (owned_end-owned)/4>65536)return false;
    for(uintptr_t p=owned;p<owned_end;p+=4) {
        int id=tgcx_TplRead<int>(p);if(id<0)return false;
        uintptr_t province=tgcx_TplRead<uintptr_t>(provinces+id*4);if(!province)continue;
        uintptr_t continent=tgcx_TplRead<uintptr_t>(province+0x16C);
        for(unsigned i=0;i<catalogue.count;++i)if(catalogue.item[i].object==continent)present[i]=true;
    }
    return true;
}
static bool tgcx_ContinentsPresent(uintptr_t world,uintptr_t country,const tgcx_ContinentCatalogue& catalogue,bool* present) {
    __try {return tgcx_ContinentsPresentImpl(world,country,catalogue,present);}
    __except(EXCEPTION_EXECUTE_HANDLER) {memset(present,0,sizeof(bool)*tgcx_continents_max);return false;}
}
// Next choice from current (0 = Any, i+1 = catalogue item i), skipping
// continents not present. With no presence data every continent is offered.
static unsigned tgcx_ContinentStep(const tgcx_ContinentCatalogue& catalogue,const bool* present,unsigned current,bool forward) {
    unsigned n=catalogue.count+1,next=current%n;
    for(unsigned tries=0;tries<n;++tries) {
        next=(next+(forward?1:n-1))%n;
        if(!next || !present || present[next-1])return next;
    }
    return 0;
}
