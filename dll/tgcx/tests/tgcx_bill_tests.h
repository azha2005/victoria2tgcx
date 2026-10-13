// Isolated Bill view-model/string tests; no game process or state mutation.
#pragma once
struct tgcx_BillTestValue { const char* name; int value; };
static tgcx_BillTestValue tgcx_bill_test_values[]={
    {"tgcx_house_total",100000},{"tgcx_gui_schema",1000},{"tgcx_gui_active",1000},{"tgcx_gui_bill_id",1000},
    {"tgcx_yes",10000},{"tgcx_no",85000},{"tgcx_margin",-75000},{"tgcx_quorum",95000},
    {"tgcx_favours",1000},{"tgcx_rate",3000},{"tgcx_spent",2000},
    {"tgcx_seats_conservative",85000},{"tgcx_seats_liberal",10000},{"tgcx_seats_reactionary",5000},
    {"tgcx_base_yes_liberal",10000},{"tgcx_left_conservative",80000},{"tgcx_left_reactionary",5000},
    {"tgcx_buy_conservative_total",5000},{"tgcx_gui_stance_conservative",-1000},
    {"tgcx_buy_conservative_yes",0},{"tgcx_gui_promised_conservative",0}
};
static int tgcx_BillTestRead(uintptr_t country,const char* key,bool* ok) {
    if(!country){*ok=false;return 0;}
    for(const auto& item:tgcx_bill_test_values)if(strcmp(item.name,key)==0)return item.value;
    return 0;
}
static void tgcx_BillTestSet(const char* key,int value) {
    for(auto& item:tgcx_bill_test_values)if(strcmp(item.name,key)==0){item.value=value;return;}
    tgcx_Expect(false,"Bill test key exists");
}
static void tgcx_BillTests() {
    auto reader=tgcx_bill_reader;tgcx_bill_reader=tgcx_BillTestRead;
    tgcx_BillModel model={};
    tgcx_Expect(tgcx_BillReadModel(1,&model),"saved 10 Aye / 5 abstain / 85 Nay model is valid");
    unsigned cons=0;for(;cons<tgcx_ideology_count;++cons)if(strcmp(tgcx_ideology_candidates[cons],"conservative")==0)break;
    tgcx_Expect(cons<tgcx_ideology_count && model.bloc[cons].yes==0 && model.bloc[cons].abstain==5 && model.bloc[cons].no==80,"bloc purchases are separated from native Aye");
    tgcx_BillTestSet("tgcx_gui_promised_conservative",1000);
    tgcx_BillTestSet("tgcx_left_conservative",0);
    tgcx_BillTestSet("tgcx_yes",95000);tgcx_BillTestSet("tgcx_no",5000);
    tgcx_BillTestSet("tgcx_margin",90000);tgcx_BillTestSet("tgcx_quorum",100000);
    tgcx_Expect(tgcx_BillReadModel(1,&model) && model.bloc[cons].yes==85 && model.bloc[cons].abstain==0,"promise converts old abstentions exactly once in display");
    tgcx_BillTestSet("tgcx_gui_promised_conservative",0);
    tgcx_BillTestSet("tgcx_base_yes_liberal",10000);
    tgcx_BillTestSet("tgcx_buy_conservative_total",100000);tgcx_BillTestSet("tgcx_buy_conservative_yes",100000);
    tgcx_Expect(tgcx_BillReadModel(1,&model) && model.bloc[cons].yes==85,"sunk purchased seats clip to remaining opposition");
    tgcx_BillTestSet("tgcx_favours",1500);tgcx_Expect(!tgcx_BillReadModel(1,&model),"fractional corrupted counter rejected");
    tgcx_BillTestSet("tgcx_favours",1000);
    tgcx_BillTestSet("tgcx_quorum",95000);tgcx_Expect(!tgcx_BillReadModel(1,&model),"inconsistent quorum rejected");tgcx_BillTestSet("tgcx_quorum",100000);
    tgcx_BillTestSet("tgcx_gui_bill_id",145000);tgcx_Expect(!tgcx_BillReadModel(1,&model),"unknown catalogue ID rejected");tgcx_BillTestSet("tgcx_gui_bill_id",0);
    tgcx_BillTestSet("tgcx_gui_schema",0);tgcx_BillTestSet("tgcx_gui_active",0);
    tgcx_Expect(tgcx_BillReadModel(1,&model) && model.id==0,"pre-GUI active save can show totals before Count migration");
    tgcx_BillTestSet("tgcx_gui_schema",1000);
    tgcx_Expect(!tgcx_BillReadModel(1,&model),"count recovery after withdrawal cannot create a phantom active Bill");
    tgcx_Expect(!tgcx_BillReadModel(0,&model),"no country refuses Bill model");
    tgcx_bill_reader=reader;
    auto small_string=tgcx_BorrowString("tgcx_test");
    tgcx_Expect(small_string.capacity==15 && small_string.length==9 && strcmp(small_string.data,"tgcx_test")==0,"native small-string ABI includes terminator");
    const char* text="POLITICSVIEW_DECISIONtgcx_count_votes";auto long_string=tgcx_BorrowString(text);
    tgcx_Expect(long_string.length==strlen(text) && long_string.capacity>15 && *reinterpret_cast<const char**>(long_string.data)==text,"native long-string ABI borrows stable caller storage");
    // Disabled callback must not mutate GUI or invoke an engine command.
    tgcx_bill_enabled=false;tgcx_bill_window=0;
    tgcx_BillOpenThunk();tgcx_BillCloseThunk();tgcx_BillCountThunk();
    tgcx_Expect(!tgcx_bill_open,"disabled native callbacks preserve fallback");
    printf("Bill: live-view model, promises, clipped purchases, invalid counters and borrowed strings checked\n");
}
