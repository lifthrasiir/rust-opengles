# Copyright (c) 2013 Kang Seonghoon. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

.PHONY: all clean

CXX ?= g++
CXXFLAGS ?= \
	-std=gnu++0x \
	-msse2 \
	-Os \
	-DANGLE_DISABLE_TRACE \
	-DANGLE_DISABLE_PERF \
	-DANGLE_COMPILE_OPTIMIZATION_LEVEL=D3DCOMPILE_OPTIMIZATION_LEVEL1 \
	-D'ANGLE_PRELOADED_D3DCOMPILER_MODULE_NAMES={ TEXT("d3dcompiler_46.dll"), TEXT("d3dcompiler_43.dll") }'
ifdef DIRECTX_SDK_INCLUDES
CXXFLAGS += -isystem $(DIRECTX_SDK_INCLUDES)
endif
LDFLAGS ?= \
	-shared \
	-L. \
	-Wl,--enable-stdcall-fixup

LIBGLESV2 = libGLESv2.dll
LIBGLESV2_A = libGLESv2.a
LIBEGL = libEGL.dll

WINDRES ?= windres

all: $(LIBGLESV2) $(LIBEGL)

SRC_PREPROCESSOR = \
	compiler/preprocessor/DiagnosticsBase.cpp \
	compiler/preprocessor/DirectiveHandlerBase.cpp \
	compiler/preprocessor/DirectiveParser.cpp \
	compiler/preprocessor/ExpressionParser.cpp \
	compiler/preprocessor/Input.cpp \
	compiler/preprocessor/Lexer.cpp \
	compiler/preprocessor/Macro.cpp \
	compiler/preprocessor/MacroExpander.cpp \
	compiler/preprocessor/Preprocessor.cpp \
	compiler/preprocessor/Token.cpp \
	compiler/preprocessor/Tokenizer.cpp
OBJ_PREPROCESSOR = $(patsubst %,%.o,$(SRC_PREPROCESSOR))
INC_PREPROCESSOR = \
	compiler/preprocessor/DiagnosticsBase.h \
	compiler/preprocessor/DirectiveHandlerBase.h \
	compiler/preprocessor/DirectiveParser.h \
	compiler/preprocessor/ExpressionParser.h \
	compiler/preprocessor/Input.h \
	compiler/preprocessor/length_limits.h \
	compiler/preprocessor/Lexer.h \
	compiler/preprocessor/Macro.h \
	compiler/preprocessor/MacroExpander.h \
	compiler/preprocessor/numeric_lex.h \
	compiler/preprocessor/pp_utils.h \
	compiler/preprocessor/Preprocessor.h \
	compiler/preprocessor/SourceLocation.h \
	compiler/preprocessor/Token.h \
	compiler/preprocessor/Tokenizer.h

define EACH_PREPROCESSOR
$(1).o: $(1) $$(INC_PREPROCESSOR)
	$$(CXX) $$(CXXFLAGS) $(1) -c -o $(1).o
endef
$(foreach file,$(SRC_PREPROCESSOR),$(eval $(call EACH_PREPROCESSOR,$(file))))

SRC_TRANSLATOR_COMMON = \
	common/debug.cpp \
	compiler/spooky.cpp \
	compiler/ossource_win.cpp \
	compiler/BuiltInFunctionEmulator.cpp \
	compiler/Compiler.cpp \
	compiler/DetectRecursion.cpp \
	compiler/Diagnostics.cpp \
	compiler/DirectiveHandler.cpp \
	compiler/ForLoopUnroll.cpp \
	compiler/glslang_lex.cpp \
	compiler/glslang_tab.cpp \
	compiler/InfoSink.cpp \
	compiler/Initialize.cpp \
	compiler/InitializeDll.cpp \
	compiler/InitializeParseContext.cpp \
	compiler/Intermediate.cpp \
	compiler/intermOut.cpp \
	compiler/IntermTraverse.cpp \
	compiler/MapLongVariableNames.cpp \
	compiler/parseConst.cpp \
	compiler/ParseHelper.cpp \
	compiler/PoolAlloc.cpp \
	compiler/QualifierAlive.cpp \
	compiler/RemoveTree.cpp \
	compiler/SymbolTable.cpp \
	compiler/util.cpp \
	compiler/ValidateLimitations.cpp \
	compiler/VariableInfo.cpp \
	compiler/VariablePacker.cpp \
	compiler/depgraph/DependencyGraph.cpp \
	compiler/depgraph/DependencyGraphBuilder.cpp \
	compiler/depgraph/DependencyGraphOutput.cpp \
	compiler/depgraph/DependencyGraphTraverse.cpp \
	compiler/timing/RestrictFragmentShaderTiming.cpp \
	compiler/timing/RestrictVertexShaderTiming.cpp \
	third_party/compiler/ArrayBoundsClamper.cpp
OBJ_TRANSLATOR_COMMON = $(patsubst %,%.o,$(SRC_TRANSLATOR_COMMON))
INC_TRANSLATOR_COMMON = \
	common/debug.h \
	compiler/spooky.h \
	compiler/BaseTypes.h \
	compiler/BuiltInFunctionEmulator.h \
	compiler/Common.h \
	compiler/ConstantUnion.h \
	compiler/DetectRecursion.h \
	compiler/Diagnostics.h \
	compiler/DirectiveHandler.h \
	compiler/ExtensionBehavior.h \
	compiler/ForLoopUnroll.h \
	compiler/glslang.h \
	compiler/glslang_tab.h \
	compiler/HashNames.h \
	compiler/InfoSink.h \
	compiler/Initialize.h \
	compiler/InitializeDll.h \
	compiler/InitializeGlobals.h \
	compiler/InitializeParseContext.h \
	compiler/intermediate.h \
	compiler/localintermediate.h \
	compiler/MapLongVariableNames.h \
	compiler/MMap.h \
	compiler/osinclude.h \
	compiler/ParseHelper.h \
	compiler/PoolAlloc.h \
	compiler/QualifierAlive.h \
	compiler/RemoveTree.h \
	compiler/RenameFunction.h \
	compiler/ShHandle.h \
	compiler/SymbolTable.h \
	compiler/Types.h \
	compiler/util.h \
	compiler/ValidateLimitations.h \
	compiler/VariableInfo.h \
	compiler/VariablePacker.h \
	compiler/depgraph/DependencyGraph.h \
	compiler/depgraph/DependencyGraphBuilder.h \
	compiler/depgraph/DependencyGraphOutput.h \
	compiler/timing/RestrictFragmentShaderTiming.h \
	compiler/timing/RestrictVertexShaderTiming.h \
	third_party/compiler/ArrayBoundsClamper.h

SRC_TRANSLATOR_GLSL = $(SRC_TRANSLATOR_COMMON) \
	compiler/CodeGenGLSL.cpp \
	compiler/OutputESSL.cpp \
	compiler/OutputGLSLBase.cpp \
	compiler/OutputGLSL.cpp \
	compiler/ShaderLang.cpp \
	compiler/TranslatorESSL.cpp \
	compiler/TranslatorGLSL.cpp \
	compiler/VersionGLSL.cpp
OBJ_TRANSLATOR_GLSL = $(patsubst %,%.o,$(SRC_TRANSLATOR_GLSL))
INC_TRANSLATOR_GLSL = $(INC_TRANSLATOR_COMMON) \
	compiler/OutputGLSLBase.h \
	compiler/OutputGLSL.h \
	compiler/TranslatorESSL.h \
	compiler/TranslatorGLSL.h \
	compiler/VersionGLSL.h \
	compiler/OutputESSL.h

SRC_TRANSLATOR_HLSL = $(SRC_TRANSLATOR_COMMON) \
	compiler/ShaderLang.cpp \
	compiler/DetectDiscontinuity.cpp \
	compiler/CodeGenHLSL.cpp \
	compiler/OutputHLSL.cpp \
	compiler/TranslatorHLSL.cpp \
	compiler/UnfoldShortCircuit.cpp \
	compiler/SearchSymbol.cpp
OBJ_TRANSLATOR_HLSL = $(patsubst %,%.o,$(SRC_TRANSLATOR_HLSL))
INC_TRANSLATOR_HLSL = $(INC_TRANSLATOR_COMMON) \
	compiler/DetectDiscontinuity.h \
	compiler/OutputHLSL.h \
	compiler/TranslatorHLSL.h \
	compiler/UnfoldShortCircuit.h \
	compiler/SearchSymbol.h

define EACH_TRANSLATOR_HLSL
$(1).o: $(1) $$(INC_PREPROCESSOR) $$(INC_TRANSLATOR_HLSL)
	$$(CXX) $$(CXXFLAGS) -I. -I../include -I../include/KHR -DCOMPILER_IMPLEMENTATION $$< -c -o $$@
endef
$(foreach file,$(SRC_TRANSLATOR_HLSL),$(eval $(call EACH_TRANSLATOR_HLSL,$(file))))

SRC_LIBGLESV2 = \
	common/RefCountObject.cpp \
	libGLESv2/IndexDataManager.cpp \
	libGLESv2/VertexDataManager.cpp \
	libGLESv2/Blit.cpp \
	libGLESv2/Buffer.cpp \
	libGLESv2/Context.cpp \
	libGLESv2/D3DConstantTable.cpp \
	libGLESv2/Fence.cpp \
	libGLESv2/Float16ToFloat32.cpp \
	libGLESv2/Framebuffer.cpp \
	libGLESv2/HandleAllocator.cpp \
	libGLESv2/libGLESv2.cpp \
	libGLESv2/main.cpp \
	libGLESv2/Program.cpp \
	libGLESv2/ProgramBinary.cpp \
	libGLESv2/Query.cpp \
	libGLESv2/Renderbuffer.cpp \
	libGLESv2/ResourceManager.cpp \
	libGLESv2/Shader.cpp \
	libGLESv2/Texture.cpp \
	libGLESv2/TextureSSE2.cpp \
	libGLESv2/utilities.cpp
OBJ_LIBGLESV2 = $(patsubst %,%.o,$(SRC_LIBGLESV2))
INC_LIBGLESV2 = \
	common/angleutils.h \
	common/RefCountObject.h \
	common/version.h \
	libGLESv2/IndexDataManager.h \
	libGLESv2/vertexconversion.h \
	libGLESv2/VertexDataManager.h \
	libGLESv2/BinaryStream.h \
	libGLESv2/Blit.h \
	libGLESv2/Buffer.h \
	libGLESv2/Context.h \
	libGLESv2/D3DConstantTable.h \
	libGLESv2/Fence.h \
	libGLESv2/Framebuffer.h \
	libGLESv2/HandleAllocator.h \
	libGLESv2/main.h \
	libGLESv2/mathutil.h \
	libGLESv2/Program.h \
	libGLESv2/ProgramBinary.h \
	libGLESv2/Query.h \
	libGLESv2/Renderbuffer.h \
	libGLESv2/ResourceManager.h \
	libGLESv2/Shader.h \
	libGLESv2/Texture.h \
	libGLESv2/utilities.h
RC_LIBGLESV2 = libGLESv2/libGLESv2.rc
DEF_LIBGLESV2 = libGLESv2/libGLESv2.def
RES_LIBGLESV2 = libGLESv2/libGLESv2.res

define EACH_LIBGLESV2
$(1).o: $(1) $$(INC_PREPROCESSOR) $$(INC_TRANSLATOR_HLSL) $$(INC_LIBGLESV2)
	$$(CXX) $$(CXXFLAGS) -I. -I../include -I../include/KHR -DCOMPONENT_BUILD $$< -c -o $$@
endef
$(foreach file,$(SRC_LIBGLESV2),$(eval $(call EACH_LIBGLESV2,$(file))))

$(RES_LIBGLESV2): $(RC_LIBGLESV2)
	$(WINDRES) -O coff $< $@

$(LIBGLESV2) $(LIBGLESV2_A): $(OBJ_PREPROCESSOR) $(OBJ_TRANSLATOR_HLSL) $(OBJ_LIBGLESV2) $(RES_LIBGLESV2) $(DEF_LIBGLESV2)
	$(CXX) $(LDFLAGS) $(OBJ_PREPROCESSOR) $(OBJ_TRANSLATOR_HLSL) $(OBJ_LIBGLESV2) $(RES_LIBGLESV2) $(DEF_LIBGLESV2) -ld3d9 -o $(LIBGLESV2) -Wl,--out-implib,$(LIBGLESV2_A)

SRC_LIBEGL = \
	libEGL/Config.cpp \
	libEGL/Display.cpp \
	libEGL/libEGL.cpp \
	libEGL/main.cpp \
	libEGL/Surface.cpp
OBJ_LIBEGL = $(patsubst %,%.o,$(SRC_LIBEGL))
INC_LIBEGL = \
	common/angleutils.h \
	common/version.h \
	libEGL/Config.h \
	libEGL/Display.h \
	libEGL/main.h \
	libEGL/Surface.h
RC_LIBEGL = libEGL/libEGL.rc
RES_LIBEGL = libEGL/libEGL.res
DEF_LIBEGL = libEGL/libEGL.def

define EACH_LIBEGL
$(1).o: $(1) $$(INC_PREPROCESSOR) $$(INC_TRANSLATOR_HLSL) $$(INC_LIBEGL)
	$$(CXX) $$(CXXFLAGS) -I. -I../include -I../include/KHR -DCOMPONENT_BUILD $$< -c -o $$@
endef
$(foreach file,$(SRC_LIBEGL),$(eval $(call EACH_LIBEGL,$(file))))

$(RES_LIBEGL): $(RC_LIBEGL)
	$(WINDRES) -O coff $< $@

$(LIBEGL): $(OBJ_PREPROCESSOR) $(OBJ_TRANSLATOR_HLSL) $(OBJ_LIBEGL) $(RES_LIBEGL) $(DEF_LIBEGL) $(LIBGLESV2_A)
	$(CXX) $(LDFLAGS) $(OBJ_PREPROCESSOR) $(OBJ_TRANSLATOR_HLSL) $(OBJ_LIBEGL) $(RES_LIBEGL) $(DEF_LIBEGL) -ld3d9 -ldxguid $(LIBGLESV2_A) -o $(LIBEGL)

clean:
	rm -f $(OBJ_PREPROCESSOR) $(OBJ_TRANSLATOR_COMMON) $(OBJ_TRANSLATOR_GLSL) $(OBJ_TRANSLATOR_HLSL) $(OBJ_LIBGLESV2) $(OBJ_LIBEGL) $(RES_LIBGLESV2) $(RES_LIBEGL) $(LIBGLESV2) $(LIBGLESV2_A) $(LIBEGL)

