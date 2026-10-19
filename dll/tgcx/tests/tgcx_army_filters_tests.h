// Independent engine-shaped fixtures: pop-specific policy, native metadata,
// actual planner/command selection, catalogue bounds and rollback-safe files.
#pragma once
static uintptr_t tgcx_filter_test_picked[128];static unsigned tgcx_filter_test_count;
static void* tgcx_FilterTestMake(uintptr_t unit,int province,uintptr_t pop,uint32_t tag,int country) {
    tgcx_filter_test_picked[tgcx_filter_test_count++]=pop;
    return tgcx_TplStubMake(unit,province,pop,tag,country);
}
static bool tgcx_FilterTestUnitAllowed(uintptr_t,uintptr_t pop,uintptr_t,uintptr_t) {
    return tgcx_TplRead<uintptr_t>(pop+0x6C)!=1;
}
static bool tgcx_FilterTestNoRoute(void*,int) {return false;}
static void tgcx_FilterTestString(unsigned char* at,const char* text) {
    memset(at,0,24);size_t n=strlen(text);
    if(n<16) {memcpy(at,text,n);*reinterpret_cast<unsigned*>(at+20)=15;}
    else {*reinterpret_cast<const char**>(at)=text;*reinterpret_cast<unsigned*>(at+20)=static_cast<unsigned>(n);}
    *reinterpret_cast<unsigned*>(at+16)=static_cast<unsigned>(n);
}
static void tgcx_ArmyFilterTests() {
    bool saved=tgcx_army_filters_enabled,saved_universal=tgcx_army_universal;
    tgcx_army_filters_enabled=true;tgcx_army_universal=false;
    static unsigned char country[0x1000],world[0xC00],stats[0x400],province[0x200],pop[3][0x290],soldier[0x50],prototype[0x50],unit[0xD0];
    memset(country,0,sizeof(country));memset(pop,0,sizeof(pop));
    uintptr_t nodes[3]={2,0,0};
    *reinterpret_cast<uintptr_t*>(country+0xE64)=1;
    *reinterpret_cast<uintptr_t*>(country+0xE54)=*reinterpret_cast<uintptr_t*>(country+0xE58)=reinterpret_cast<uintptr_t>(nodes);
    *reinterpret_cast<unsigned*>(country+0xE5C)=1;
    uintptr_t countries[]={0,reinterpret_cast<uintptr_t>(country)},manager[]={2,reinterpret_cast<uintptr_t>(countries)};
    uintptr_t bases[]={0,0,0,reinterpret_cast<uintptr_t>(prototype)},types[]={0,0,0,reinterpret_cast<uintptr_t>(unit)};
    uintptr_t db[5]={},popdb[5]={},kinds[]={0,reinterpret_cast<uintptr_t>(soldier)},provinces[]={0,reinterpret_cast<uintptr_t>(province)};
    int owned[]={1};uintptr_t buckets[8]={};
    db[3]=reinterpret_cast<uintptr_t>(bases);db[4]=db[3]+sizeof(bases);
    popdb[3]=reinterpret_cast<uintptr_t>(kinds);popdb[4]=popdb[3]+sizeof(kinds);
    *reinterpret_cast<int*>(world+0xB60)=1;*reinterpret_cast<uintptr_t*>(world+0xACC)=reinterpret_cast<uintptr_t>(provinces);
    *reinterpret_cast<uintptr_t*>(country+0xBCC)=reinterpret_cast<uintptr_t>(stats);
    *reinterpret_cast<uintptr_t*>(stats+0x310)=reinterpret_cast<uintptr_t>(types);
    *reinterpret_cast<uintptr_t*>(country+0x9D8)=reinterpret_cast<uintptr_t>(owned);
    *reinterpret_cast<uintptr_t*>(country+0x9DC)=reinterpret_cast<uintptr_t>(owned)+sizeof(owned);
    *reinterpret_cast<uintptr_t*>(province+0x194)=reinterpret_cast<uintptr_t>(buckets);
    *reinterpret_cast<int*>(province+0x58)=1;*reinterpret_cast<uintptr_t*>(province+0x16C)=11;
    buckets[4]=reinterpret_cast<uintptr_t>(pop[0]);soldier[0x42]=1;
    *reinterpret_cast<int*>(prototype+0x24)=3;tgcx_FilterTestString(prototype+8,"infantry");
    *reinterpret_cast<int*>(unit+0x24)=3;unit[0x39]=1;
    for(unsigned i=0;i<3;++i) {
        *reinterpret_cast<uintptr_t*>(pop[i]+0x6C)=i+1;
        *reinterpret_cast<int*>(pop[i]+0x100)=i==0?1:i==1?2:30;
        *reinterpret_cast<uintptr_t*>(pop[i]+0x27C)=i<2?reinterpret_cast<uintptr_t>(pop[i+1]):0;
    }
    tgcx_TplGlobals g={reinterpret_cast<uintptr_t>(world),reinterpret_cast<uintptr_t>(manager),reinterpret_cast<uintptr_t>(db),reinterpret_cast<uintptr_t>(popdb),0xC0FFEE};
    tgcx_TplNative native={tgcx_TplStubCanBuild,tgcx_TplStubFree,tgcx_FilterTestMake,tgcx_TplStubPost};
    int counts[tgcx_tpl_units]={};counts[3]=4;static tgcx_TplCountry scan;static tgcx_TplPlan plan;
    for(unsigned mode=0;mode<=2;++mode) {
        tgcx_ArmyFilter filter={mode,0};
        tgcx_Expect(tgcx_TplScan(g,native,&scan) && scan.pop_count==3,"Filters read all mixed-culture soldier pops in one province");
        tgcx_Expect(tgcx_ArmyApply(scan.country,scan.pops,scan.pop_count,filter),"Culture snapshot is validated once for planning");
        tgcx_TplMakePlan(counts,scan.pops,scan.pop_count,&plan);
        tgcx_tpl_s_made=tgcx_tpl_s_posted=tgcx_filter_test_count=0;tgcx_tpl_s_post_limit=16;tgcx_tpl_s_vtable=g.command_vtable;
        auto r=tgcx_TplRecruitWith(g,native,counts,nullptr,nullptr,nullptr,nullptr,&filter);
        unsigned expected=mode==0?4:mode==1?3:1;
        tgcx_Expect(r.scanned && r.queued==expected && r.missing==4-expected && plan.count==r.queued && plan.missing==r.missing,
                    "Culture preview and actual command planner agree, including shortage");
        for(unsigned i=0;i<tgcx_filter_test_count;++i) {
            auto chosen=tgcx_filter_test_picked[i];
            tgcx_Expect(!mode || chosen!=reinterpret_cast<uintptr_t>(pop[2]),"Restricted commands never choose the foreign pop sharing a province");
            tgcx_Expect(mode!=2 || chosen==reinterpret_cast<uintptr_t>(pop[0]),"Primary-only commands keep the exact primary pop");
        }
    }
    tgcx_ArmyFilter filter={TGCX_CULTURE_PRIMARY,11};
    auto limited=native;limited.can_build=tgcx_FilterTestUnitAllowed;
    tgcx_tpl_s_made=tgcx_tpl_s_posted=tgcx_filter_test_count=0;
    auto restricted=tgcx_TplRecruitWith(g,limited,counts,nullptr,nullptr,nullptr,nullptr,&filter);
    tgcx_Expect(restricted.scanned && !restricted.queued && restricted.missing==4,
                "Culture preference cannot bypass native unit-specific eligibility");
    restricted=tgcx_TplRecruitWith(g,native,counts,nullptr,nullptr,tgcx_FilterTestNoRoute,nullptr,&filter);
    tgcx_Expect(restricted.scanned && !restricted.queued && restricted.missing==4,
                "An eligible primary pop still needs the independent Conform land route");
    filter={TGCX_CULTURE_ACCEPTED,99};tgcx_TplScan(g,native,&scan);
    tgcx_Expect(tgcx_ArmyApply(scan.country,scan.pops,scan.pop_count,filter),"Culture and continent combine without overriding native eligibility");
    tgcx_TplMakePlan(counts,scan.pops,scan.pop_count,&plan);
    tgcx_Expect(plan.count==0 && plan.missing==4,"Selected other continent reports all shortage, never falls back to Any");
    filter.continent=11;filter.culture=TGCX_CULTURE_PRIMARY;
    *reinterpret_cast<uintptr_t*>(country+0xE64)=3;tgcx_TplScan(g,native,&scan);
    tgcx_Expect(tgcx_ArmyApply(scan.country,scan.pops,scan.pop_count,filter) && scan.pops[2].free==30 && scan.pops[0].free==0,
                "Changing primary culture is observed without cached file metadata");
    nodes[2]=reinterpret_cast<uintptr_t>(nodes);tgcx_CultureSnapshot cs;
    tgcx_Expect(!tgcx_ReadCultures(reinterpret_cast<uintptr_t>(country),&cs),"Cyclic accepted-culture list refuses recruitment");nodes[2]=0;
    tgcx_Expect(!tgcx_ReadCultures(1,&cs),"Invalid culture memory is guarded");
    *reinterpret_cast<unsigned*>(country+0xE5C)=0;*reinterpret_cast<uintptr_t*>(country+0xE54)=*reinterpret_cast<uintptr_t*>(country+0xE58)=0;
    tgcx_Expect(tgcx_ReadCultures(reinterpret_cast<uintptr_t>(country),&cs) && cs.count==0 && cs.primary==3,"Empty accepted list still has a primary culture");

    static unsigned char source[0x2A90],continents[14][0x80];uintptr_t entries[14];
    char keys[14][16];for(unsigned i=0;i<14;++i) {
        entries[i]=reinterpret_cast<uintptr_t>(continents[i]);*reinterpret_cast<uintptr_t*>(continents[i])=1234;
        sprintf_s(keys[i],"custom_%02u",i);tgcx_FilterTestString(continents[i]+0x40,keys[i]);
        tgcx_FilterTestString(continents[i]+0x5C,"Translated continent name");
    }
    tgcx_ContinentCatalogue catalogue={};
    for(unsigned n: {6u,8u,14u}) {
        *reinterpret_cast<uintptr_t*>(source+0x2A78)=reinterpret_cast<uintptr_t>(entries);
        *reinterpret_cast<uintptr_t*>(source+0x2A7C)=reinterpret_cast<uintptr_t>(entries)+n*4;
        tgcx_Expect(tgcx_ContinentsRead(n,reinterpret_cast<uintptr_t>(source),1234,&catalogue) && catalogue.count==n,
                    "Active continent catalogue supports six, eight and fourteen custom entries");
        tgcx_Expect(tgcx_ContinentByKey(catalogue,keys[n-1]) && !tgcx_ContinentByKey(catalogue,"removed_key"),
                    "Continent identity uses stable keys, even when translated labels coincide");
    }
    // Owned provinces on items 1 and 4 only: the selector skips the rest.
    {
        static unsigned char owned_world[0xAD0],owner[0x9E0],owned_province[3][0x170];static uintptr_t table[3];static int ids[3]={0,1,2};
        *reinterpret_cast<uintptr_t*>(owned_world+0xACC)=reinterpret_cast<uintptr_t>(table);
        for(unsigned i=0;i<3;++i)table[i]=reinterpret_cast<uintptr_t>(owned_province[i]);
        *reinterpret_cast<uintptr_t*>(owned_province[0]+0x16C)=catalogue.item[4].object;
        *reinterpret_cast<uintptr_t*>(owned_province[1]+0x16C)=catalogue.item[1].object;
        *reinterpret_cast<uintptr_t*>(owned_province[2]+0x16C)=catalogue.item[4].object;
        *reinterpret_cast<uintptr_t*>(owner+0x9D8)=reinterpret_cast<uintptr_t>(ids);
        *reinterpret_cast<uintptr_t*>(owner+0x9DC)=reinterpret_cast<uintptr_t>(ids+3);
        bool present[tgcx_continents_max];
        tgcx_Expect(tgcx_ContinentsPresent(reinterpret_cast<uintptr_t>(owned_world),reinterpret_cast<uintptr_t>(owner),catalogue,present) &&
                    present[1] && present[4] && !present[0] && !present[13],"Present continents come from owned provinces only");
        tgcx_Expect(tgcx_ContinentStep(catalogue,present,0,true)==2 && tgcx_ContinentStep(catalogue,present,2,true)==5 &&
                    tgcx_ContinentStep(catalogue,present,5,true)==0 && tgcx_ContinentStep(catalogue,present,0,false)==5,
                    "Continent selector cycles Any and present continents only, both directions");
        tgcx_Expect(tgcx_ContinentStep(catalogue,nullptr,0,true)==1,"Unknown presence offers every continent");
        *reinterpret_cast<uintptr_t*>(owner+0x9DC)=reinterpret_cast<uintptr_t>(ids)+2;
        tgcx_Expect(!tgcx_ContinentsPresent(reinterpret_cast<uintptr_t>(owned_world),reinterpret_cast<uintptr_t>(owner),catalogue,present),
                    "Misaligned owned-province list is refused");
        memset(present,0,sizeof(present));
        tgcx_Expect(tgcx_ContinentStep(catalogue,present,0,true)==0,"No present continent leaves only Any");
    }
    *reinterpret_cast<uintptr_t*>(source+0x2A7C)=reinterpret_cast<uintptr_t>(entries)+129*4;
    tgcx_Expect(!tgcx_ContinentsRead(14,reinterpret_cast<uintptr_t>(source),1234,&catalogue),"Oversized catalogue refuses before reading past entries");
    tgcx_army_filters_enabled=saved;tgcx_army_universal=saved_universal;
    printf("Army filters: exact mixed-pop commands, preview/shortage agreement, canonical cultures and dynamic continents checked\n");
}

static void tgcx_FilterStorageTests() {
    wchar_t temp[MAX_PATH],legacy[MAX_PATH],preferred[MAX_PATH];GetTempPathW(MAX_PATH,temp);GetTempFileNameW(temp,L"atc",0,legacy);
    swprintf_s(preferred,L"%s.v2",legacy);
    const char* old="tgcx_templates 1\ntemplate\ninfantry=2\nend\n";
    auto put=[](const wchar_t* path,const char* text) {FILE* file=nullptr;_wfopen_s(&file,path,L"wb");if(file){fwrite(text,1,strlen(text),file);fclose(file);}};
    put(legacy,old);tgcx_TplLibrary library={};bool imported=false;
    tgcx_Expect(tgcx_TplLoadPreferred(preferred,legacy,&library,&imported) && imported && library.size==1 && !library.item[0].culture,
                "Old v1 templates import unrestricted without rewriting the original");
    library.item[0].culture=TGCX_CULTURE_PRIMARY;
    tgcx_Expect(tgcx_TplSaveFile(preferred,library),"Culture preference saves to separate atomic v2 library");
    tgcx_TplLibrary loaded={};
    tgcx_Expect(tgcx_TplLoadPreferred(preferred,legacy,&loaded,&imported) && !imported && !memcmp(&library,&loaded,sizeof(library)),
                "V2 modes/counts round trip and take precedence over legacy file");
    char bytes[128]={};FILE* file=nullptr;_wfopen_s(&file,legacy,L"rb");if(file){fread(bytes,1,sizeof(bytes)-1,file);fclose(file);}
    tgcx_Expect(!strcmp(bytes,old),"Original v1 library stays byte-identical for accepted-build rollback");
    const char* bad[]={"tgcx_templates 2\ntemplate\ninfantry=2\nend\n",
        "tgcx_templates 2\ntemplate\n@culture=9\nend\n",
        "tgcx_templates 2\ntemplate\n@culture=1\n@culture=2\nend\n"};
    for(const char* text:bad) {
        put(preferred,text);loaded=library;
        tgcx_Expect(!tgcx_TplLoadPreferred(preferred,legacy,&loaded,&imported) && !memcmp(&loaded,&library,sizeof(loaded)),
                    "Malformed v2 refuses, preserves memory and never falls back to unrestricted v1");
    }
    DeleteFileW(preferred);DeleteFileW(legacy);
    printf("Army filter storage: v1 import, separate v2 round trip, malformed refusal and rollback preservation checked\n");
}
