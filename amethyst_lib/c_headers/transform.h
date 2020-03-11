#include <generated_engine.h>

Transform* push_transform(lua_State* ls) {
  if (!lua_checkstack(ls, 1)) {
      printf("o woe is me. no more room in hell...I mean stack...\n");return NULL;
    }
  Transform* dummy = lua_newuserdata(ls, sizeof(Transform));
  luaL_getmetatable(ls, "Transform");
  lua_setmetatable(ls, -2);
  lua_pushlughtuserdata(ls, dummy);
  lua_pushvalue(ls, -2);
  lua_settable(ls, LUA_REGISTRYINDEX);
  return dummy;
}

static Transform* pop_transform(lua_State* ls, int index) {
  Transform* dummy;
  dummy = luaL_checkudata(ls, index, "Transform");
  if (!dummy) printf("error:bad type, expected Transform\n");
  return dummy;
}

static int getter_translation_y(lua_State* ls) {
    Transform* t = pop_transform(ls, -1);
    lua_pushnumber(ls, translation_y(t));
    return 1;
}

static const luaL_Reg transform_methods[] = {
    {"new", new_transform},
    {"translation_y", getter_translation_y},
    {0, 0}
};

static const luaL_Reg transform_meta[] = {{0, 0}};

int transform_register(lua_State *ls) {
  lua_checkstack(ls, 4);
  lua_newtable(ls);
  luaL_setfuncs(ls, transform_methods, 0);
  luaL_newmetatable(ls, "Transform");
  luaL_setfuncs(ls, transform_methods, 0);
  luaL_setfuncs(ls, transform_meta, 0);
  lua_pushliteral(ls, "__index");
  lua_pushvalue(ls, -3);
  lua_rawset(ls, -3);
  lua_pushliteral(ls, "__metatable");
  lua_pushvalue(ls, -3);
  lua_rawset(ls, -3);
  lua_setglobal(ls, "Transform");
  return 0;
}
