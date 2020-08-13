#include <lua.h>
#include <lauxlib.h>
#include <lualib.h> 

/* #include <custom_engine.h> */
#include <generated_engine.h>
#include <color.h>

#include <stdio.h>

//extern void rust_log(char *s);

//void pwd() {
//    char cwd[1024];
//    getcwd(cwd, sizeof(cwd));
//    printf("Current working dir: %s\n", cwd);
//}

/* static int wrapper_log(lua_State *L) { */
/*    char* m = lua_tostring(L, -1); */
/*    rust_log(m); */
/*    return 1; */
/* } */

/* static int wrapper_moveup(lua_State *L) { */
/*     if (!lua_checkstack(L, 2)) { */
/*         fprintf(stderr, "wrong amount of arguments\n"); */
/*         return 0; */
/*     } */

/*     // pushes onto the top of the stack the value of the field `ref` of */
/*     // the table `self` that is the first arg of the function moveup */
/*     lua_getfield(L, 1, "ref"); */

/*     // convert the value of `ref` that is on the top of the stack */
/*     // to a Transform reference of rust pointer */
/*     Transform* t = (Transform*) lua_touserdata(L, -1); */
    
/*     double amount = lua_tonumber(L, 2); */
/*     move_up(t, (float) amount); */
/*     return 1; */
/* } */

/* void create_transform_table_instance(lua_State *L, Transform* t) { */
/*     lua_newtable(L); */
    
/*     lua_pushstring(L, "ref"); */
/*     lua_pushlightuserdata(L, t); */
/*     lua_settable(L, -3); */
    
    
/*     lua_pushstring(L, "move_up"); */
/*     lua_pushcfunction(L, wrapper_moveup); */
/*     lua_settable(L, -3); */
    
/*     lua_setglobal(L, "Transform"); */
/* } */

void C_call_lua_bytes(lua_State* state, const unsigned char* source, size_t size) { 
    if(luaL_loadbuffer(state, source, size, "script xaaab")){
        fprintf(stderr, "Lua `ERROR`: `%s\n`", lua_tostring(state, -1));
    }
    if(lua_pcall(state, 0, 0, 0)){
        fprintf(stderr, "Lua `ERROR`: `%s\n`", lua_tostring(state, -1));
    }
}

void C_setup_lua_script(lua_State* state, const unsigned char* source, size_t size, const char* script) {
    if (luaL_loadbuffer(state, source, size, script) || lua_pcall(state, 0, 0, 0)) {
        printf("error buffer: %s", lua_tostring(state, -1));
        return -1;
    }

    int status = lua_getglobal(state, "reads");
    

}

void C_run_lua_script(lua_State* state, const unsigned char* source, size_t size, const char* script) {
    if (luaL_loadbuffer(state, source, size, script) || lua_pcall(state, 0, 0, 0)) {
        printf("error buffer: %s", lua_tostring(state, -1));
        return -1;
    }
    int status = lua_getglobal(state, "run");

    // preparar parametros antes
    if(lua_pcall(state, 0, 1, 0) != 0) {
        printf("error running function `run`: %s\n", lua_tostring(state, -1));
    }
}

/* void call_lua(const char* script, Transform* t) { */
/*     rust_log("DEU BOM"); */

/*     lua_State *L; */
/*     L = luaL_newstate(); */
/*     printf("C: loading lua script %s\n", script); */
/*     luaL_openlibs(L); */

/*     lua_pushcfunction(L, wrapper_log); */
/*     lua_setglobal(L, "rust_log"); */

/*     create_transform_table_instance(L, t); */
    
/*     luaL_loadfile(L, script); */

/*     if (lua_pcall(L, 0, 0, 0)) */
/*         printf("C: falhou: %s\n", lua_tostring(L, -1)); */
    
/*     lua_close(L); */
/* } */

/* void call_on_start_lua(const char* script) { */
/*     lua_State* L; */
/*     L = luaL_newstate(); */
/*     luaL_openlibs(L); */

/*     luaL_dofile(L, script); */
     
/*     lua_getglobal(L, "start"); */

/*     if(lua_pcall(L, 0, 0, 0) != 0) { */
        /* printf("error running function `on_start`: %s\n", lua_tostring(L, -1)); */
    /* } */

    /* lua_close(L); */
/* } */
