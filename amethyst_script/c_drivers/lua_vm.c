#include <lua.h>
#include <lauxlib.h>
#include <lualib.h> 


#include <stdio.h>

//extern void rust_log(char *s);

//void pwd() {
//    char cwd[1024];
//    getcwd(cwd, sizeof(cwd));
//    printf("Current working dir: %s\n", cwd);
//}

//static int wrapper_log(lua_State *L) {
//   char* m = lua_tostring(L, -1);
//   rust_log(m);
//   return 1;
//}

void call_lua(const char* script) {
  //  pwd();
    //rust_log("DEU BOM PORRA");
    lua_State *L;
    L = luaL_newstate();
    printf("C: loading lua script %s\n", script);
    luaL_openlibs(L);
    luaL_loadfile(L, script);

    if (lua_pcall(L, 0, 0, 0))
        printf("C: falhou: %s\n", lua_tostring(L, -1));

    lua_close(L);
}
