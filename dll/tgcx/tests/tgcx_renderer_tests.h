// Invoke the guarded engine entry in owned executable fixture memory with its native ESI ABI.
__declspec(naked) static void tgcx_TestNativePresent(uintptr_t) {
    __asm { push esi } __asm { mov esi,[esp+8] } __asm { mov eax,tgcx_base }
    __asm { add eax,0x59A520 } __asm { call eax } __asm { pop esi } __asm { ret }
}
// Invoke the post-Draw site with the renderer in EBX and entity in ESI, as FUN_00999D00 does.
__declspec(naked) static void tgcx_TestOrderedDraw(uintptr_t,uintptr_t) {
    __asm { push esi } __asm { push ebx }
    __asm { mov ebx,[esp+12] } __asm { mov esi,[esp+16] }
    __asm { mov eax,tgcx_base } __asm { add eax,0x599F22 } __asm { call eax }
    __asm { pop ebx } __asm { pop esi } __asm { ret }
}
static uintptr_t tgcx_test_ordered_entry=0;
__declspec(naked) static void tgcx_TestOrderedState(uintptr_t,uintptr_t) {
    __asm {
        push esi
        push ebx
        mov ebx,[esp+12]
        mov esi,[esp+16]
        mov eax,tgcx_base
        add eax,0x599F22
        mov tgcx_test_ordered_entry,eax
        fninit
        fld1
        pcmpeqd xmm0,xmm0
        fxsave tgcx_before_fx
        mov eax,12345678h
        mov ecx,23456789h
        mov edx,3456789Ah
        std
        call dword ptr [tgcx_test_ordered_entry]
        cld
        pop ebx
        pop esi
        fninit
        ret
    }
}
// Real D3D9 tests in an invisible owned window; no game process or user-input changes.
#pragma once
__declspec(naked) static void tgcx_FakeShow() {
    __asm { mov eax,[ecx+4Ch]
            mov byte ptr [eax+67h],1
            ret }
}
__declspec(naked) static void tgcx_FakeHide() {
    __asm { mov eax,[ecx+4Ch]
            mov byte ptr [eax+67h],0
            ret }
}
__declspec(naked) static void tgcx_FakeDestroy() {
    __asm { mov eax,ecx
            ret 4 }
}
__declspec(naked) static void tgcx_FakeTip() {
    __asm { mov eax,[esp+4]
            mov byte ptr [eax],1
            ret 8 }
}
static void tgcx_SaveSurface(IDirect3DSurface9* surface) {
    D3DSURFACE_DESC desc={};surface->GetDesc(&desc);
    D3DLOCKED_RECT locked={};
    if(FAILED(surface->LockRect(&locked,nullptr,D3DLOCK_READONLY))){tgcx_Expect(false,"readback lock");return;}
    BITMAPFILEHEADER file={};file.bfType=0x4D42;file.bfOffBits=sizeof(file)+sizeof(BITMAPINFOHEADER);
    file.bfSize=file.bfOffBits+desc.Width*desc.Height*4;
    BITMAPINFOHEADER info={};info.biSize=sizeof(info);info.biWidth=static_cast<LONG>(desc.Width);
    info.biHeight=-static_cast<LONG>(desc.Height);info.biPlanes=1;info.biBitCount=32;
    HANDLE output=CreateFileW(L"tgcx_render_fixture.bmp",GENERIC_WRITE,0,nullptr,CREATE_ALWAYS,0,nullptr);
    if(output!=INVALID_HANDLE_VALUE) {
        DWORD written=0;WriteFile(output,&file,sizeof(file),&written,nullptr);WriteFile(output,&info,sizeof(info),&written,nullptr);
        for(unsigned int row=0;row<desc.Height;++row)WriteFile(output,static_cast<char*>(locked.pBits)+row*locked.Pitch,desc.Width*4,&written,nullptr);
        CloseHandle(output);
    }
    surface->UnlockRect();
}
static void tgcx_RendererTests(const tgcx_Snapshot& live_sample) {
    tgcx_CpuScope cpu;
    tgcx_Snapshot sample=live_sample;
    unsigned int seats[9]={};
    tgcx_Expect(tgcx_AllocateSeats(sample,seats) && seats[0]==83 && seats[1]==5 && seats[5]==12,"largest-remainder allocation reproduces UK screenshots: 83/5/12");
    tgcx_Seat dots[100];tgcx_SeatGeometry(seats,dots);
    unsigned int actual[9]={};
    for(const auto& dot:dots) {
        tgcx_Expect(dot.ideology<9 && dot.x>=38 && dot.x<=160 && dot.y>=100 && dot.y<205,"all seven-ring dots bounded in chart area");
        ++actual[dot.ideology];
    }
    tgcx_Expect(memcmp(actual,seats,sizeof(seats))==0,"geometry carries exactly the allocated seats");
    // Main seating: Liberal (5) leftmost, Conservative (0) centre, Reactionary (1) rightmost, each one wedge.
    unsigned int leftmost=0,rightmost=0;
    for(unsigned int i=1;i<100;++i) {
        if(dots[i].x<dots[leftmost].x)leftmost=i;
        if(dots[i].x>dots[rightmost].x)rightmost=i;
    }
    tgcx_Expect(dots[leftmost].ideology==5 && dots[rightmost].ideology==1,"Main wedge order: Liberal left end, Reactionary right end");
    bool contiguous=true;
    for(unsigned int i=0;i<100;++i) for(unsigned int j=0;j<100;++j) {
        float ai=atan2f(fabsf(183.0f-dots[i].y),dots[i].x-99.0f),aj=atan2f(fabsf(183.0f-dots[j].y),dots[j].x-99.0f);
        unsigned int pi=0,pj=0;
        for(unsigned int k=0;k<9;++k){if(tgcx_seating_order[k]==dots[i].ideology)pi=k;if(tgcx_seating_order[k]==dots[j].ideology)pj=k;}
        if(ai>aj+1e-4f && pi>pj)contiguous=false;
    }
    tgcx_Expect(contiguous,"seats sweep left to right in political order (no interleaved parties)");
    sample.sum=0;tgcx_Expect(!tgcx_AllocateSeats(sample,seats),"zero total rejected without division");
    sample=live_sample;sample.shares_in_range=false;tgcx_Expect(!tgcx_AllocateSeats(sample,seats),"invalid shares retain original pie");
    sample=live_sample;
    // Known-vector tests use representative TGC colours, separately from the name/colour memory fixture.
    sample.colors[0]=0xFF0A0AFA;sample.colors[1]=0xFF14143C;sample.colors[5]=0xFFFafa05;

    // Source-owned PE import fixture drives the actual startup installation route.
    auto* dos=reinterpret_cast<IMAGE_DOS_HEADER*>(tgcx_base);dos->e_magic=IMAGE_DOS_SIGNATURE;dos->e_lfanew=0x100;
    auto* nt=reinterpret_cast<IMAGE_NT_HEADERS32*>(tgcx_base+0x100);nt->Signature=IMAGE_NT_SIGNATURE;
    nt->OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT]={0x1000,40};
    auto* imports=reinterpret_cast<IMAGE_IMPORT_DESCRIPTOR*>(tgcx_base+0x1000);
    imports[0].OriginalFirstThunk=0x1100;imports[0].FirstThunk=0x1200;imports[0].Name=0x1300;
    *reinterpret_cast<uint32_t*>(tgcx_base+0x1100)=0x1400;
    strcpy_s(reinterpret_cast<char*>(tgcx_base+0x1300),32,"d3d9.dll");
    strcpy_s(reinterpret_cast<char*>(tgcx_base+0x1402),32,"Direct3DCreate9");
    auto* iat=reinterpret_cast<uintptr_t*>(tgcx_base+0x1200);
    HMODULE graphics=LoadLibraryExW(L"d3d9.dll",nullptr,LOAD_LIBRARY_SEARCH_SYSTEM32);
    tgcx_Expect(graphics!=nullptr,"system D3D9 module loaded before IAT fixture");if(!graphics)return;
    *iat=reinterpret_cast<uintptr_t>(&tgcx_FakeShow);
    tgcx_Expect(!tgcx_InstallRenderer(),"foreign Direct3D IAT target rejected before Politics patches");
    *iat=reinterpret_cast<uintptr_t>(GetProcAddress(graphics,"Direct3DCreate9"));
    const uint32_t functions[]={0x2D2C10,0x2D2C40,0x2D2B00,0x2D2DD0};
    const uintptr_t targets[]={reinterpret_cast<uintptr_t>(&tgcx_FakeShow),reinterpret_cast<uintptr_t>(&tgcx_FakeHide),
        reinterpret_cast<uintptr_t>(&tgcx_FakeDestroy),reinterpret_cast<uintptr_t>(&tgcx_FakeTip)};
    for(unsigned int i=0;i<4;++i) {
        unsigned char* code=reinterpret_cast<unsigned char*>(tgcx_base+functions[i]);code[0]=0xE9;
        uint32_t displacement=static_cast<uint32_t>(targets[i]-(tgcx_base+functions[i]+5));memcpy(code+1,&displacement,4);
    }
    *reinterpret_cast<unsigned char*>(tgcx_base+0x599F22)^=1;
    tgcx_Expect(!tgcx_InstallRenderer() && *iat==reinterpret_cast<uintptr_t>(GetProcAddress(graphics,"Direct3DCreate9")) &&
        *reinterpret_cast<uintptr_t*>(tgcx_base+0xA0E460)==tgcx_base+0x2D2C10 &&
        *reinterpret_cast<unsigned char*>(tgcx_base+0x59A520)==0xFF,
        "changed ordered Draw bytes reject both native stages and roll back IAT/view hooks");
    *reinterpret_cast<unsigned char*>(tgcx_base+0x599F22)^=1;
    tgcx_Expect(tgcx_InstallRenderer() && *iat==reinterpret_cast<uintptr_t>(&tgcx_Create9),"guarded startup installs IAT, both native stages and three Politics lifecycle callbacks");

    HWND window=CreateWindowExW(0,L"STATIC",L"TGCX isolated renderer fixture",WS_OVERLAPPED,0,0,640,480,nullptr,nullptr,GetModuleHandleW(nullptr),nullptr);
    tgcx_Expect(window!=nullptr,"invisible test window created");if(!window)return;
    IDirect3D9* factory=reinterpret_cast<tgcx_Create9Fn>(*iat)(D3D_SDK_VERSION);
    tgcx_Expect(factory!=nullptr,"real Direct3D9 factory and guarded CreateDevice hook");
    if(!factory){DestroyWindow(window);return;}
    D3DPRESENT_PARAMETERS parameters={};parameters.Windowed=TRUE;parameters.SwapEffect=D3DSWAPEFFECT_DISCARD;
    parameters.BackBufferWidth=640;parameters.BackBufferHeight=480;parameters.BackBufferFormat=D3DFMT_X8R8G8B8;
    parameters.hDeviceWindow=window;parameters.PresentationInterval=D3DPRESENT_INTERVAL_IMMEDIATE;
    IDirect3DDevice9* device=nullptr;
    HRESULT result=factory->CreateDevice(D3DADAPTER_DEFAULT,D3DDEVTYPE_HAL,window,D3DCREATE_SOFTWARE_VERTEXPROCESSING,&parameters,&device);
    tgcx_Expect(SUCCEEDED(result) && device,"real D3D9 device created through hook");
    if(FAILED(result))printf("  CreateDevice HRESULT=%08X\n",static_cast<unsigned int>(result));
    if(!device){factory->Release();DestroyWindow(window);return;}
    tgcx_Expect(tgcx_HookDevice(device),"device hook is idempotent");
    unsigned int initial_frames=tgcx_present_count;
    for(unsigned int frame=0;frame<3;++frame) {
        device->BeginScene();device->EndScene();device->Present(nullptr,nullptr,nullptr,nullptr);
    }
    tgcx_Expect(tgcx_present_count==initial_frames,"COM presentation forwards without a duplicate native frame tick");
    IDirect3DVertexBuffer9* vertices=nullptr;
    device->CreateVertexBuffer(256,0,tgcx_fvf,D3DPOOL_DEFAULT,&vertices,nullptr);
    tgcx_Expect(vertices!=nullptr,"stream-state fixture buffer");
    device->SetFVF(tgcx_fvf);device->SetStreamSource(0,vertices,4,sizeof(tgcx_Vertex));
    device->SetRenderState(D3DRS_ALPHABLENDENABLE,FALSE);device->SetRenderState(D3DRS_SRCBLEND,D3DBLEND_ONE);
    device->SetTextureStageState(0,D3DTSS_TEXTURETRANSFORMFLAGS,D3DTTFF_COUNT2);
    D3DVIEWPORT9 viewport={0,0,640,480,0,1};device->SetViewport(&viewport);
    tgcx_GuiRect gui={30,60,640,480};
    device->Clear(0,nullptr,D3DCLEAR_TARGET,0xFF30352C,1,0);
    tgcx_Expect(SUCCEEDED(device->BeginScene()),"fixture BeginScene");
    tgcx_Expect(tgcx_DrawChart(device,sample,gui)==S_OK,"100-seat draw on real GPU");
    DWORD alpha=1,blend=0,transform=0;device->GetRenderState(D3DRS_ALPHABLENDENABLE,&alpha);device->GetRenderState(D3DRS_SRCBLEND,&blend);
    device->GetTextureStageState(0,D3DTSS_TEXTURETRANSFORMFLAGS,&transform);
    IDirect3DVertexBuffer9* restored=nullptr;UINT offset=0,stride=0;device->GetStreamSource(0,&restored,&offset,&stride);
    tgcx_Expect(alpha==FALSE && blend==D3DBLEND_ONE && transform==D3DTTFF_COUNT2,"render/stage state restored after draw");
    tgcx_Expect(restored==vertices && offset==4 && stride==sizeof(tgcx_Vertex),"DrawPrimitiveUP stream restored by state block");
    if(restored)restored->Release();
    tgcx_Expect(SUCCEEDED(device->EndScene()),"EndScene hook delegates original without nesting scenes");
    IDirect3DSurface9* target=nullptr;IDirect3DSurface9* surface=nullptr;
    device->GetRenderTarget(0,&target);
    device->CreateOffscreenPlainSurface(640,480,D3DFMT_X8R8G8B8,D3DPOOL_SYSTEMMEM,&surface,nullptr);
    tgcx_Expect(target && surface && SUCCEEDED(device->GetRenderTargetData(target,surface)),"real render-target readback");
    if(target && surface) {
        D3DLOCKED_RECT locked={};
        if(SUCCEEDED(surface->LockRect(&locked,nullptr,D3DLOCK_READONLY))) {
            unsigned int pixels[3]={};
            for(unsigned int y=0;y<480;++y)for(unsigned int x=0;x<220;++x) {
                DWORD color=reinterpret_cast<DWORD*>(static_cast<char*>(locked.pBits)+y*locked.Pitch)[x]&0xFFFFFF;
                if(color==0x0A0AFA)++pixels[0];if(color==0x14143C)++pixels[1];if(color==0xFAFA05)++pixels[2];
            }
            tgcx_Expect(pixels[0]>500 && pixels[1]>20 && pixels[2]>50,"all three UK seat colours occur in GPU pixels");
            auto pixel=[&locked](unsigned int x,unsigned int y) {
                return reinterpret_cast<DWORD*>(static_cast<char*>(locked.pBits)+y*locked.Pitch)[x]&0xFFFFFF;
            };
            DWORD fringe=pixel(68,170);
            tgcx_Expect(pixel(66,170)==0x30352C && pixel(100,170)==0xCBC9C1 &&
                ((fringe>>16)&255)>0x30 && ((fringe>>16)&255)<0xCB,
                "panel hides native pie in its core and blends its outer edge without leaking outside");
            surface->UnlockRect();tgcx_SaveSurface(surface);
        }
    }
    if(target)target->Release();if(surface)surface->Release();

    // Production callbacks receive a live synthetic Politics object and world. No original game memory is used.
    unsigned char view[0x60]={},root[0x2C4]={};uintptr_t root_table[18]={};
    unsigned char icon[0xA0]={},entity[0x100]={},names[28]={};uintptr_t objects[]={reinterpret_cast<uintptr_t>(icon)};
    unsigned char* renderer=static_cast<unsigned char*>(VirtualAlloc(nullptr,0x6BD00,MEM_RESERVE|MEM_COMMIT,PAGE_READWRITE));
    tgcx_Expect(renderer!=nullptr,"owned GUI renderer metadata");
    if(renderer) {
        *reinterpret_cast<uintptr_t*>(renderer)=tgcx_base+0xA19B8C;
        *reinterpret_cast<IDirect3DDevice9**>(renderer+0x64)=device;renderer[0x6BCA0]=1;
        // The original native entry has already passed its full byte guard; replace only its fixture continuation.
        *reinterpret_cast<unsigned char*>(tgcx_base+0x59A526)=0xC3;
        auto* continuation=reinterpret_cast<unsigned char*>(tgcx_base+0x599F29);continuation[0]=0xE9;
        uint32_t continuation_jump=static_cast<uint32_t>(reinterpret_cast<uintptr_t>(&tgcx_FakePump)-(tgcx_base+0x599F2E));
        memcpy(continuation+1,&continuation_jump,4);

        *reinterpret_cast<uintptr_t*>(view)=tgcx_base+0xA0E458;*reinterpret_cast<unsigned char**>(view+0x4C)=root;
        *reinterpret_cast<uintptr_t**>(root)=root_table;root_table[8]=tgcx_base+0x5E4450;root_table[9]=tgcx_base+0x5E4030;
        root_table[17]=tgcx_base+0x5E4D40;
        *reinterpret_cast<uintptr_t*>(icon)=tgcx_base+0xA3A930;
        *reinterpret_cast<uintptr_t*>(icon+0x94)=reinterpret_cast<uintptr_t>(entity);
        *reinterpret_cast<uintptr_t*>(entity)=tgcx_base+0xA3FF08;
        *reinterpret_cast<uintptr_t**>(root+0x2AC)=objects;*reinterpret_cast<uintptr_t**>(root+0x2B0)=objects+1;
        *reinterpret_cast<unsigned char**>(root+0x2BC)=names;*reinterpret_cast<unsigned char**>(root+0x2C0)=names+28;
        const char* overlay_name="chart_upper_house_overlay";
        *reinterpret_cast<const char**>(names)=overlay_name;
        *reinterpret_cast<uint32_t*>(names+16)=static_cast<uint32_t>(strlen(overlay_name));
        *reinterpret_cast<uint32_t*>(names+20)=31;
        *reinterpret_cast<unsigned char**>(root+0x90)=renderer;
        *reinterpret_cast<uint32_t*>(root+0x250)=30|(60u<<16);
        *reinterpret_cast<int*>(renderer+0x6B9BC)=640;*reinterpret_cast<int*>(renderer+0x6B9C0)=480;
        auto* view_table=reinterpret_cast<uintptr_t*>(tgcx_base+0xA0E458);
        auto show=reinterpret_cast<void(__fastcall*)(void*,void*)>(view_table[2]);
        auto hide=reinterpret_cast<void(__fastcall*)(void*,void*)>(view_table[4]);
        SetLastError(789);show(view,nullptr);
        tgcx_Expect(GetLastError()==789 && tgcx_politics==reinterpret_cast<uintptr_t>(view) && root[0x67]==1,"show thunk preserves ABI and original visibility");

        tgcx_Expect(view_table[10]==tgcx_base+0x2D2DD0,"native structured tooltip callback remains untouched (crash regression)");
        tgcx_GuiRect found={};tgcx_Expect(tgcx_ReadPoliticsRect(&found) && found.left==30 && found.top==60,"absolute GUI position matches live engine convention");
        unsigned int draws=tgcx_draw_count;
        device->BeginScene();device->EndScene();
        tgcx_Expect(tgcx_draw_count==draws,"intermediate EndScene cannot draw the chart");
        device->Clear(0,nullptr,D3DCLEAR_TARGET,0xFF000000,1.0f,0);
        D3DVIEWPORT9 narrow={11,17,180,140,0.0f,1.0f};device->SetViewport(&narrow);
        device->BeginScene();
        entity[0xF0]=1;SetLastError(654321);
        tgcx_TestOrderedState(reinterpret_cast<uintptr_t>(renderer),reinterpret_cast<uintptr_t>(entity));
        tgcx_Expect(tgcx_seen_eax==0x12345678 && tgcx_seen_ecx==0x23456789 && tgcx_seen_edx==0x3456789A,
            "ordered draw thunk preserves volatile registers around real D3D calls");
        tgcx_Expect((tgcx_seen_flags&0x441)==0x400,"original CMP flags replicated and caller DF restored after ordered draw");
        tgcx_Expect(memcmp(tgcx_before_fx,tgcx_seen_fx,32)==0 && memcmp(tgcx_before_fx+32,tgcx_seen_fx+32,256)==0,
            "ordered draw thunk preserves x87/SSE state around real D3D calls");
        tgcx_Expect(GetLastError()==654321,"ordered draw thunk preserves LastError");
        D3DVIEWPORT9 restored_view={};device->GetViewport(&restored_view);
        tgcx_Expect(tgcx_draw_count==draws+1 && tgcx_chart_drawn,"ordered native callback draws live upper house within the existing scene");
        tgcx_Expect(restored_view.X==11 && restored_view.Y==17 && restored_view.Width==180 && restored_view.Height==140,"non-fullscreen native viewport restored after fullscreen overlay");
        // A later opaque UI element overlaps a real seat. Final presentation must leave it above the chart.
        D3DRECT popup={110,170,150,200};device->SetViewport(&viewport);
        device->Clear(1,&popup,D3DCLEAR_TARGET,0xFF32EE51,1.0f,0);device->SetViewport(&narrow);
        device->EndScene();tgcx_TestNativePresent(reinterpret_cast<uintptr_t>(renderer));
        tgcx_Expect(tgcx_draw_count==draws+1,"final Present never redraws above later UI elements");
        tgcx_Expect(*reinterpret_cast<uint32_t*>(renderer+0x6BC8C)==1,"native thunk exactly replicates original renderer frame increment");
        IDirect3DSurface9* final_target=nullptr;IDirect3DSurface9* final_pixels=nullptr;
        device->GetRenderTarget(0,&final_target);device->CreateOffscreenPlainSurface(640,480,D3DFMT_X8R8G8B8,D3DPOOL_SYSTEMMEM,&final_pixels,nullptr);
        bool native_pixel=false,popup_pixel=false;
        if(final_target && final_pixels && SUCCEEDED(device->GetRenderTargetData(final_target,final_pixels))) {
            D3DLOCKED_RECT pixels={};if(SUCCEEDED(final_pixels->LockRect(&pixels,nullptr,D3DLOCK_READONLY))) {
                native_pixel=(reinterpret_cast<DWORD*>(static_cast<char*>(pixels.pBits)+243*pixels.Pitch)[71]&0xFFFFFF)==0xFF0000;
                popup_pixel=(reinterpret_cast<DWORD*>(static_cast<char*>(pixels.pBits)+185*pixels.Pitch)[129]&0xFFFFFF)==0x32EE51;
                final_pixels->UnlockRect();
            }
        }
        if(final_target)final_target->Release();if(final_pixels)final_pixels->Release();
        *reinterpret_cast<int*>(renderer+0x6B9BC)=2560;*reinterpret_cast<int*>(renderer+0x6B9C0)=1440;
        *reinterpret_cast<uint32_t*>(root+0x250)=768|(470u<<16);
        tgcx_GuiRect live_geometry={};
        tgcx_Expect(tgcx_ReadPoliticsRect(&live_geometry) && live_geometry.left==768 && live_geometry.top==470,"live 2560x1440 root coordinates are not centred twice");
        *reinterpret_cast<int*>(renderer+0x6B9BC)=640;*reinterpret_cast<int*>(renderer+0x6B9C0)=480;
        *reinterpret_cast<uint32_t*>(root+0x250)=30|(60u<<16);
        tgcx_Expect(native_pixel,"ordered chart produces GPU pixels outside inherited narrow viewport");
        tgcx_Expect(popup_pixel,"later UI pixels occlude seats and survive final Present (layering regression)");
        draws=tgcx_draw_count;device->BeginScene();
        tgcx_TestOrderedDraw(reinterpret_cast<uintptr_t>(renderer),reinterpret_cast<uintptr_t>(icon));
        *reinterpret_cast<uintptr_t*>(entity)^=4;
        tgcx_TestOrderedDraw(reinterpret_cast<uintptr_t>(renderer),reinterpret_cast<uintptr_t>(entity));
        *reinterpret_cast<uintptr_t*>(entity)^=4;
        *reinterpret_cast<uint32_t*>(names+16)=1;
        tgcx_TestOrderedDraw(reinterpret_cast<uintptr_t>(renderer),reinterpret_cast<uintptr_t>(entity));
        *reinterpret_cast<uint32_t*>(names+16)=static_cast<uint32_t>(strlen(overlay_name));
        *reinterpret_cast<unsigned char**>(root+0x2C0)=names+27;
        tgcx_TestOrderedDraw(reinterpret_cast<uintptr_t>(renderer),reinterpret_cast<uintptr_t>(entity));
        *reinterpret_cast<unsigned char**>(root+0x2C0)=names+28;device->EndScene();
        tgcx_Expect(tgcx_draw_count==draws,"foreign entity, vtable, name and malformed parallel vectors cannot trigger chart drawing");


        tgcx_last_sample=GetTickCount()-10001;tgcx_sampled=true;
        unsigned int frames=tgcx_present_count;tgcx_TestNativePresent(reinterpret_cast<uintptr_t>(renderer));device->Present(nullptr,nullptr,nullptr,nullptr);
        tgcx_Expect(tgcx_present_count==frames+1 && GetTickCount()-tgcx_last_sample<1000,"final engine Present samples while date remains paused");
        draws=tgcx_draw_count;hide(view,nullptr);device->BeginScene();
        tgcx_TestOrderedDraw(reinterpret_cast<uintptr_t>(renderer),reinterpret_cast<uintptr_t>(entity));
        device->EndScene();tgcx_TestNativePresent(reinterpret_cast<uintptr_t>(renderer));
        tgcx_Expect(!tgcx_politics && !tgcx_chart_drawn && tgcx_draw_count==draws,"closing Politics removes chart before next draw");
        root[0x67]=1;tgcx_politics=reinterpret_cast<uintptr_t>(view);root_table[8]^=1;
        tgcx_Expect(!tgcx_ReadPoliticsRect(&found),"unexpected GUI implementation rejected");
        auto destroy=reinterpret_cast<void*(__fastcall*)(void*,void*,unsigned int)>(view_table[1]);
        tgcx_Expect(destroy(view,nullptr,0)==view && !tgcx_politics && !tgcx_chart_drawn,"deleting-destructor thunk clears retained view and forwards stack argument");
        VirtualFree(renderer,0,MEM_RELEASE);
    }
    device->SetStreamSource(0,nullptr,0,0);if(vertices)vertices->Release();
    tgcx_Expect(SUCCEEDED(device->Reset(&parameters)),"device Reset succeeds with chart resources released");
    tgcx_Expect(!tgcx_chart_drawn,"Reset clears stale chart hit area");
    device->BeginScene();tgcx_Expect(tgcx_DrawChart(device,sample,gui)==S_OK,"chart renders after device Reset");device->EndScene();
    tgcx_Expect(device->Release()==0,"all device resources/references released");factory->Release();DestroyWindow(window);FreeLibrary(graphics);
    printf("renderer: real D3D9 pixels, 100 seats, state restoration, ordered layering, paused Present, visibility and Reset checked\n");
}
