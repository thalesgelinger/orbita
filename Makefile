build: 
	@luajit -b main.lua main.out
	@xxd -i main.out > main_lua.h
	@mv main.out build/
	@mv main_lua.h build/
	@gcc -o orb main.c -I/opt/homebrew/include/luajit-2.1 -L/opt/homebrew/lib -Wl,-search_paths_first -Wl,-rpath,/opt/homebrew/lib -lluajit-5.1 -lm -ldl -lreadline

clean:
	rm -f orb

