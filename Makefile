compile:
	rustc src/main.rs -o bin/baulu
test:
	./bin/baulu examples/hello.bul -log
compile-test:
	make compile
	make test