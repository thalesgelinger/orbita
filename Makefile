build: 
	luajit -b main.lua orbite
	echo '#!/usr/bin/env luajit' | cat - orb > tmp && mv tmp orb
	chmod +x orb

clean:
	rm -f orb

