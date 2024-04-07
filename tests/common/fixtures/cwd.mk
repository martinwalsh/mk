#| Print the current working directory
cwd:
	@echo "Called in: $(notdir $(CURDIR))"
	@echo "Called from: $(notdir $(MK_CWD))"
.PHONY: cwd
