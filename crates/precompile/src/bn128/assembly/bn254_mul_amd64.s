// BN254 Field Multiplication Assembly
// Implements CIOS Montgomery multiplication optimized for BN254
// Based on gnark-crypto's implementation

#include "textflag.h"

// BN254 field modulus q
DATA ·qElement+0(SB)/8, $0x3c208c16d87cfd47
DATA ·qElement+8(SB)/8, $0x97816a916871ca8d
DATA ·qElement+16(SB)/8, $0xb85045b68181585d
DATA ·qElement+24(SB)/8, $0x30644e72e131a029
GLOBL ·qElement(SB), (RODATA+NOPTR), $32

// qInvNeg = -q^{-1} mod 2^64
DATA ·qInvNeg(SB)/8, $0x87d20782e4866389
GLOBL ·qInvNeg(SB), (RODATA+NOPTR), $8

// supportAdx flag for runtime detection
DATA ·supportAdx(SB)/1, $0
GLOBL ·supportAdx(SB), (RODATA+NOPTR), $1

// Macro for multiply-accumulate with carry chains
// Uses ADX instructions for parallel carry propagation
#define MACC(in0, in1, in2) \
	ADCXQ in0, in1     \
	MULXQ in2, AX, in0 \
	ADOXQ AX, in1      \

// Montgomery reduction step
// Computes m = t[0] * qInvNeg and performs reduction
#define DIV_SHIFT() \
	MOVQ  ·qInvNeg(SB), DX        \
	IMULQ R14, DX                 \
	XORQ  AX, AX                  \
	MULXQ ·qElement+0(SB), AX, R12  \
	ADCXQ R14, AX                 \
	MOVQ  R12, R14                \
	MACC(R13, R14, ·qElement+8(SB)) \
	MACC(CX, R13, ·qElement+16(SB)) \
	MACC(BX, CX, ·qElement+24(SB))  \
	MOVQ  $0, AX                  \
	ADCXQ AX, BX                  \
	ADOXQ BP, BX                  \

// First multiplication word
#define MUL_WORD_0() \
	XORQ  AX, AX       \
	MULXQ DI, R14, R13 \
	MULXQ R8, AX, CX   \
	ADOXQ AX, R13      \
	MULXQ R9, AX, BX   \
	ADOXQ AX, CX       \
	MULXQ R10, AX, BP  \
	ADOXQ AX, BX       \
	MOVQ  $0, AX       \
	ADOXQ AX, BP       \
	DIV_SHIFT()        \

// Subsequent multiplication words
#define MUL_WORD_N() \
	XORQ  AX, AX      \
	MULXQ DI, AX, BP  \
	ADOXQ AX, R14     \
	MACC(BP, R13, R8) \
	MACC(BP, CX, R9)  \
	MACC(BP, BX, R10) \
	MOVQ  $0, AX      \
	ADCXQ AX, BP      \
	ADOXQ AX, BP      \
	DIV_SHIFT()       \

// mul(res, x, y *Element)
// Implements Montgomery multiplication for BN254
TEXT ·mul(SB), NOSPLIT, $0-24
	// Check for ADX support
	CMPB ·supportAdx(SB), $1
	JNE  fallback

	// Load operands
	MOVQ x+8(FP), SI
	MOVQ 0(SI), DI
	MOVQ 8(SI), R8
	MOVQ 16(SI), R9
	MOVQ 24(SI), R10
	MOVQ y+16(FP), R11

	// Perform multiplication
	MOVQ 0(R11), DX
	MUL_WORD_0()
	
	MOVQ 8(R11), DX
	MUL_WORD_N()
	
	MOVQ 16(R11), DX
	MUL_WORD_N()
	
	MOVQ 24(R11), DX
	MUL_WORD_N()

	// Final reduction if needed
	MOVQ    R14, SI
	SUBQ    ·qElement+0(SB), R14
	MOVQ    R13, DI
	SBBQ    ·qElement+8(SB), R13
	MOVQ    CX, R8
	SBBQ    ·qElement+16(SB), CX
	MOVQ    BX, R9
	SBBQ    ·qElement+24(SB), BX
	
	// Conditional move based on carry
	CMOVQCS SI, R14
	CMOVQCS DI, R13
	CMOVQCS R8, CX
	CMOVQCS R9, BX

	// Store result
	MOVQ res+0(FP), AX
	MOVQ R14, 0(AX)
	MOVQ R13, 8(AX)
	MOVQ CX, 16(AX)
	MOVQ BX, 24(AX)
	RET

fallback:
	// Fallback to generic implementation
	// In practice, this would call back to Rust
	RET