###############################################################################
# COMMON HELPERS
###############################################################################
.DEFAULT_GOAL := help
MAKEFLAGS += --no-print-directory

###############################################################################
# LOGGING HELPERS
#

ifneq (,$(shell tput colors 2> /dev/null))
tput   := $(shell command -v tput 2>/dev/null)
bold   := $(shell $(tput) bold)
reset   := $(shell $(tput) sgr0)

red    := $(shell $(tput) setaf 1)
green  := $(shell $(tput) setaf 2)
yellow := $(shell $(tput) setaf 3)
cyan   := $(shell $(tput) setaf 6)
gray   := $(shell $(tput) setaf 248)
endif

# $(call dbg,<message>)
#
# Returns <message> wrapped in tput bold formatting, in the color gray.
define dbg
echo "$(bold)$(gray)""$(strip $(1))""$(reset)"
endef

# $(call log,<message>)
#
# Returns <message> wrapped in tput bold formatting.
define log
echo "$(bold)""$(strip $(1))""$(reset)"
endef

# $(call wrn,<message>)
#
# Returns <message> wrapped in tput bold formatting, in the color yellow.
define wrn
echo "$(bold)$(yellow)""$(strip $(1))""$(reset)"
endef

# $(call err,<message>)
#
# Returns <message> wrapped in tput bold formatting, in the color red.
define err
echo "$(bold)$(red)""$(strip $(1))""$(reset)"
endef

# $(call raise,<message>)
#
# Raises an error with the given <message> wrapped in tput bold formatting, in the color red.
define raise
$(error $(bold)$(red)$(strip $(1))$(reset))
endef


###############################################################################
# STRING HELPERS
#

# $(call lc,ALLUPPERCASE) -> alluppercase
lc = $(shell echo "$(1)" | tr '[:upper:]' '[:lower:]')

# $(call uc,alluppercase) -> ALLUPPERCASE
uc = $(shell echo "$(1)" | tr '[:lower:]' '[:upper:]'

###############################################################################
# PREREQUISITE HELPERS
#

# some_target: | _env_HOME
# 	@echo "do something"
#
# Prerequisite target that errors if the given environment variable is not
# defined in the current context, as named by the wildcard part of the target name.
_env_%:
	@_='$(or $($*),$(error `$*` is required))'


# joke: | _cmd_curl
# 	@curl -sSfL https://icanhazdadjoke.com/
# .PHONY: joke
#
# Prerequisite target that errors if the given program is not
# found on the system PATH, as named in the wildcard part of the target name.
_cmd_%:
	@_='$(or $(shell command -v $* 2>/dev/null),$(error `$*` command not found))'


###############################################################################
# PATH HELPERS
#

# $(call assert-path, some/path/to/a/folder/or/file, ERROR: Path does not exist)
#
# Callable that checks if a path exists, or performs an action (e.g. raise an error).
define assert-path
@_='$(or $(wildcard $(strip $(1))),$(call raise,$(or $(2),Path $(1) does not exist)))'
endef


###############################################################################
# OS/PLATFORM DETECTION
#

ifeq ($(OS),Windows_NT)
OS_NAME := windows
else
OS_NAME := $(call lc,$(shell uname -s))
endif

###############################################################################
# MISC HELPERS
#

# replace-text: | _cmd_sed
# 	$(call sed_i) 's/old/new/g' file.txt
#
# Callable that returns an appropriate `sed` in-place command for the current platform.
_gnu_sed := $(shell sed --version 2>/dev/null | grep -o 'GNU' | head -1)
sed_i := sed $(if $(filter $(call _gnu_sed),GNU),-i'',-i '')


# $(call with_dryrun, docker-compose push)
#
# Callable that prevents the execution of the given
# command when DR is set, by logging the command instead.
define with_dryrun
$(if $(DR),$(call wrn,[DRYRUN] WOULD RUN $(1)),$(1))
endef

# lint: | _fixme
# 	@echo "do something"
#
# Prerequisite target that errors if a FIXME comment is found anywhere in the source.
_fixme: _cmd_git
	@if git grep FIXME -- ':!\.makefiles/**'; then \
		$(call err,One or more FIXME comments are present in the source code.); \
		exit 1; \
	fi
.PHONY: _fixme

# lint: | _todo
# 	@echo "do something"
#
# Prerequisite target that errors if a TODO comment is found anywhere in the source.
_todo: _cmd_git
	@if git grep TODO -- ':!\.makefiles/**'; then \
		$(call err,One or more TODO comments are present in the source code.); \
		exit 1; \
	fi
.PHONY: _todo

###############################################################################
# HELP
#

# #| Print this helpful message
# help::
# 	@$(if $(shell command -v mk 2>/dev/null), mk --help)
# .PHONY: help


###############################################################################
# CLEAN
#

#| Remove all build artifacts
clean::
	@git clean -ffdx
.PHONY: clean
