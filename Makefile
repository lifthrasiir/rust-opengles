CC ?= gcc
CXX ?= g++
CXXFLAGS ?=
AR ?= ar
DLLTOOL ?= dlltool

ANGLE_SRC = angle/src

.PHONY: all clean clean-all
all:
	@echo 'Use Cargo to build rust-opengles-angle.'
	@echo 'This Makefile is only used to build ANGLE. Use `${MAKE} angle` for that.'
clean:
clean-all: angle-clean

.PHONY: angle angle-clean angle-rust-deps
ifneq (,$(findstring MINGW,$(shell uname -s)))
angle: $(ANGLE_SRC)/libGLESv2.dll $(ANGLE_SRC)/libEGL.dll
angle-clean:
	cd $(ANGLE_SRC) && $(MAKE) clean
angle-rust-deps: $(OUT_DIR)/libGLESv2.a $(OUT_DIR)/libEGL.a
else
angle:
angle-clean:
angle-rust-deps:
endif

$(ANGLE_SRC)/libEGL.dll:
	cd $(ANGLE_SRC) && $(MAKE) libEGL.dll
$(ANGLE_SRC)/libGLESv2.dll:
	cd $(ANGLE_SRC) && $(MAKE) libGLESv2.dll

$(OUT_DIR)/libEGL.a: $(ANGLE_SRC)/libEGL/libEGL.def
	$(DLLTOOL) -d $< -l $@
$(OUT_DIR)/libGLESv2.a: $(ANGLE_SRC)/libGLESv2/libGLESv2.def
	$(DLLTOOL) -d $< -l $@

