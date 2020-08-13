#include <lua.h>
#include <lauxlib.h>
#include <lualib.h> 

#include <generated_engine.h>


static int wrapper_get_r(lua_State *L) {
    lua_getfield(L, 1, "ref");

    // convert the value of `ref` that is on the top of the stack
    // to a Transform reference of rust pointer
    Color* colors = (Color*) lua_touserdata(L, -1);
    
    uint8_t r = get_r(colors);
    lua_pushnumber(L, r);
    
    return 1;
}

void create_color_table_instance(lua_State *L, Color* colors) {
    lua_newtable(L);
    
    lua_pushstring(L, "ref");
    lua_pushlightuserdata(L, colors);
    lua_settable(L, -3);
    
    
    lua_pushstring(L, "get_r");
    lua_pushcfunction(L, wrapper_get_r);
    lua_settable(L, -3);
    
    lua_setglobal(L, "Color");
}
