.PHONY: all build_script move_target

all: build_script move_target

build_script:
	@echo "Building project.."
	cargo build

move_target:
	@echo "Moving target to root.."
	cp target/debug/domletters .

clean: 
	@echo "Cleaning up.."
	rm domletters