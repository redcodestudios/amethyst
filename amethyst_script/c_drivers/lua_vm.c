#include <lua.h>
#include <lauxlib.h>
#include <lualib.h> 

#include <custom_engine.h>
#include <generated_engine.h>

#include <stdio.h>

//extern void rust_log(char *s);

//void pwd() {
//    char cwd[1024];
//    getcwd(cwd, sizeof(cwd));
//    printf("Current working dir: %s\n", cwd);
//}

static int wrapper_log(lua_State *L) {
   char* m = lua_tostring(L, -1);
   rust_log(m);
   return 1;
}

void call_lua(const char* script, Transform* t) {
  //  pwd();
    rust_log("DEU BOM");

    //Transform* t = get_transform(components);
    //Transform* t = transform;
    //print_addr(t);
    //printf("VM SAYS: address is : %p\n", t);
    //printf("VM SAYS: transform y is %f\n", translation_y(t));
    move_up(t, 50);    
    //printf("VM SAYS: now transform y is %f\n", translation_y(t));
    
    lua_State *L;
    L = luaL_newstate();
    printf("C: loading lua script %s\n", script);
    luaL_openlibs(L);

    lua_pushcfunction(L, wrapper_log);
    lua_setglobal(L, "rust_log"); luaL_loadfile(L, script);

    if (lua_pcall(L, 0, 0, 0))
        printf("C: falhou: %s\n", lua_tostring(L, -1));
   
    //*transform = *t;
    lua_close(L);
}
