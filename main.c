#include <lauxlib.h>
#include <lua.h>
#include <lualib.h>

extern unsigned char main_lua[];
extern unsigned int main_lua_size;

int main(void) {
  lua_State *L = luaL_newstate();
  luaL_openlibs(L);
  if (luaL_loadbuffer(L, (const char *)main_lua, main_lua_size, "main.lua") ||
      lua_pcall(L, 0, 0, 0)) {
    fprintf(stderr, "Error: %s\n", lua_tostring(L, -1));
    return 1;
  }
  lua_close(L);
  return 0;
}
