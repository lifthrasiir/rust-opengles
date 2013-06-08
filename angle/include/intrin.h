// vvv rust-opengles-angle patch ahead
#ifndef _INTRIN_H
#define _INTRIN_H
#include <emmintrin.h>
static inline void __cpuid(int CPUInfo[4], int InfoType) {
	__asm__("cpuid"
		: "=a"(CPUInfo[0]), "=b"(CPUInfo[1]), "=c"(CPUInfo[2]), "=d"(CPUInfo[3])
		: "0"(InfoType));
}
#endif
// ^^^ rust-opengles-angle patch behind
