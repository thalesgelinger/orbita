#include "build/main_lua.h" // AUTOMATICALLY GENERATED
#include <lauxlib.h>
#include <lua.h>
#include <lualib.h>
#include <stdio.h>

extern unsigned char main_out[];
extern unsigned int main_out_len;

int main(int argc, char **argv) {
  lua_State *L = luaL_newstate();
  luaL_openlibs(L);

  lua_newtable(L);

  for (int i = 1; i < argc; i++) {
    lua_pushstring(L, argv[i]);
    lua_rawseti(L, -2, i);
  }
  lua_setglobal(L, "arg");

  if (luaL_loadbuffer(L, (const char *)main_out, main_out_len, "main") ||
      lua_pcall(L, 0, 0, 0)) {
    fprintf(stderr, "Error: %s\n", lua_tostring(L, -1));
    lua_close(L);
    return 1;
  }

  lua_close(L);
  return 0;
}

