docker_image = docker_developer_environment

help:
	$(info -Targets -----------------------------------------------------------------------------)
	$(info -Development Targets -----------------------------------------------------------------)
	$(info lint                         | run lints with clippy)
	$(info benchmark                    | just for fun, really)
	$(info profile                      | only on linux - run callgrind and annotate it)
	$(info journey-tests                | run all stateless journey test)
	$(info continuous-journey-tests     | run all stateless journey test whenever something changes)
	$(info worst-case-scenario	    | run the code against the worst case scenario)
	$(info -- Use docker for all dependencies - run make interactively from there ----------------)
	$(info interactive-developer-environment-in-docker | gives you everything you need to run all targets)

always:

interactive-developer-environment-in-docker:
	docker build -t $(docker_image) - < etc/developer.Dockerfile
	docker run -v $$PWD:/volume -w /volume -it $(docker_image)

target/debug/codebreaker: always
	cargo build

target/release/codebreaker: always
	cargo build --release

lint:
	cargo clippy

profile: target/release/codebreaker
	valgrind --callgrind-out-file=callgrind.profile --tool=callgrind  $< < tests/fixtures/valid-medium.input >/dev/null
	callgrind_annotate --auto=yes callgrind.profile

benchmark: target/release/codebreaker
	hyperfine '$<' < tests/fixtures/valid.input

journey-tests: target/release/codebreaker
	./tests/stateless-journey.sh $<

continuous-journey-tests:
	watchexec $(MAKE) journey-tests

worst-case-scenario: target/release/codebreaker
	time $< < tests/fixtures/valid-massive.input > /dev/null

