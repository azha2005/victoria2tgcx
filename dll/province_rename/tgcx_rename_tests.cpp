#define TGCX_RENAME_TEST
#include "tgcx_rename.cpp"
#include <stdlib.h>
#include <vector>

static unsigned tgcx_rename_failures=0,tgcx_rename_checks=0;
static void tgcx_RenameExpect(bool ok,const char* label) {
    ++tgcx_rename_checks;if(!ok){++tgcx_rename_failures;printf("FAIL %s\n",label);}
}
static unsigned char* tgcx_RenameMap(const wchar_t* path) {
    FILE* f=nullptr;if(_wfopen_s(&f,path,L"rb") || !f)return nullptr;
    fseek(f,0,SEEK_END);long size=ftell(f);rewind(f);std::vector<unsigned char> data(size);
    bool ok=fread(data.data(),1,size,f)==static_cast<size_t>(size);fclose(f);if(!ok)return nullptr;
    auto* nt=reinterpret_cast<IMAGE_NT_HEADERS32*>(data.data()+reinterpret_cast<IMAGE_DOS_HEADER*>(data.data())->e_lfanew);
    auto* result=static_cast<unsigned char*>(VirtualAlloc(nullptr,nt->OptionalHeader.SizeOfImage,MEM_COMMIT|MEM_RESERVE,PAGE_EXECUTE_READWRITE));
    if(!result)return nullptr;
    memcpy(result,data.data(),nt->OptionalHeader.SizeOfHeaders);
    auto* section=IMAGE_FIRST_SECTION(nt);
    for(unsigned i=0;i<nt->FileHeader.NumberOfSections;++i)
        memcpy(result+section[i].VirtualAddress,data.data()+section[i].PointerToRawData,section[i].SizeOfRawData);
    auto dir=nt->OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_BASERELOC];
    unsigned at=dir.VirtualAddress,end=at+dir.Size;
    while(at<end) {
        auto* block=reinterpret_cast<IMAGE_BASE_RELOCATION*>(result+at);
        if(block->SizeOfBlock<8)break;
        auto* rel=reinterpret_cast<WORD*>(result+at+8);
        for(unsigned i=0;i<(block->SizeOfBlock-8)/2;++i)
            if((rel[i]>>12)==IMAGE_REL_BASED_HIGHLOW)
                *reinterpret_cast<uint32_t*>(result+block->VirtualAddress+(rel[i]&4095))+=reinterpret_cast<uintptr_t>(result)-nt->OptionalHeader.ImageBase;
        at+=block->SizeOfBlock;
    }
    return result;
}
static void tgcx_RenameJump(unsigned rva,uintptr_t fn) {
    auto* p=reinterpret_cast<unsigned char*>(tgcx_rename_base+rva);p[0]=0xE9;
    *reinterpret_cast<uint32_t*>(p+1)=static_cast<uint32_t>(fn-(reinterpret_cast<uintptr_t>(p)+5));
}
static unsigned tgcx_rename_test_allocation=0,tgcx_rename_test_freed=0,tgcx_rename_test_posted=0,tgcx_rename_test_labels=0,tgcx_rename_test_map=0;
static void* tgcx_rename_test_command=nullptr;
static uintptr_t tgcx_rename_test_session=0;
static unsigned tgcx_rename_test_refreshes=0;
static void __stdcall tgcx_RenameTestRefresh(uintptr_t){++tgcx_rename_test_refreshes;}
static const char* tgcx_RenameRaw(const tgcx_RenameString* s) {return s->capacity>15?*reinterpret_cast<const char* const*>(s->data):s->data;}
static void tgcx_RenameTestAssign(tgcx_RenameString* dest,const char* text,unsigned count) {
    if(dest->capacity>15)free(*reinterpret_cast<void**>(dest->data));
    memset(dest,0,sizeof(*dest));dest->capacity=count<16?15:count;dest->length=count;
    char* target=dest->data;
    if(count>15){target=static_cast<char*>(malloc(count+1));*reinterpret_cast<char**>(dest->data)=target;}
    memcpy(target,text,count);target[count]=0;
}
static void* __cdecl tgcx_RenameTestNew(size_t size) {
    tgcx_rename_test_allocation=static_cast<unsigned>(size);auto* p=static_cast<unsigned char*>(calloc(1,size+16));
    memset(p+size,0xC5,16);return p;
}
static void __cdecl tgcx_RenameTestDelete(void* p){++tgcx_rename_test_freed;free(p);}
static uintptr_t __fastcall tgcx_RenameTestString(uintptr_t self,void*,const char* text,unsigned count) {
    tgcx_RenameTestAssign(reinterpret_cast<tgcx_RenameString*>(self),text,count);return self;
}
static uintptr_t __fastcall tgcx_RenameTestCopy(uintptr_t self,void*,const tgcx_RenameString* text,unsigned,unsigned) {
    tgcx_RenameTestAssign(reinterpret_cast<tgcx_RenameString*>(self),tgcx_RenameRaw(text),text->length);return self;
}
static uintptr_t __fastcall tgcx_RenameTestSession(uintptr_t,void*) {return tgcx_rename_test_session;}
static void __fastcall tgcx_RenameTestPost(uintptr_t,void*,void* command){++tgcx_rename_test_posted;tgcx_rename_test_command=command;}
static char tgcx_rename_test_name[81];
static void __stdcall tgcx_RenameTestLabel(uintptr_t,int id,tgcx_RenameString name) {
    ++tgcx_rename_test_labels;tgcx_RenameExpect(id==1,"native execute forwards province id to map label cache");
    strcpy_s(tgcx_rename_test_name,tgcx_RenameRaw(&name));
    if(name.capacity>15)free(*reinterpret_cast<void**>(name.data));
}
static void __stdcall tgcx_RenameTestMap(uintptr_t,int id,uintptr_t,float x,float y) {
    ++tgcx_rename_test_map;tgcx_RenameExpect(id==1 && x==12 && y==34,"native execute refreshes positioned map label");
}
static void __fastcall tgcx_RenameTestInsert(uintptr_t buffer,void*,const tgcx_RenameString* value) {
    tgcx_RenameTestAssign(reinterpret_cast<tgcx_RenameString*>(buffer+0x18),tgcx_RenameRaw(value),value->length);
}

static uintptr_t tgcx_rename_test_definition[1];
static unsigned tgcx_rename_test_created=0,tgcx_rename_test_wrong_type=0;
static uintptr_t tgcx_rename_test_windows=0,tgcx_rename_test_entry_type=0,tgcx_rename_test_window_type=0;
static unsigned tgcx_rename_test_global_button_lookup=0;
static uintptr_t __fastcall tgcx_RenameTestDefinition(uintptr_t registry,void*,const tgcx_RenameString* name) {
    if(registry==tgcx_rename_test_windows) {
        if(!strcmp(tgcx_RenameRaw(name),"tgcx_rename_entry"))return tgcx_rename_test_entry_type;
        if(!strcmp(tgcx_RenameRaw(name),"tgcx_rename_window"))return tgcx_rename_test_window_type;
        ++tgcx_rename_test_global_button_lookup;return 0;
    }
    return reinterpret_cast<uintptr_t>(tgcx_rename_test_definition);
}
static uintptr_t __fastcall tgcx_RenameTestCreate(uintptr_t self,void*,uintptr_t,uintptr_t,uintptr_t,uintptr_t,uintptr_t) {
    ++tgcx_rename_test_created;
    tgcx_RenameExpect(self==reinterpret_cast<uintptr_t>(tgcx_rename_test_definition),"native button dispatch preserves correct definition pointer");
    return 0x1234;
}
static uintptr_t __fastcall tgcx_RenameTestWrongType(uintptr_t self,void*,uintptr_t,uintptr_t,uintptr_t,uintptr_t,uintptr_t) {
    ++tgcx_rename_test_wrong_type;
    tgcx_RenameExpect(self==reinterpret_cast<uintptr_t>(tgcx_rename_test_definition)-0x64,"native collision thunk subtracts 0x64 before destructive dispatch");
    return 0x1234;
}
static void __fastcall tgcx_RenameTestRegister(uintptr_t,void*,uintptr_t) {}
// Actual button builder uses ESI for its context and consumes a by-value
// 28-byte string plus parent (RET 0x20). No executable entry point is run.
__declspec(naked) static uintptr_t __stdcall tgcx_RenameTestDispatch(uintptr_t,const tgcx_RenameString*) {
    __asm {push ebp} __asm {mov ebp,esp} __asm {push esi}
    __asm {mov esi,[ebp+8]} __asm {mov edx,[ebp+12]} __asm {push 0}
    __asm {push [edx+24]} __asm {push [edx+20]} __asm {push [edx+16]}
    __asm {push [edx+12]} __asm {push [edx+8]} __asm {push [edx+4]} __asm {push [edx]}
    __asm {mov eax,tgcx_rename_base} __asm {add eax,0x5CFF20} __asm {call eax}
    __asm {pop esi} __asm {pop ebp} __asm {ret 8}
}
static void tgcx_RenameSchemaTests() {
    static uintptr_t registry_vt[2],host_vt[17],host[1];
    static unsigned char dispatch[0x160],root[0x100],context[0x100],entry_type[0x160],window_type[0x160];
    auto put=[](void* p,unsigned at,uintptr_t value){*reinterpret_cast<uintptr_t*>(static_cast<char*>(p)+at)=value;};
    // Retain native slot addresses; replace only external boundary behavior.
    registry_vt[1]=tgcx_rename_base+0x5A29F0;
    tgcx_RenameJump(0x5A29F0,reinterpret_cast<uintptr_t>(tgcx_RenameTestDefinition));
    tgcx_RenameJump(0x5F0310,reinterpret_cast<uintptr_t>(tgcx_RenameTestCreate));
    tgcx_RenameJump(0x659540,reinterpret_cast<uintptr_t>(tgcx_RenameTestWrongType));
    host_vt[16]=reinterpret_cast<uintptr_t>(tgcx_RenameTestRegister);host[0]=reinterpret_cast<uintptr_t>(host_vt);
    put(dispatch,0x150,reinterpret_cast<uintptr_t>(registry_vt));put(dispatch,0x14C,reinterpret_cast<uintptr_t>(host));
    auto name=tgcx_RenameBorrow("rename_open");
    tgcx_rename_test_definition[0]=tgcx_rename_base+0xA3CFBC;
    tgcx_RenameExpect(tgcx_RenameTestDispatch(reinterpret_cast<uintptr_t>(dispatch),&name)==0x1234 && tgcx_rename_test_created==1 && !tgcx_rename_test_wrong_type,"real native builder resolves unique button name to button creation");
    tgcx_rename_test_definition[0]=tgcx_rename_base+0xA41D4C;
    tgcx_RenameExpect(tgcx_RenameTestDispatch(reinterpret_cast<uintptr_t>(dispatch),&name)==0x1234 && tgcx_rename_test_wrong_type==1,"old collision reaches actual native destructor thunk instead of creation");
    put(root,0x4C,reinterpret_cast<uintptr_t>(context));put(root,0x90,0x1234);
    put(context,0,tgcx_rename_base+0xA08A44);put(context,0x30,0x1234);put(context,0x34,reinterpret_cast<uintptr_t>(registry_vt));
    tgcx_rename_test_windows=reinterpret_cast<uintptr_t>(context+0x34);
    tgcx_rename_test_entry_type=reinterpret_cast<uintptr_t>(entry_type);tgcx_rename_test_window_type=reinterpret_cast<uintptr_t>(window_type);
    put(entry_type,0x150,reinterpret_cast<uintptr_t>(registry_vt));put(window_type,0x150,reinterpret_cast<uintptr_t>(registry_vt));
    tgcx_RenameExpect(!tgcx_RenameSchema(reinterpret_cast<uintptr_t>(root)),"runtime schema refuses collided definition before root construction");
    tgcx_rename_test_definition[0]=tgcx_rename_base+0xA3CFBC;
    tgcx_RenameExpect(tgcx_RenameSchema(reinterpret_cast<uintptr_t>(root)),"runtime schema accepts button definition type");
    tgcx_RenameExpect(!tgcx_rename_test_global_button_lookup,"preflight resolves button definitions within each owner; global registry contains windows only");
}

// Independently measured live registry: the engine's case-insensitive ternary
// trie wrapper at RVA241BA0, table A41E84, nodes value/character/low/high/equal.
// Execute actual lookup bytes; only CRT tolower is a test-process boundary.
struct tgcx_RenameTestNode {uintptr_t value;unsigned character;tgcx_RenameTestNode *low,*high,*equal;};
struct tgcx_RenameTestRegistry {uintptr_t table;tgcx_RenameTestNode* head;};
static tgcx_RenameTestNode tgcx_rename_test_nodes[512];
static unsigned tgcx_rename_test_nodes_used=0;
static void tgcx_RenameTestKey(tgcx_RenameTestRegistry* registry,const char* key,uintptr_t value) {
    auto** slot=&registry->head;
    while(*key) {
        unsigned ch=static_cast<unsigned char>(*key);
        if(!*slot){*slot=&tgcx_rename_test_nodes[tgcx_rename_test_nodes_used++];(*slot)->character=ch;}
        auto* node=*slot;
        if(ch<node->character)slot=&node->low;
        else if(ch>node->character)slot=&node->high;
        else if(!*++key){node->value=value;return;}
        else slot=&node->equal;
    }
}
static int __cdecl tgcx_RenameTestLower(int value) {return value>='A' && value<='Z'?value+32:value;}
static void tgcx_RenameRealRegistryTests() {
    static unsigned char context[0x100],root[0x100],entry[0x160],window[0x160];
    static uintptr_t buttons[3];
    auto put=[](void* p,unsigned at,uintptr_t value){*reinterpret_cast<uintptr_t*>(static_cast<char*>(p)+at)=value;};
    put(root,0x4C,reinterpret_cast<uintptr_t>(context));put(root,0x90,0x1234);
    put(context,0,tgcx_rename_base+0xA08A44);put(context,0x30,0x1234);
    auto* windows=reinterpret_cast<tgcx_RenameTestRegistry*>(context+0x34);
    auto* entry_buttons=reinterpret_cast<tgcx_RenameTestRegistry*>(entry+0x150);
    auto* body_buttons=reinterpret_cast<tgcx_RenameTestRegistry*>(window+0x150);
    windows->table=entry_buttons->table=body_buttons->table=tgcx_rename_base+0xA41E84;
    for(auto& button:buttons)button=tgcx_rename_base+0xA3CFBC; // All three live definitions measured this table.
    tgcx_RenameTestKey(windows,"tgcx_rename_entry",reinterpret_cast<uintptr_t>(entry));
    tgcx_RenameTestKey(windows,"tgcx_rename_window",reinterpret_cast<uintptr_t>(window));
    tgcx_RenameTestKey(entry_buttons,"tgcx_rename_open",reinterpret_cast<uintptr_t>(&buttons[0]));
    tgcx_RenameTestKey(body_buttons,"tgcx_rename_apply",reinterpret_cast<uintptr_t>(&buttons[1]));
    tgcx_RenameTestKey(body_buttons,"tgcx_rename_cancel",reinterpret_cast<uintptr_t>(&buttons[2]));
    tgcx_RenameJump(0x6AEDB6,reinterpret_cast<uintptr_t>(tgcx_RenameTestLower));
    tgcx_RenameExpect(tgcx_RenameSchema(reinterpret_cast<uintptr_t>(root)),"actual native registry resolves measured standard button definitions and passes preflight");
    tgcx_RenameExpect(tgcx_RenameLookup(reinterpret_cast<uintptr_t>(body_buttons),"TGCX_RENAME_CANCEL")==reinterpret_cast<uintptr_t>(&buttons[2]),"actual native registry preserves case-insensitive name semantics");
    buttons[1]=tgcx_rename_base+0xA41D4C;
    tgcx_RenameExpect(!tgcx_RenameSchema(reinterpret_cast<uintptr_t>(root)),"actual native registry rejects a colliding wrong-type Apply definition");buttons[1]=tgcx_rename_base+0xA3CFBC;
    auto* factory=reinterpret_cast<uintptr_t*>(tgcx_rename_base+0xA3CFE8);uintptr_t saved=*factory;*factory=tgcx_rename_base+0x65A430;
    tgcx_RenameExpect(!tgcx_RenameSchema(reinterpret_cast<uintptr_t>(root)),"correct type with a changed destructive factory slot is refused");*factory=saved;
}

static uintptr_t tgcx_rename_test_header=0,tgcx_rename_test_name_control=0;
static unsigned char tgcx_rename_test_font[0x500],tgcx_rename_test_w[0x824],tgcx_rename_test_i[0x824],tgcx_rename_test_accent[0x824];
static void tgcx_RenameFontTests() {
    auto put=[](void* p,unsigned at,uintptr_t value){*reinterpret_cast<uintptr_t*>(static_cast<char*>(p)+at)=value;};
    put(tgcx_rename_test_font,0,tgcx_rename_base+0xA19A8C);put(tgcx_rename_test_font,0x494,18);
    put(tgcx_rename_test_font,('W'+0x25)*4,reinterpret_cast<uintptr_t>(tgcx_rename_test_w));
    put(tgcx_rename_test_font,('i'+0x25)*4,reinterpret_cast<uintptr_t>(tgcx_rename_test_i));
    put(tgcx_rename_test_font,(0xED+0x25)*4,reinterpret_cast<uintptr_t>(tgcx_rename_test_accent));
    put(tgcx_rename_test_w,0x18,16);put(tgcx_rename_test_i,0x18,3);put(tgcx_rename_test_accent,0x18,6);
    put(tgcx_rename_test_w,0x81C,1);put(tgcx_rename_test_w,0x1C,'i');put(tgcx_rename_test_w,0x20,static_cast<uintptr_t>(-2));
    static unsigned char rendered[0x1C4];put(rendered,0x1BC,reinterpret_cast<uintptr_t>(tgcx_rename_test_font));
    auto text=tgcx_RenameBorrow("Wi\xED");memcpy(rendered+0x150,&text,sizeof(text));
    int width=0,height=0;
    tgcx_RenameExpect(tgcx_RenameNameMetrics(reinterpret_cast<uintptr_t>(rendered),&width,&height) && width==23 && height==18,"actual native font measures proportional widths, kerning and accented CP1252 glyphs");
    text=tgcx_RenameBorrow("WW");memcpy(rendered+0x150,&text,sizeof(text));
    tgcx_RenameExpect(tgcx_RenameNameMetrics(reinterpret_cast<uintptr_t>(rendered),&width,&height) && width==32,"ABC metric changes with actual displayed name instead of character count");
    put(rendered,0x1BC,0);
    tgcx_RenameExpect(!tgcx_RenameNameMetrics(reinterpret_cast<uintptr_t>(rendered),&width,&height),"missing loaded font refuses exact measurement safely");
}
static unsigned tgcx_rename_test_input_refreshes=0;
static int tgcx_rename_test_input_x=0,tgcx_rename_test_position[2]={},tgcx_rename_test_raster[2]={};
static char tgcx_rename_test_raster_text[1024];
static double __fastcall tgcx_RenameTestCaretTime(uintptr_t,void*) {return 0.0;}
static LONG CALLBACK tgcx_RenameTestFault(EXCEPTION_POINTERS* info) {
    printf("Fixture exception %08lX EIP=%08lX return RVA=%08lX ECX=%08lX\n",
        info->ExceptionRecord->ExceptionCode,info->ContextRecord->Eip,
        *reinterpret_cast<DWORD*>(info->ContextRecord->Esp)-static_cast<DWORD>(tgcx_rename_base),info->ContextRecord->Ecx);
    printf("checks=%u base=%08X ESP=%08lX EBP=%08lX\n",tgcx_rename_checks,static_cast<unsigned>(tgcx_rename_base),info->ContextRecord->Esp,info->ContextRecord->Ebp);
    for(int n=0;n<16;++n)printf("%08lX ",reinterpret_cast<DWORD*>(info->ContextRecord->Esp)[n]);printf("\n");
    fflush(stdout);ExitProcess(1);
}
static bool tgcx_RenameTestCaretUpdate(uintptr_t object) {
    reinterpret_cast<void(__fastcall*)(uintptr_t,void*,uintptr_t)>(tgcx_rename_base+0x5B6160)(object,nullptr,0);
    return true;
}
static void __stdcall tgcx_RenameTestInputRefresh(uintptr_t edit) {
    ++tgcx_rename_test_input_refreshes;tgcx_rename_test_input_x=*reinterpret_cast<int*>(edit+0x150);
}
static void __fastcall tgcx_RenameTestInputPosition(uintptr_t,void*,const int* point) {
    tgcx_rename_test_position[0]=point[0];tgcx_rename_test_position[1]=point[1];
}
static void __fastcall tgcx_RenameTestRaster(uintptr_t,void*,uintptr_t,const tgcx_RenameString* text,
    uintptr_t,uintptr_t,int,int,int x,int y,int,int) {
    tgcx_rename_test_raster[0]=x;tgcx_rename_test_raster[1]=y;
    strcpy_s(tgcx_rename_test_raster_text,tgcx_RenameRaw(text));
}
static bool tgcx_RenameTestCenter(uintptr_t edit) {
    int x=0,y=0;if(!tgcx_RenameInputInsets(edit,&x,&y))return false;
    tgcx_RenameInputPosition(edit,x,y);return true;
}
static void tgcx_RenameInputTests() {
    static unsigned char edit[0x1E0],other[0x1E0],render[0x310],line[32];
    auto put=[](void* p,unsigned at,uintptr_t value){*reinterpret_cast<uintptr_t*>(static_cast<char*>(p)+at)=value;};
    uintptr_t object=reinterpret_cast<uintptr_t>(edit),font=reinterpret_cast<uintptr_t>(tgcx_rename_test_font);
    typedef uintptr_t(__stdcall* BufferCtor)(uintptr_t,uintptr_t,uintptr_t);
    reinterpret_cast<BufferCtor>(tgcx_rename_base+0x5F4BC0)(object+0x54,0x12345678,font);
    tgcx_RenameExpect(*reinterpret_cast<uintptr_t*>(edit+0x54+0x78)==font && *reinterpret_cast<uintptr_t*>(edit+0x54+0x74)==0x12345678,"actual buffer constructor independently establishes font +78, separate from input context +74");
    put(edit,0,tgcx_rename_base+0xA39878);put(edit,0x54,tgcx_rename_base+0xA39938);
    put(edit,0x160,296|(30<<16));put(edit,0x9C,280|(18<<16));put(edit,0x54+0x38,0x10002);
    put(render,0,tgcx_rename_base+0xA3BB58);put(edit,0x110,reinterpret_cast<uintptr_t>(render));
    put(edit,0x15C,50|(70<<16));
    tgcx_RenameJump(0x61FEB0,reinterpret_cast<uintptr_t>(tgcx_RenameTestInputPosition));
    tgcx_RenameJump(0x5DD4D0,reinterpret_cast<uintptr_t>(tgcx_RenameTestRaster));
    tgcx_rename_panel={};tgcx_rename_panel.edit=object;tgcx_rename_panel.thread=GetCurrentThreadId();
    auto value=tgcx_RenameBorrow("WW");memcpy(edit+0x54+0x18,&value,sizeof(value));
    tgcx_RenameExpect(tgcx_RenameTestCenter(object) && *reinterpret_cast<int*>(edit+0x150)==132 && *reinterpret_cast<int*>(edit+0x154)==6,"centered input uses actual native font width and height");
    value=tgcx_RenameBorrow("Wi\xED");memcpy(edit+0x54+0x18,&value,sizeof(value));
    tgcx_RenameExpect(tgcx_RenameTestCenter(object) && *reinterpret_cast<int*>(edit+0x150)==136,"input recenters for proportional CP1252 text and kerning");
    char long_name[81];memset(long_name,'W',80);long_name[80]=0;value=tgcx_RenameBorrow(long_name);memcpy(edit+0x54+0x18,&value,sizeof(value));
    tgcx_RenameExpect(tgcx_RenameTestCenter(object) && *reinterpret_cast<int*>(edit+0x150)==8,"long names keep native minimum inset rather than a negative origin");
    tgcx_RenameExpect(*reinterpret_cast<unsigned*>(edit+0x9C)==(280|(18<<16)) && *reinterpret_cast<unsigned*>(edit+0x54+0x38)==0x10002,"centering preserves native buffer dimensions and cursor/selection state");
    value=tgcx_RenameBorrow("");memcpy(edit+0x54+0x18,&value,sizeof(value));
    tgcx_RenameExpect(tgcx_RenameTestCenter(object) && *reinterpret_cast<int*>(edit+0x150)==148,"empty field starts at its horizontal centre");
    memcpy(other,edit,sizeof(edit));put(other,0x150,17);
    tgcx_RenameExpect(!tgcx_RenameTestCenter(reinterpret_cast<uintptr_t>(other)) && *reinterpret_cast<int*>(other+0x150)==17,"other native edit controls are never changed");
    put(edit,0x54+0x78,0);
    tgcx_RenameExpect(!tgcx_RenameTestCenter(object) && *reinterpret_cast<int*>(edit+0x150)==148,"unknown input font refuses centering without guessed metrics");put(edit,0x54+0x78,font);
    tgcx_rename_edit_original=reinterpret_cast<uintptr_t>(tgcx_RenameTestInputRefresh);
    value=tgcx_RenameBorrow("WW");memcpy(edit+0x54+0x18,&value,sizeof(value));SetLastError(12345);
    reinterpret_cast<void(__stdcall*)(uintptr_t)>(tgcx_RenameEditRefreshHook)(object);
    tgcx_RenameExpect(tgcx_rename_test_input_refreshes==1 && tgcx_rename_test_input_x==0 && *reinterpret_cast<int*>(edit+0x150)==132 && GetLastError()==12345,"actual hook gives raster zero padding then restores centered caret inset and preserves last-error");
    reinterpret_cast<void(__stdcall*)(uintptr_t)>(tgcx_RenameEditRefreshHook)(reinterpret_cast<uintptr_t>(other));
    tgcx_RenameExpect(tgcx_rename_test_input_refreshes==2 && tgcx_rename_test_input_x==17,"refresh hook forwards unrelated controls unchanged");
    // Execute the real display refresh, native line/cursor getters, texture
    // setter and native packed position method. Only raster/GPU endpoints
    // are spies. 0.2.3's mock refresh missed this second inset consumer.
    using Refresh=void(__stdcall*)(uintptr_t);
    tgcx_rename_edit_original=tgcx_rename_base+0x5B6400;
    put(edit,0x54+0xA4,reinterpret_cast<uintptr_t>(line));
    put(edit,0x54+0xA8,reinterpret_cast<uintptr_t>(line+32));
    put(edit,0x54+0xAC,reinterpret_cast<uintptr_t>(line+32));
    edit[0x54+0xB4]=edit[0x54+0xB5]=0;put(edit,0x54+0x38,0);
    value=tgcx_RenameBorrow("WW");memcpy(line,&value,sizeof(value));
    put(edit,0x118+20,15);put(edit,0x134+20,15);put(edit,0x1A4+20,15);
    put(render,0x1BC+20,15);put(render,0x1D8+20,15);put(render,0x184+20,15);
    reinterpret_cast<Refresh>(tgcx_rename_edit_original)(object);
    tgcx_RenameInputPosition(object,132,6);
    tgcx_RenameExpect(tgcx_rename_test_raster[0]==132 && tgcx_rename_test_position[0]+tgcx_rename_test_raster[0]==314,"actual native refresh plus placement reproduces 0.2.3 doubled centering offset");
    const char* samples[]={"","i","Wi","WW","Wi\xED"};bool stable=true;
    for(const char* sample:samples) {
        value=tgcx_RenameBorrow(sample);memcpy(line,&value,sizeof(value));memcpy(edit+0x6C,&value,sizeof(value));
        int width=0,height=0;tgcx_RenameFontMetrics(font,sample,&width,&height);int wanted=(296-width)/2;
        reinterpret_cast<Refresh>(tgcx_RenameEditRefreshHook)(object);
        stable=stable && tgcx_rename_test_raster[0]==0 && tgcx_rename_test_raster[1]==0 &&
            tgcx_rename_test_position[0]==50+wanted && tgcx_rename_test_position[1]==76 &&
            !strcmp(tgcx_rename_test_raster_text,sample) && *reinterpret_cast<int*>(edit+0x150)==wanted;
        for(int frame=0;frame<4;++frame) {
            tgcx_RenameInputPosition(object,*reinterpret_cast<int*>(edit+0x150),*reinterpret_cast<int*>(edit+0x154));
            stable=stable && tgcx_rename_test_position[0]+tgcx_rename_test_raster[0]==50+wanted;
        }
    }
    tgcx_RenameExpect(stable,"actual native refresh and repeated parent placement apply one centered origin across typing and deletion");
    static unsigned char caret[0x310];static uintptr_t clock[1],clock_table[6];
    clock[0]=reinterpret_cast<uintptr_t>(clock_table);
    clock_table[5]=reinterpret_cast<uintptr_t>(tgcx_RenameTestCaretTime);
    put(edit,0x174,reinterpret_cast<uintptr_t>(clock));put(edit,0x114,reinterpret_cast<uintptr_t>(caret));
    put(caret,0,tgcx_rename_base+0xA3BB58);edit[0x4F]=edit[0x164]=1;
    put(edit,0x16C,0);put(edit,0x170,0);
    bool caret_aligned=true;
    for(unsigned row_height: {16u,18u}) {
        put(tgcx_rename_test_font,0x494,row_height);
        for(const char* sample:samples) {
            value=tgcx_RenameBorrow(sample);memcpy(edit+0x6C,&value,sizeof(value));
            int width=0,height=0;tgcx_RenameFontMetrics(font,sample,&width,&height);
            tgcx_RenameTestCenter(object);
            // Actual native getter's cached branch returns its measured X+3.
            // The underlying cursor indices and cache stay engine-owned.
            put(edit,0x54+0x38,static_cast<unsigned>(strlen(sample)));
            put(edit,0x54+0x82,static_cast<unsigned>(strlen(sample)));
            put(edit,0x54+0x7E,width+3);
            if(!tgcx_RenameTestCaretUpdate(object)) {caret_aligned=false;break;}
            int want_x=50+(296-width)/2+width+1;
            caret_aligned=caret_aligned && tgcx_rename_test_position[0]+7==want_x &&
                tgcx_rename_test_position[1]+6==77;
        }
    }
    tgcx_RenameExpect(caret_aligned,"actual native caret update and pixel getter align visible sprite ink after text and centre its 16px line for both font heights");
    put(tgcx_rename_test_font,0x494,18);
    tgcx_rename_panel={};tgcx_rename_edit_original=0;
}
static uintptr_t __fastcall tgcx_RenameTestFindHeader(uintptr_t,void*,const tgcx_RenameString* key) {
    return !strcmp(tgcx_RenameRaw(key),"province_view_header")?tgcx_rename_test_header:0;
}
static uintptr_t __fastcall tgcx_RenameTestFindName(uintptr_t host,void*,const tgcx_RenameString* key) {
    return host==tgcx_rename_test_header && !strcmp(tgcx_RenameRaw(key),"province_name")?tgcx_rename_test_name_control:0;
}
static int tgcx_rename_test_draw_x=0,tgcx_rename_test_draw_y=0;
static void __fastcall tgcx_RenameTestFontColor(uintptr_t,void*,uintptr_t) {}
static void __fastcall tgcx_RenameTestFontReset(uintptr_t,void*) {}
static void __fastcall tgcx_RenameTestFontDraw(uintptr_t,void*,uintptr_t,int,int,int x,int y,int,int,int,int,uintptr_t) {
    tgcx_rename_test_draw_x=x;tgcx_rename_test_draw_y=y;
}
static void tgcx_RenameTextDrawTests() {
    // Execute actual native Draw, intercepting only the GPU/font boundary.
    // This independently establishes the local text offsets missing in 0.2.0.
    static unsigned char text[0x200],state[0x40];static uintptr_t font[20];
    auto put=[](void* p,unsigned at,uintptr_t value){*reinterpret_cast<uintptr_t*>(static_cast<char*>(p)+at)=value;};
    font[8]=reinterpret_cast<uintptr_t>(tgcx_RenameTestFontDraw);
    font[15]=reinterpret_cast<uintptr_t>(tgcx_RenameTestFontColor);font[17]=reinterpret_cast<uintptr_t>(tgcx_RenameTestFontReset);
    uintptr_t font_object=reinterpret_cast<uintptr_t>(font);
    put(text,0x1BC,reinterpret_cast<uintptr_t>(&font_object));
    put(text,0x188,17);put(text,0x18C,45);put(text,0x1A0,300);put(text,0x1A4,16);
    typedef void(__fastcall* Draw)(uintptr_t,void*,uintptr_t,uintptr_t,uintptr_t);
    reinterpret_cast<Draw>(tgcx_rename_base+0x65EB00)(reinterpret_cast<uintptr_t>(text),nullptr,0,0,reinterpret_cast<uintptr_t>(state));
    tgcx_RenameExpect(tgcx_rename_test_draw_x==17 && tgcx_rename_test_draw_y==45,"actual native Draw passes local text offsets to font drawing despite an identity parent matrix");
}
static void tgcx_RenameAnchorTests() {
    static unsigned char root[0x500],header[0x500],name[0x120],definition[0xB0],renderer[0x6BA00];
    static unsigned char root_inner[0x110],header_inner[0x110],name_inner[0x1C4];
    auto put=[](void* p,unsigned at,uintptr_t value){*reinterpret_cast<uintptr_t*>(static_cast<char*>(p)+at)=value;};
    auto inner=[&](void* p,float x,float y,uintptr_t owner){put(p,0,tgcx_rename_base+(p==name_inner?0xA41F10:0xA38350));*reinterpret_cast<float*>(static_cast<char*>(p)+0x84)=x;*reinterpret_cast<float*>(static_cast<char*>(p)+0x88)=-y;put(p,0xE4,owner);};
    put(root,0,tgcx_rename_base+0xA3C0D8);put(header,0,tgcx_rename_base+0xA3C0D8);
    put(root,0x90,reinterpret_cast<uintptr_t>(renderer));put(root,0x94,reinterpret_cast<uintptr_t>(definition));
    put(root,0x494,reinterpret_cast<uintptr_t>(root_inner));put(name,0xA0,reinterpret_cast<uintptr_t>(name_inner));
    // Live CWindowType stores position at A0/A4 and size at A8/AC. Keep X=0
    // so the old guessed-width path fails rather than sharing its assumption.
    put(definition,0xA0,0);put(definition,0xA4,static_cast<uintptr_t>(-599));put(definition,0xA8,480);put(definition,0xAC,525);
    put(root,0x4A0,480);put(root,0x4A4,525);
    put(renderer,0x6B9BC,1280);put(renderer,0x6B9C0,1440);
    inner(root_inner,20,800,reinterpret_cast<uintptr_t>(renderer+0x6B9E0));
    inner(header_inner,7,5,reinterpret_cast<uintptr_t>(root_inner));inner(name_inner,0,0,reinterpret_cast<uintptr_t>(header_inner));
    put(name_inner,0x188,25);put(name_inner,0x18C,45);
    put(name_inner,0x1BC,reinterpret_cast<uintptr_t>(tgcx_rename_test_font));auto text=tgcx_RenameBorrow("WW");memcpy(name_inner+0x150,&text,sizeof(text));
    tgcx_rename_test_header=reinterpret_cast<uintptr_t>(header);tgcx_rename_test_name_control=reinterpret_cast<uintptr_t>(name);
    tgcx_RenameJump(0x5E4A20,reinterpret_cast<uintptr_t>(tgcx_RenameTestFindHeader));
    tgcx_RenameJump(0x5E4D00,reinterpret_cast<uintptr_t>(tgcx_RenameTestFindName));
    tgcx_RenamePlacement p={};
    tgcx_RenameExpect(tgcx_RenameAnchor(reinterpret_cast<uintptr_t>(root),&p) && p.entry_x==94 && p.entry_y==846&& p.panel_x==520 && p.panel_y==800,"universal ABC follows rendered name width, translated parents and live 480px host width");
    put(name_inner,0x188,35);put(name_inner,0x18C,55);
    tgcx_RenameExpect(tgcx_RenameAnchor(reinterpret_cast<uintptr_t>(root),&p) && p.entry_x==104 && p.entry_y==856,"mod-specific name position updates entry without fixed TGC coordinates");
    put(name_inner,0x1BC,0);bool measured=true;
    tgcx_RenameExpect(tgcx_RenameAnchor(reinterpret_cast<uintptr_t>(root),&p,&measured) && !measured && p.entry_x==472,"unsupported font retains a clickable ABC at native header right edge");
    inner(root_inner,0,841,reinterpret_cast<uintptr_t>(renderer+0x6B9E0));inner(header_inner,0,0,reinterpret_cast<uintptr_t>(root_inner));
    put(root,0x4A0,380);put(name_inner,0x188,17);put(name_inner,0x18C,45);
    put(name_inner,0x1BC,reinterpret_cast<uintptr_t>(tgcx_rename_test_font));
    tgcx_RenameExpect(tgcx_RenameAnchor(reinterpret_cast<uintptr_t>(root),&p) && p.entry_x==59 && p.entry_y==882&& p.panel_x==400 && p.panel_y==841,"live TGC geometry regression uses runtime 380px width and text-local17,45 rather than definition X and identity matrix");
    tgcx_rename_test_name_control=0;
    tgcx_RenameExpect(!tgcx_RenameAnchor(reinterpret_cast<uintptr_t>(root),&p),"missing native anchor refuses placement without guessing another base's layout");
    tgcx_RenameExpect(tgcx_RenameLayout(0,900,380,17,945,80,18,640,1440,&p) && p.panel_x==0 && p.panel_y==668,"narrow viewport places editor above host when the side lacks space");
    tgcx_RenameExpect(!tgcx_RenameLayout(0,0,380,17,45,80,18,320,200,&p),"viewport smaller than editor refuses offscreen placement");
}

static void tgcx_RenameNativeTests(const wchar_t* path) {
    auto* image=tgcx_RenameMap(path);tgcx_RenameExpect(image!=nullptr,"reference PE mapped without executing its entry point");if(!image)return;
    tgcx_rename_base=reinterpret_cast<uintptr_t>(image);
    tgcx_RenameExpect(tgcx_RenameGuards(),"78 guards match independently relocated reference image");
    unsigned native_command_size=image[0x1802BD+1];
    image[0x340050]^=1;tgcx_RenameExpect(!tgcx_RenameGuards(),"changed executable code refuses native guards");image[0x340050]^=1;
    tgcx_RenameJump(0x6AE9AF,reinterpret_cast<uintptr_t>(tgcx_RenameTestNew));
    tgcx_RenameJump(0x6AE91B,reinterpret_cast<uintptr_t>(tgcx_RenameTestDelete));
    tgcx_RenameJump(0x9350,reinterpret_cast<uintptr_t>(tgcx_RenameTestString));
    tgcx_RenameJump(0x8ED0,reinterpret_cast<uintptr_t>(tgcx_RenameTestCopy));
    tgcx_RenameJump(0x402800,reinterpret_cast<uintptr_t>(tgcx_RenameTestLabel));
    tgcx_RenameJump(0x3F18B0,reinterpret_cast<uintptr_t>(tgcx_RenameTestMap));
    FlushInstructionCache(GetCurrentProcess(),image,tgcx_rename_image_size);
    static unsigned char world[0xC00],view[0x30],province[0x200],map[0x180],country[0x30],session[0x2200],shape[0x100],marker[16];
    static uintptr_t countries[2],provinces[2],front[1],queue[1],front_vt[18],queue_vt[7],labels[2];
    auto put=[](void* p,unsigned offset,uintptr_t value){*reinterpret_cast<uintptr_t*>(static_cast<char*>(p)+offset)=value;};
    put(image,0xE588E8,reinterpret_cast<uintptr_t>(world));
    static uintptr_t manager[2];manager[0]=2;manager[1]=reinterpret_cast<uintptr_t>(countries);
    put(image,0xE587E4,reinterpret_cast<uintptr_t>(manager));countries[1]=reinterpret_cast<uintptr_t>(country);
    put(country,0x1C,0x415247);put(country,0x20,1);put(world,0xB60,1);put(world,0xB5C,0x415247);
    provinces[1]=reinterpret_cast<uintptr_t>(province);put(world,0xACC,reinterpret_cast<uintptr_t>(provinces));put(world,0xAD0,reinterpret_cast<uintptr_t>(provinces+2));
    put(view,0,tgcx_rename_base+0xA13768);put(view,0x24,reinterpret_cast<uintptr_t>(province));
    put(province,0,tgcx_rename_base+0x9FD280);put(province,0x58,1);put(province,0x5C,reinterpret_cast<uintptr_t>(map));put(province,0x12C,1);map[0x2A]=1;
    auto* stored=reinterpret_cast<tgcx_RenameString*>(map+0xE4);stored->capacity=15;tgcx_RenameTestAssign(stored,"Old name",8);
    front[0]=reinterpret_cast<uintptr_t>(front_vt);front_vt[14]=front_vt[17]=reinterpret_cast<uintptr_t>(tgcx_RenameTestSession);
    put(world,0xB24,reinterpret_cast<uintptr_t>(front));tgcx_rename_test_session=reinterpret_cast<uintptr_t>(session);
    queue[0]=reinterpret_cast<uintptr_t>(queue_vt);queue_vt[6]=reinterpret_cast<uintptr_t>(tgcx_RenameTestPost);put(session,0x38,reinterpret_cast<uintptr_t>(queue));
    put(session,0x20D8,reinterpret_cast<uintptr_t>(labels));labels[1]=reinterpret_cast<uintptr_t>(marker);put(marker,8,reinterpret_cast<uintptr_t>(shape));
    *reinterpret_cast<float*>(shape+0x6C)=12;*reinterpret_cast<float*>(shape+0x70)=34;
    tgcx_RenameTarget target={};tgcx_RenameExpect(tgcx_RenameSnapshot(reinterpret_cast<uintptr_t>(view),&target),"owned land province snapshot accepted");
    map[0x2A]=0;tgcx_RenameTarget bad={};tgcx_RenameExpect(!tgcx_RenameSnapshot(reinterpret_cast<uintptr_t>(view),&bad),"sea province excluded by native land flag");map[0x2A]=1;
    put(province,0x12C,0);tgcx_RenameExpect(!tgcx_RenameSend(reinterpret_cast<uintptr_t>(view),target,"New name") && !tgcx_rename_test_posted,"ownership loss refuses command before allocation/queue");put(province,0x12C,1);
    auto changed=target;changed.world++;
    tgcx_RenameExpect(!tgcx_RenameSend(reinterpret_cast<uintptr_t>(view),changed,"New name") && !tgcx_rename_test_posted,"world change cancels locked edit");
    static unsigned char old_root[0x100],new_root[0x100];
    put(old_root,0,tgcx_rename_base+0xA3C0D8);old_root[0x67]=1;put(view,0x20,reinterpret_cast<uintptr_t>(new_root));
    tgcx_rename_panel={};tgcx_rename_panel.view=reinterpret_cast<uintptr_t>(view);tgcx_rename_panel.root=reinterpret_cast<uintptr_t>(old_root);
    tgcx_rename_panel.thread=GetCurrentThreadId();tgcx_rename_panel.open=true;tgcx_rename_panel.target=target;
    tgcx_RenameClick(1);
    tgcx_RenameExpect(!tgcx_rename_panel.open && !tgcx_rename_test_posted,"Apply between root replacement and timer poll cancels stale editor before reading input");
    tgcx_rename_panel={};
    tgcx_RenameExpect(!tgcx_RenameSend(reinterpret_cast<uintptr_t>(view),target,"Bad\"name") && !tgcx_rename_test_posted,"unsafe text never enters command queue");
    const char* name="San Mart\xEDn de los Andes";
    tgcx_RenameExpect(tgcx_RenameSend(reinterpret_cast<uintptr_t>(view),target,name),"rename uses the native command constructor and queue");
    auto* command=static_cast<unsigned char*>(tgcx_rename_test_command);
    tgcx_RenameExpect(tgcx_rename_test_posted==1 && tgcx_rename_test_allocation==native_command_size,"one queued command matches allocation push in actual native clone (P45)");
    tgcx_RenameExpect(*reinterpret_cast<uintptr_t*>(command)==tgcx_rename_base+0xA00F1C && *reinterpret_cast<int*>(command+0x2C)==0x707 && *reinterpret_cast<int*>(command+0x30)==0x707,"actual constructor initializes rename command id and vtable");
    tgcx_RenameExpect(*reinterpret_cast<int*>(command+0x58)==1 && !strcmp(tgcx_RenameRaw(reinterpret_cast<tgcx_RenameString*>(command+0x3C)),name),"native command contains exact province and owned long accented text");
    bool canary=true;for(unsigned i=0;i<16;++i)if(command[0x5C+i]!=0xC5)canary=false;
    tgcx_RenameExpect(canary,"reference constructor respects 0x5C allocation boundary");
    reinterpret_cast<tgcx_RenameZero>(tgcx_rename_base+0x180040)(reinterpret_cast<uintptr_t>(command),nullptr);
    tgcx_RenameExpect(!strcmp(tgcx_RenameRaw(stored),name),"actual native execute and province setter update the map province name");
    tgcx_RenameExpect(tgcx_rename_test_labels==1 && tgcx_rename_test_map==1 && !strcmp(tgcx_rename_test_name,name),"actual native execute refreshes both map label paths");
    tgcx_rename_panel.pending=true;tgcx_rename_panel.queued_target=target;
    tgcx_rename_panel.view=reinterpret_cast<uintptr_t>(view);strcpy_s(tgcx_rename_panel.queued_name,name);
    tgcx_rename_update_original=reinterpret_cast<uintptr_t>(tgcx_RenameTestRefresh);
    tgcx_RenameConfirm(target);tgcx_RenameConfirm(target);
    tgcx_RenameExpect(tgcx_rename_test_refreshes==1 && !tgcx_rename_panel.pending,"applied name refreshes cached native header exactly once");
    tgcx_rename_panel.pending=true;tgcx_rename_panel.queued_target=changed;tgcx_RenameConfirm(target);
    tgcx_RenameExpect(tgcx_rename_test_refreshes==1 && !tgcx_rename_panel.pending,"stale world cannot trigger header refresh");
    tgcx_rename_panel={};tgcx_rename_update_original=0;
    tgcx_RenameString owned={};
    reinterpret_cast<tgcx_RenameOne>(tgcx_rename_base+0xAAf90)(reinterpret_cast<uintptr_t>(province),nullptr,&owned);
    tgcx_RenameExpect(!strcmp(tgcx_RenameRaw(&owned),name),"native province display getter returns the renamed text");
    if(owned.capacity>15)free(*reinterpret_cast<void**>(owned.data));
    typedef void(__fastcall* Destroy)(uintptr_t,void*,unsigned);
    reinterpret_cast<Destroy>(tgcx_rename_base+0x17FFD0)(reinterpret_cast<uintptr_t>(command),nullptr,1);
    tgcx_RenameExpect(tgcx_rename_test_freed==2,"native command destructor frees owned name and command");
    if(stored->capacity>15)free(*reinterpret_cast<void**>(stored->data));
    // Execute the real native edit SetText and GetText boundaries with a fake
    // font-dependent insert operation. SetText must replace, not append.
    static unsigned char buffer[0x100];static uintptr_t buffer_vt[22];
    put(buffer,0,reinterpret_cast<uintptr_t>(buffer_vt));buffer_vt[21]=reinterpret_cast<uintptr_t>(tgcx_RenameTestInsert);
    auto* raw=reinterpret_cast<tgcx_RenameString*>(buffer+0x18);raw->capacity=15;tgcx_RenameTestAssign(raw,"stale",5);
    auto input=tgcx_RenameBorrow("Replacement");
    reinterpret_cast<tgcx_RenameOne>(tgcx_rename_base+0x5F5290)(reinterpret_cast<uintptr_t>(buffer),nullptr,&input);
    owned={};reinterpret_cast<tgcx_RenameOne>(tgcx_rename_base+0x5F5950)(reinterpret_cast<uintptr_t>(buffer),nullptr,&owned);
    tgcx_RenameExpect(!strcmp(tgcx_RenameRaw(&owned),"Replacement"),"real edit-buffer SetText/GetText ABIs replace stale input");
    tgcx_RenameRealRegistryTests();tgcx_RenameSchemaTests();tgcx_RenameFontTests();tgcx_RenameInputTests();tgcx_RenameTextDrawTests();tgcx_RenameAnchorTests();
    VirtualFree(image,0,MEM_RELEASE);tgcx_rename_base=0;
}

int wmain(int argc,wchar_t** argv) {
    SetErrorMode(SEM_FAILCRITICALERRORS|SEM_NOGPFAULTERRORBOX|SEM_NOOPENFILEERRORBOX);
    AddVectoredExceptionHandler(1,tgcx_RenameTestFault);
    (void)&tgcx_RenameHash;(void)&tgcx_RenameDestroyHook;(void)&tgcx_RenameUpdateHook;
    (void)&tgcx_RenameRaw;
    char out[81];
    tgcx_RenameExpect(tgcx_RenameName("  New La Plata  ",16,out) && !strcmp(out,"New La Plata"),"surrounding spaces trimmed");
    tgcx_RenameExpect(!tgcx_RenameName("   ",3,out),"blank refused");
    tgcx_RenameExpect(!tgcx_RenameName("A\nB",3,out),"newlines refused");
    tgcx_RenameExpect(!tgcx_RenameName("A\\B",3,out),"backslashes refused");
    tgcx_RenameExpect(!tgcx_RenameName("A\xA7" "B",3,out),"formatting refused");
    char full[82];memset(full,'X',81);full[81]=0;
    tgcx_RenameExpect(tgcx_RenameName(full,80,out) && strlen(out)==80,"80-byte boundary retained exactly");
    tgcx_RenameExpect(!tgcx_RenameName(full,81,out),"81-byte input refused");
    tgcx_RenameExpect(!tgcx_RenameName("A\0B",3,out),"embedded NUL refused");
    if(argc!=2){printf("usage: tgcx_rename_tests <reference-exe>\n");return 2;}
    tgcx_RenameNativeTests(argv[1]);
    printf("Province Rename: %u checks, %u failures\n",tgcx_rename_checks,tgcx_rename_failures);
    return tgcx_rename_failures?1:0;
}
