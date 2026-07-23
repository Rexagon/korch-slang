#ifndef SLANG_H
#define SLANG_H

#ifdef SLANG_USER_CONFIG
    #include SLANG_USER_CONFIG
#endif

/** \file slang.h

The Slang API provides services to compile, reflect, and specialize code
written in the Slang shading language.
*/

/*
The following section attempts to detect the compiler and version in use.

If an application defines `SLANG_COMPILER` before including this header,
they take responsibility for setting any compiler-dependent macros
used later in the file.

Most applications should not need to touch this section.
*/
#ifndef SLANG_COMPILER
    #define SLANG_COMPILER

    /*
    Compiler defines, see http://sourceforge.net/p/predef/wiki/Compilers/
    NOTE that SLANG_VC holds the compiler version - not just 1 or 0
    */
    #if defined(_MSC_VER)
        #if _MSC_VER >= 1900
            #define SLANG_VC 14
        #elif _MSC_VER >= 1800
            #define SLANG_VC 12
        #elif _MSC_VER >= 1700
            #define SLANG_VC 11
        #elif _MSC_VER >= 1600
            #define SLANG_VC 10
        #elif _MSC_VER >= 1500
            #define SLANG_VC 9
        #else
            #error "unknown version of Visual C++ compiler"
        #endif
    #elif defined(__clang__)
        #define SLANG_CLANG 1
    #elif defined(__SNC__)
        #define SLANG_SNC 1
    #elif defined(__ghs__)
        #define SLANG_GHS 1
    #elif defined(__GNUC__) /* note: __clang__, __SNC__, or __ghs__ imply __GNUC__ */
        #define SLANG_GCC 1
    #else
        #error "unknown compiler"
    #endif
    /*
    Any compilers not detected by the above logic are now now explicitly zeroed out.
    */
    #ifndef SLANG_VC
        #define SLANG_VC 0
    #endif
    #ifndef SLANG_CLANG
        #define SLANG_CLANG 0
    #endif
    #ifndef SLANG_SNC
        #define SLANG_SNC 0
    #endif
    #ifndef SLANG_GHS
        #define SLANG_GHS 0
    #endif
    #ifndef SLANG_GCC
        #define SLANG_GCC 0
    #endif
#endif /* SLANG_COMPILER */

/*
The following section attempts to detect the target platform being compiled for.

If an application defines `SLANG_PLATFORM` before including this header,
they take responsibility for setting any compiler-dependent macros
used later in the file.

Most applications should not need to touch this section.
*/
#ifndef SLANG_PLATFORM
    #define SLANG_PLATFORM
    /**
    Operating system defines, see http://sourceforge.net/p/predef/wiki/OperatingSystems/
    */
    #if defined(WINAPI_FAMILY) && WINAPI_FAMILY == WINAPI_PARTITION_APP
        #define SLANG_WINRT 1 /* Windows Runtime, either on Windows RT or Windows 8 */
    #elif defined(XBOXONE)
        #define SLANG_XBOXONE 1
    #elif defined(_WIN64) /* note: XBOXONE implies _WIN64 */
        #define SLANG_WIN64 1
    #elif defined(_M_PPC)
        #define SLANG_X360 1
    #elif defined(_WIN32) /* note: _M_PPC implies _WIN32 */
        #define SLANG_WIN32 1
    #elif defined(__ANDROID__)
        #define SLANG_ANDROID 1
    #elif defined(__linux__) || defined(__CYGWIN__) /* note: __ANDROID__ implies __linux__ */
        #define SLANG_LINUX 1
    #elif defined(__APPLE__)
        #include "TargetConditionals.h"
        #if TARGET_OS_MAC
            #define SLANG_OSX 1
        #else
            #define SLANG_IOS 1
        #endif
    #elif defined(__CELLOS_LV2__)
        #define SLANG_PS3 1
    #elif defined(__ORBIS__)
        #define SLANG_PS4 1
    #elif defined(__SNC__) && defined(__arm__)
        #define SLANG_PSP2 1
    #elif defined(__ghs__)
        #define SLANG_WIIU 1
    #elif defined(__EMSCRIPTEN__)
        #define SLANG_WASM 1
    #else
        #error "unknown target platform"
    #endif
    /*
    Any platforms not detected by the above logic are now now explicitly zeroed out.
    */
    #ifndef SLANG_WINRT
        #define SLANG_WINRT 0
    #endif
    #ifndef SLANG_XBOXONE
        #define SLANG_XBOXONE 0
    #endif
    #ifndef SLANG_WIN64
        #define SLANG_WIN64 0
    #endif
    #ifndef SLANG_X360
        #define SLANG_X360 0
    #endif
    #ifndef SLANG_WIN32
        #define SLANG_WIN32 0
    #endif
    #ifndef SLANG_ANDROID
        #define SLANG_ANDROID 0
    #endif
    #ifndef SLANG_LINUX
        #define SLANG_LINUX 0
    #endif
    #ifndef SLANG_IOS
        #define SLANG_IOS 0
    #endif
    #ifndef SLANG_OSX
        #define SLANG_OSX 0
    #endif
    #ifndef SLANG_PS3
        #define SLANG_PS3 0
    #endif
    #ifndef SLANG_PS4
        #define SLANG_PS4 0
    #endif
    #ifndef SLANG_PSP2
        #define SLANG_PSP2 0
    #endif
    #ifndef SLANG_WIIU
        #define SLANG_WIIU 0
    #endif
    #ifndef SLANG_WASM
        #define SLANG_WASM 0
    #endif
#endif /* SLANG_PLATFORM */

/* Shorthands for "families" of compilers/platforms */
#define SLANG_GCC_FAMILY (SLANG_CLANG || SLANG_SNC || SLANG_GHS || SLANG_GCC)
#define SLANG_WINDOWS_FAMILY (SLANG_WINRT || SLANG_WIN32 || SLANG_WIN64)
#define SLANG_MICROSOFT_FAMILY (SLANG_XBOXONE || SLANG_X360 || SLANG_WINDOWS_FAMILY)
#define SLANG_LINUX_FAMILY (SLANG_LINUX || SLANG_ANDROID)
#define SLANG_APPLE_FAMILY (SLANG_IOS || SLANG_OSX) /* equivalent to #if __APPLE__ */
#define SLANG_UNIX_FAMILY \
    (SLANG_LINUX_FAMILY || SLANG_APPLE_FAMILY) /* shortcut for unix/posix platforms */

/* Macros concerning DirectX */
#if !defined(SLANG_CONFIG_DX_ON_VK) || !SLANG_CONFIG_DX_ON_VK
    #define SLANG_ENABLE_DXVK 0
    #define SLANG_ENABLE_VKD3D 0
#else
    #define SLANG_ENABLE_DXVK 1
    #define SLANG_ENABLE_VKD3D 1
#endif

#if SLANG_WINDOWS_FAMILY
    #define SLANG_ENABLE_DIRECTX 1
    #define SLANG_ENABLE_DXGI_DEBUG 1
    #define SLANG_ENABLE_DXBC_SUPPORT 1
    #define SLANG_ENABLE_PIX 1
#elif SLANG_LINUX_FAMILY
    #define SLANG_ENABLE_DIRECTX (SLANG_ENABLE_DXVK || SLANG_ENABLE_VKD3D)
    #define SLANG_ENABLE_DXGI_DEBUG 0
    #define SLANG_ENABLE_DXBC_SUPPORT 0
    #define SLANG_ENABLE_PIX 0
#else
    #define SLANG_ENABLE_DIRECTX 0
    #define SLANG_ENABLE_DXGI_DEBUG 0
    #define SLANG_ENABLE_DXBC_SUPPORT 0
    #define SLANG_ENABLE_PIX 0
#endif

/* Macro for declaring if a method is no throw. Should be set before the return parameter. */
#ifndef SLANG_NO_THROW
    #if SLANG_WINDOWS_FAMILY && !defined(SLANG_DISABLE_EXCEPTIONS)
        #define SLANG_NO_THROW __declspec(nothrow)
    #endif
#endif
#ifndef SLANG_NO_THROW
    #define SLANG_NO_THROW
#endif

/* The `SLANG_STDCALL` and `SLANG_MCALL` defines are used to set the calling
convention for interface methods.
*/
#ifndef SLANG_STDCALL
    #if SLANG_MICROSOFT_FAMILY
        #define SLANG_STDCALL __stdcall
    #else
        #define SLANG_STDCALL
    #endif
#endif
#ifndef SLANG_MCALL
    #define SLANG_MCALL SLANG_STDCALL
#endif


#if !defined(SLANG_STATIC) && !defined(SLANG_DYNAMIC)
    #define SLANG_DYNAMIC
#endif

#if defined(_MSC_VER)
    #define SLANG_DLL_EXPORT __declspec(dllexport)
#else
    #if SLANG_WINDOWS_FAMILY
        #define SLANG_DLL_EXPORT \
            __attribute__((dllexport)) __attribute__((__visibility__("default")))
    #else
        #define SLANG_DLL_EXPORT __attribute__((__visibility__("default")))
    #endif
#endif

#if defined(SLANG_DYNAMIC)
    #if defined(_MSC_VER)
        #ifdef SLANG_DYNAMIC_EXPORT
            #define SLANG_API SLANG_DLL_EXPORT
        #else
            #define SLANG_API __declspec(dllimport)
        #endif
    #else
        // TODO: need to consider compiler capabilities
        // #     ifdef SLANG_DYNAMIC_EXPORT
        #define SLANG_API SLANG_DLL_EXPORT
    // #     endif
    #endif
#endif

#ifndef SLANG_API
    #define SLANG_API
#endif

// GCC Specific
#if SLANG_GCC_FAMILY
    #define SLANG_NO_INLINE __attribute__((noinline))
    #define SLANG_FORCE_INLINE inline __attribute__((always_inline))
    #define SLANG_BREAKPOINT(id) __builtin_trap();
#endif // SLANG_GCC_FAMILY

#if SLANG_GCC_FAMILY || defined(__clang__)
    // Use the builtin directly so we don't need to have an include of stddef.h
    #define SLANG_OFFSET_OF(T, ELEMENT) __builtin_offsetof(T, ELEMENT)
#endif

#ifndef SLANG_OFFSET_OF
    #define SLANG_OFFSET_OF(T, ELEMENT) (size_t(&((T*)1)->ELEMENT) - 1)
#endif

// Microsoft VC specific
#if SLANG_VC
    #define SLANG_NO_INLINE __declspec(noinline)
    #define SLANG_FORCE_INLINE __forceinline
    #define SLANG_BREAKPOINT(id) __debugbreak();

    #define SLANG_INT64(x) (x##i64)
    #define SLANG_UINT64(x) (x##ui64)
#endif // SLANG_MICROSOFT_FAMILY

#ifndef SLANG_FORCE_INLINE
    #define SLANG_FORCE_INLINE inline
#endif
#ifndef SLANG_NO_INLINE
    #define SLANG_NO_INLINE
#endif

#ifndef SLANG_COMPILE_TIME_ASSERT
    #define SLANG_COMPILE_TIME_ASSERT(x) static_assert(x)
#endif

#ifndef SLANG_BREAKPOINT
    // Make it crash with a write to 0!
    #define SLANG_BREAKPOINT(id) (*((int*)0) = int(id));
#endif

// Use for getting the amount of members of a standard C array.
// Use 0[x] here to catch the case where x has an overloaded subscript operator
#define SLANG_COUNT_OF(x) (SlangSSizeT(sizeof(x) / sizeof(0 [x])))
/// SLANG_INLINE exists to have a way to inline consistent with SLANG_ALWAYS_INLINE
#define SLANG_INLINE inline

// If explicitly disabled and not set, set to not available
#if !defined(SLANG_HAS_EXCEPTIONS) && defined(SLANG_DISABLE_EXCEPTIONS)
    #define SLANG_HAS_EXCEPTIONS 0
#endif

// If not set, the default is exceptions are available
#ifndef SLANG_HAS_EXCEPTIONS
    #define SLANG_HAS_EXCEPTIONS 1
#endif

// Other defines
#define SLANG_STRINGIZE_HELPER(X) #X
#define SLANG_STRINGIZE(X) SLANG_STRINGIZE_HELPER(X)

#define SLANG_CONCAT_HELPER(X, Y) X##Y
#define SLANG_CONCAT(X, Y) SLANG_CONCAT_HELPER(X, Y)

#ifndef SLANG_UNUSED
    #define SLANG_UNUSED(v) (void)v;
#endif

#if defined(__llvm__)
    #define SLANG_MAYBE_UNUSED [[maybe_unused]]
#else
    #define SLANG_MAYBE_UNUSED
#endif

// Used for doing constant literals
#ifndef SLANG_INT64
    #define SLANG_INT64(x) (x##ll)
#endif
#ifndef SLANG_UINT64
    #define SLANG_UINT64(x) (x##ull)
#endif


#ifdef __cplusplus
    #define SLANG_EXTERN_C extern "C"
#else
    #define SLANG_EXTERN_C
#endif

#ifdef __cplusplus
    // C++ specific macros
    // Clang
    #if SLANG_CLANG
        #if (__clang_major__ * 10 + __clang_minor__) >= 33
            #define SLANG_HAS_MOVE_SEMANTICS 1
            #define SLANG_HAS_ENUM_CLASS 1
            #define SLANG_OVERRIDE override
        #endif

    // Gcc
    #elif SLANG_GCC_FAMILY
        // Check for C++11
        #if (__cplusplus >= 201103L)
            #if (__GNUC__ * 100 + __GNUC_MINOR__) >= 405
                #define SLANG_HAS_MOVE_SEMANTICS 1
            #endif
            #if (__GNUC__ * 100 + __GNUC_MINOR__) >= 406
                #define SLANG_HAS_ENUM_CLASS 1
            #endif
            #if (__GNUC__ * 100 + __GNUC_MINOR__) >= 407
                #define SLANG_OVERRIDE override
            #endif
        #endif
    #endif // SLANG_GCC_FAMILY

    // Visual Studio

    #if SLANG_VC
        // C4481: nonstandard extension used: override specifier 'override'
        #if _MSC_VER < 1700
            #pragma warning(disable : 4481)
        #endif
        #define SLANG_OVERRIDE override
        #if _MSC_VER >= 1600
            #define SLANG_HAS_MOVE_SEMANTICS 1
        #endif
        #if _MSC_VER >= 1700
            #define SLANG_HAS_ENUM_CLASS 1
        #endif
    #endif // SLANG_VC

    // Set non set
    #ifndef SLANG_OVERRIDE
        #define SLANG_OVERRIDE
    #endif
    #ifndef SLANG_HAS_ENUM_CLASS
        #define SLANG_HAS_ENUM_CLASS 0
    #endif
    #ifndef SLANG_HAS_MOVE_SEMANTICS
        #define SLANG_HAS_MOVE_SEMANTICS 0
    #endif

#endif // __cplusplus

/* Macros for detecting processor */
#if defined(_M_ARM) || defined(__ARM_EABI__)
    // This is special case for nVidia tegra
    #define SLANG_PROCESSOR_ARM 1
#elif defined(__i386__) || defined(_M_IX86)
    #define SLANG_PROCESSOR_X86 1
#elif defined(_M_AMD64) || defined(_M_X64) || defined(__amd64) || defined(__x86_64)
    #define SLANG_PROCESSOR_X86_64 1
#elif defined(_PPC_) || defined(__ppc__) || defined(__POWERPC__) || defined(_M_PPC)
    #if defined(__powerpc64__) || defined(__ppc64__) || defined(__PPC64__) || \
        defined(__64BIT__) || defined(_LP64) || defined(__LP64__)
        #define SLANG_PROCESSOR_POWER_PC_64 1
    #else
        #define SLANG_PROCESSOR_POWER_PC 1
    #endif
#elif defined(__arm__)
    #define SLANG_PROCESSOR_ARM 1
#elif defined(_M_ARM64) || defined(__aarch64__) || defined(__ARM_ARCH_ISA_A64)
    #define SLANG_PROCESSOR_ARM_64 1
#elif defined(__EMSCRIPTEN__)
    #define SLANG_PROCESSOR_WASM 1
#endif

#ifndef SLANG_PROCESSOR_ARM
    #define SLANG_PROCESSOR_ARM 0
#endif

#ifndef SLANG_PROCESSOR_ARM_64
    #define SLANG_PROCESSOR_ARM_64 0
#endif

#ifndef SLANG_PROCESSOR_X86
    #define SLANG_PROCESSOR_X86 0
#endif

#ifndef SLANG_PROCESSOR_X86_64
    #define SLANG_PROCESSOR_X86_64 0
#endif

#ifndef SLANG_PROCESSOR_POWER_PC
    #define SLANG_PROCESSOR_POWER_PC 0
#endif

#ifndef SLANG_PROCESSOR_POWER_PC_64
    #define SLANG_PROCESSOR_POWER_PC_64 0
#endif

#ifndef SLANG_PROCESSOR_WASM
    #define SLANG_PROCESSOR_WASM 0
#endif

// Processor families

#define SLANG_PROCESSOR_FAMILY_X86 (SLANG_PROCESSOR_X86_64 | SLANG_PROCESSOR_X86)
#define SLANG_PROCESSOR_FAMILY_ARM (SLANG_PROCESSOR_ARM | SLANG_PROCESSOR_ARM_64)
#define SLANG_PROCESSOR_FAMILY_POWER_PC (SLANG_PROCESSOR_POWER_PC_64 | SLANG_PROCESSOR_POWER_PC)

// Pointer size
#define SLANG_PTR_IS_64 \
    (SLANG_PROCESSOR_ARM_64 | SLANG_PROCESSOR_X86_64 | SLANG_PROCESSOR_POWER_PC_64)
#define SLANG_PTR_IS_32 (SLANG_PTR_IS_64 ^ 1)

// Processor features
#if SLANG_PROCESSOR_FAMILY_X86
    #define SLANG_LITTLE_ENDIAN 1
    #define SLANG_UNALIGNED_ACCESS 1
#elif SLANG_PROCESSOR_FAMILY_ARM
    #if defined(__ARMEB__)
        #define SLANG_BIG_ENDIAN 1
    #else
        #define SLANG_LITTLE_ENDIAN 1
    #endif
#elif SLANG_PROCESSOR_FAMILY_POWER_PC
    #define SLANG_BIG_ENDIAN 1
#elif SLANG_WASM
    #define SLANG_LITTLE_ENDIAN 1
#endif

#ifndef SLANG_LITTLE_ENDIAN
    #define SLANG_LITTLE_ENDIAN 0
#endif

#ifndef SLANG_BIG_ENDIAN
    #define SLANG_BIG_ENDIAN 0
#endif

#ifndef SLANG_UNALIGNED_ACCESS
    #define SLANG_UNALIGNED_ACCESS 0
#endif

// Backtrace
#if SLANG_LINUX_FAMILY
    #include <features.h> // for __GLIBC__ define, if using GNU libc
    #if defined(__GLIBC__) || (__ANDROID_API__ >= 33)
        #define SLANG_HAS_BACKTRACE 1
    #else
        #define SLANG_HAS_BACKTRACE 0
    #endif
#else
    #define SLANG_HAS_BACKTRACE 0
#endif

// One endianness must be set
#if ((SLANG_BIG_ENDIAN | SLANG_LITTLE_ENDIAN) == 0)
    #error "Couldn't determine endianness"
#endif

#ifndef SLANG_NO_INTTYPES
    #include <inttypes.h>
#endif // ! SLANG_NO_INTTYPES

#ifndef SLANG_NO_STDDEF
    #include <stddef.h>
#endif // ! SLANG_NO_STDDEF

#ifdef SLANG_NO_DEPRECATION
    #define SLANG_DEPRECATED
#else
    #define SLANG_DEPRECATED [[deprecated]]
#endif

#ifdef __cplusplus
extern "C"
{
#endif
    /*!
    @mainpage Introduction

    API Reference: slang.h

    @file slang.h
    */

    typedef uint32_t SlangUInt32;
    typedef int32_t SlangInt32;

    // Use SLANG_PTR_ macros to determine SlangInt/SlangUInt types.
    // This is used over say using size_t/ptrdiff_t/intptr_t/uintptr_t, because on some targets,
    // these types are distinct from their uint_t/int_t equivalents and so produce ambiguity with
    // function overloading.
    //
    // SlangSizeT is helpful as on some compilers size_t is distinct from a regular integer type and
    // so overloading doesn't work. Casting to SlangSizeT works around this.
#if SLANG_PTR_IS_64
    typedef int64_t SlangInt;
    typedef uint64_t SlangUInt;

    typedef int64_t SlangSSizeT;
    typedef uint64_t SlangSizeT;
#else
typedef int32_t SlangInt;
typedef uint32_t SlangUInt;

typedef int32_t SlangSSizeT;
typedef uint32_t SlangSizeT;
#endif

    typedef bool SlangBool;


    /*!
    @brief Severity of a diagnostic generated by the compiler.
    Values come from the enum below, with higher values representing more severe
    conditions, and all values >= SLANG_SEVERITY_ERROR indicating compilation
    failure.
    */
    typedef int SlangSeverityIntegral;
    enum SlangSeverity : SlangSeverityIntegral
    {
        SLANG_SEVERITY_DISABLED = 0, /**< A message that is disabled, filtered out. */
        SLANG_SEVERITY_NOTE = 1,     /**< An informative message. */
        SLANG_SEVERITY_WARNING = 2,  /**< A warning, which indicates a possible problem. */
        SLANG_SEVERITY_ERROR = 3,    /**< An error, indicating that compilation failed. */
        SLANG_SEVERITY_FATAL = 4, /**< An unrecoverable error, which forced compilation to abort. */
        SLANG_SEVERITY_INTERNAL =
            5, /**< An internal error, indicating a logic error in the compiler.
                */
    };

    /* A warning "level" (group), modeled on the clang/gcc -Wall/-Wextra/-Wpedantic
    groups. Each group is enabled independently: a warning tagged with a group is
    emitted only when that group has been enabled, while warnings in the implicit
    Default group are always emitted. */
    typedef int SlangWarningLevelIntegral;
    enum SlangWarningLevel : SlangWarningLevelIntegral
    {
        SLANG_WARNING_LEVEL_DEFAULT = 0,  /**< Always emitted; this is the baseline group and is not
                                             something a caller enables explicitly. */
        SLANG_WARNING_LEVEL_ALL = 1,      /**< Warnings enabled by -Wall. */
        SLANG_WARNING_LEVEL_EXTRA = 2,    /**< Warnings enabled by -Wextra. */
        SLANG_WARNING_LEVEL_PEDANTIC = 3, /**< Warnings enabled by -Wpedantic. */
    };

    typedef int SlangDiagnosticFlags;
    enum
    {
        SLANG_DIAGNOSTIC_FLAG_VERBOSE_PATHS = 0x01,
        SLANG_DIAGNOSTIC_FLAG_TREAT_WARNINGS_AS_ERRORS = 0x02
    };

    typedef int SlangBindableResourceIntegral;
    enum SlangBindableResourceType : SlangBindableResourceIntegral
    {
        SLANG_NON_BINDABLE = 0,
        SLANG_TEXTURE = 1,
        SLANG_SAMPLER = 2,
        SLANG_UNIFORM_BUFFER = 3,
        SLANG_STORAGE_BUFFER = 4,
    };

    /* NOTE! To keep binary compatibility care is needed with this enum!

    * To add value, only add at the bottom (before COUNT_OF)
    * To remove a value, add _DEPRECATED as a suffix, but leave in the list

    This will make the enum values stable, and compatible with libraries that might not use the
    latest enum values.
    */
    typedef int SlangCompileTargetIntegral;
    enum SlangCompileTarget : SlangCompileTargetIntegral
    {
        SLANG_TARGET_UNKNOWN = 0,
        SLANG_TARGET_NONE = 1,
        SLANG_GLSL = 2,
        SLANG_GLSL_VULKAN_DEPRECATED = 3, //< deprecated and removed: just use `SLANG_GLSL`.
        SLANG_GLSL_VULKAN_ONE_DESC_DEPRECATED = 4, //< deprecated and removed.
        SLANG_HLSL = 5,
        SLANG_SPIRV = 6,
        SLANG_SPIRV_ASM = 7,
        SLANG_DXBC = 8,
        SLANG_DXBC_ASM = 9,
        SLANG_DXIL = 10,
        SLANG_DXIL_ASM = 11,
        SLANG_C_SOURCE = 12,              ///< The C language
        SLANG_CPP_SOURCE = 13,            ///< C++ code for shader kernels.
        SLANG_HOST_EXECUTABLE = 14,       ///< Standalone binary executable (for hosting CPU/OS)
        SLANG_SHADER_SHARED_LIBRARY = 15, ///< A shared library/Dll for shader kernels (for hosting
                                          ///< CPU/OS)
        SLANG_SHADER_HOST_CALLABLE = 16,  ///< A CPU target that makes the compiled shader code
                                          ///< available to be run immediately
        SLANG_CUDA_SOURCE = 17,           ///< Cuda source
        SLANG_PTX = 18,                   ///< PTX
        SLANG_CUDA_OBJECT_CODE = 19,      ///< Object code that contains CUDA functions.
        SLANG_OBJECT_CODE = 20, ///< Object code that can be used for later linking (kernel/shader)
        SLANG_HOST_CPP_SOURCE = 21,     ///< C++ code for host library or executable.
        SLANG_HOST_HOST_CALLABLE = 22,  ///< Host callable host code (ie non kernel/shader)
        SLANG_CPP_PYTORCH_BINDING = 23, ///< C++ PyTorch binding code.
        SLANG_METAL = 24,               ///< Metal shading language
        SLANG_METAL_LIB = 25,           ///< Metal library
        SLANG_METAL_LIB_ASM = 26,       ///< Metal library assembly
        SLANG_HOST_SHARED_LIBRARY = 27, ///< A shared library/Dll for host code (for hosting CPU/OS)
        SLANG_WGSL = 28,                ///< WebGPU shading language
        SLANG_WGSL_SPIRV_ASM = 29,      ///< SPIR-V assembly via WebGPU shading language
        SLANG_WGSL_SPIRV = 30,          ///< SPIR-V via WebGPU shading language

        SLANG_HOST_VM = 31,     ///< Bytecode that can be interpreted by the Slang VM
        SLANG_CPP_HEADER = 32,  ///< C++ header for shader kernels.
        SLANG_CUDA_HEADER = 33, ///< Cuda header

        SLANG_HOST_OBJECT_CODE = 34, ///< Host object code
        SLANG_HOST_LLVM_IR = 35,     ///< Host LLVM IR assembly
        SLANG_SHADER_LLVM_IR = 36,   ///< Host LLVM IR assembly (kernel/shader)

        SLANG_TARGET_COUNT_OF,
    };

    /* A "container format" describes the way that the outputs
    for multiple files, entry points, targets, etc. should be
    combined into a single artifact for output. */
    typedef int SlangContainerFormatIntegral;
    enum SlangContainerFormat : SlangContainerFormatIntegral
    {
        /* Don't generate a container. */
        SLANG_CONTAINER_FORMAT_NONE = 0,

        /* Generate a container in the `.slang-module` format,
        which includes reflection information, compiled kernels, etc. */
        SLANG_CONTAINER_FORMAT_SLANG_MODULE = 1,
    };

    typedef int SlangPassThroughIntegral;
    enum SlangPassThrough : SlangPassThroughIntegral
    {
        SLANG_PASS_THROUGH_NONE = 0,
        SLANG_PASS_THROUGH_FXC = 1,
        SLANG_PASS_THROUGH_DXC = 2,
        SLANG_PASS_THROUGH_GLSLANG = 3,
        SLANG_PASS_THROUGH_SPIRV_DIS = 4,
        SLANG_PASS_THROUGH_CLANG = 5,         ///< Clang C/C++ compiler
        SLANG_PASS_THROUGH_VISUAL_STUDIO = 6, ///< Visual studio C/C++ compiler
        SLANG_PASS_THROUGH_GCC = 7,           ///< GCC C/C++ compiler
        SLANG_PASS_THROUGH_GENERIC_C_CPP = 8, ///< Generic C or C++ compiler, which is decided by
                                              ///< the source type
        SLANG_PASS_THROUGH_NVRTC = 9,         ///< NVRTC Cuda compiler
        SLANG_PASS_THROUGH_LLVM = 10,         ///< LLVM 'compiler' - includes LLVM and Clang
        SLANG_PASS_THROUGH_SPIRV_OPT = 11,    ///< SPIRV-opt
        SLANG_PASS_THROUGH_METAL = 12,        ///< Metal compiler
        SLANG_PASS_THROUGH_TINT = 13,         ///< Tint WGSL compiler
        SLANG_PASS_THROUGH_SPIRV_LINK = 14,   ///< SPIRV-link
        SLANG_PASS_THROUGH_COUNT_OF,
    };

    /* Defines an archive type used to holds a 'file system' type structure. */
    typedef int SlangArchiveTypeIntegral;
    enum SlangArchiveType : SlangArchiveTypeIntegral
    {
        SLANG_ARCHIVE_TYPE_UNDEFINED = 0,
        SLANG_ARCHIVE_TYPE_ZIP = 1,
        SLANG_ARCHIVE_TYPE_RIFF = 2, ///< Riff container with no compression
        SLANG_ARCHIVE_TYPE_RIFF_DEFLATE = 3,
        SLANG_ARCHIVE_TYPE_RIFF_LZ4 = 4,
        SLANG_ARCHIVE_TYPE_COUNT_OF,
    };

    /*!
    Flags to control compilation behavior.
    */
    typedef unsigned int SlangCompileFlags;
    enum
    {
        /* Do as little mangling of names as possible, to try to preserve original names */
        SLANG_COMPILE_FLAG_NO_MANGLING = 1 << 3,

        /* Skip code generation step, just check the code and generate layout */
        SLANG_COMPILE_FLAG_NO_CODEGEN = 1 << 4,

        /* Obfuscate shader names on release products */
        SLANG_COMPILE_FLAG_OBFUSCATE = 1 << 5,

        /* Deprecated flags: kept around to allow existing applications to
        compile. Note that the relevant features will still be left in
        their default state. */
        SLANG_COMPILE_FLAG_NO_CHECKING = 0,
        SLANG_COMPILE_FLAG_SPLIT_MIXED_TYPES = 0,
    };

    /*!
    @brief Flags to control code generation behavior of a compilation target */
    typedef unsigned int SlangTargetFlags;
    enum
    {
        /* When compiling for a D3D Shader Model 5.1 or higher target, allocate
           distinct register spaces for parameter blocks.

           @deprecated This behavior is now enabled unconditionally.
        */
        SLANG_TARGET_FLAG_PARAMETER_BLOCKS_USE_REGISTER_SPACES = 1 << 4,

        /* When set, will generate target code that contains all entrypoints defined
           in the input source or specified via the `spAddEntryPoint` function in a
           single output module (library/source file).
        */
        SLANG_TARGET_FLAG_GENERATE_WHOLE_PROGRAM = 1 << 8,

        /* When set, will dump out the IR between intermediate compilation steps.*/
        SLANG_TARGET_FLAG_DUMP_IR = 1 << 9,

        /* When set, will generate SPIRV directly rather than via glslang. */
        // This flag will be deprecated, use CompilerOption instead.
        SLANG_TARGET_FLAG_GENERATE_SPIRV_DIRECTLY = 1 << 10,
    };
    inline constexpr SlangTargetFlags kDefaultTargetFlags =
        SLANG_TARGET_FLAG_GENERATE_SPIRV_DIRECTLY;

    /*!
    @brief Options to control floating-point precision guarantees for a target.
    */
    typedef unsigned int SlangFloatingPointModeIntegral;
    enum SlangFloatingPointMode : SlangFloatingPointModeIntegral
    {
        SLANG_FLOATING_POINT_MODE_DEFAULT = 0,
        SLANG_FLOATING_POINT_MODE_FAST = 1,
        SLANG_FLOATING_POINT_MODE_PRECISE = 2,
    };

    /*!
    @brief Options to control floating-point denormal handling mode for a target.
    */
    typedef unsigned int SlangFpDenormalModeIntegral;
    enum SlangFpDenormalMode : SlangFpDenormalModeIntegral
    {
        SLANG_FP_DENORM_MODE_ANY = 0,
        SLANG_FP_DENORM_MODE_PRESERVE = 1,
        SLANG_FP_DENORM_MODE_FTZ = 2,
    };

    /*!
    @brief Options to control emission of `#line` directives
    */
    typedef unsigned int SlangLineDirectiveModeIntegral;
    enum SlangLineDirectiveMode : SlangLineDirectiveModeIntegral
    {
        SLANG_LINE_DIRECTIVE_MODE_DEFAULT =
            0,                              /**< Default behavior: pick behavior base on target. */
        SLANG_LINE_DIRECTIVE_MODE_NONE = 1, /**< Don't emit line directives at all. */
        SLANG_LINE_DIRECTIVE_MODE_STANDARD = 2,   /**< Emit standard C-style `#line` directives. */
        SLANG_LINE_DIRECTIVE_MODE_GLSL = 3,       /**< Emit GLSL-style directives with file *number*
                                                       instead of name */
        SLANG_LINE_DIRECTIVE_MODE_SOURCE_MAP = 4, /**< Use a source map to track line mappings (ie
                                                     no #line will appear in emitting source) */
    };

    typedef int SlangSourceLanguageIntegral;
    enum SlangSourceLanguage : SlangSourceLanguageIntegral
    {
        SLANG_SOURCE_LANGUAGE_UNKNOWN = 0,
        SLANG_SOURCE_LANGUAGE_SLANG = 1,
        SLANG_SOURCE_LANGUAGE_HLSL = 2,
        SLANG_SOURCE_LANGUAGE_GLSL = 3,
        SLANG_SOURCE_LANGUAGE_C = 4,
        SLANG_SOURCE_LANGUAGE_CPP = 5,
        SLANG_SOURCE_LANGUAGE_CUDA = 6,
        SLANG_SOURCE_LANGUAGE_SPIRV = 7,
        SLANG_SOURCE_LANGUAGE_METAL = 8,
        SLANG_SOURCE_LANGUAGE_WGSL = 9,
        SLANG_SOURCE_LANGUAGE_LLVM = 10,
        SLANG_SOURCE_LANGUAGE_COUNT_OF,
    };

    typedef unsigned int SlangProfileIDIntegral;
    enum SlangProfileID : SlangProfileIDIntegral
    {
        SLANG_PROFILE_UNKNOWN = 0,
    };


    typedef SlangInt32 SlangCapabilityIDIntegral;
    enum SlangCapabilityID : SlangCapabilityIDIntegral
    {
        SLANG_CAPABILITY_UNKNOWN = 0,
    };

    typedef unsigned int SlangMatrixLayoutModeIntegral;
    enum SlangMatrixLayoutMode : SlangMatrixLayoutModeIntegral
    {
        SLANG_MATRIX_LAYOUT_MODE_UNKNOWN = 0,
        SLANG_MATRIX_LAYOUT_ROW_MAJOR = 1,
        SLANG_MATRIX_LAYOUT_COLUMN_MAJOR = 2,
    };

    typedef SlangUInt32 SlangStageIntegral;
    enum SlangStage : SlangStageIntegral
    {
        SLANG_STAGE_NONE = 0,
        SLANG_STAGE_VERTEX = 1,
        SLANG_STAGE_HULL = 2,
        SLANG_STAGE_DOMAIN = 3,
        SLANG_STAGE_GEOMETRY = 4,
        SLANG_STAGE_FRAGMENT = 5,
        SLANG_STAGE_COMPUTE = 6,
        SLANG_STAGE_RAY_GENERATION = 7,
        SLANG_STAGE_INTERSECTION = 8,
        SLANG_STAGE_ANY_HIT = 9,
        SLANG_STAGE_CLOSEST_HIT = 10,
        SLANG_STAGE_MISS = 11,
        SLANG_STAGE_CALLABLE = 12,
        SLANG_STAGE_MESH = 13,
        SLANG_STAGE_AMPLIFICATION = 14,
        SLANG_STAGE_DISPATCH = 15,
        SLANG_STAGE_NODE = 16,
        //
        SLANG_STAGE_COUNT,

        // alias:
        SLANG_STAGE_PIXEL = SLANG_STAGE_FRAGMENT,
    };

    typedef SlangUInt32 SlangScopeIntegral;
    enum SlangScope : SlangScopeIntegral
    {
        SLANG_SCOPE_NONE,
        SLANG_SCOPE_THREAD,
        SLANG_SCOPE_WAVE,
        SLANG_SCOPE_THREAD_GROUP,
    };

    typedef SlangUInt32 SlangCooperativeMatrixUseIntegral;
    enum SlangCooperativeMatrixUse : SlangCooperativeMatrixUseIntegral
    {
        SLANG_COOPERATIVE_MATRIX_USE_A = 0,
        SLANG_COOPERATIVE_MATRIX_USE_B = 1,
        SLANG_COOPERATIVE_MATRIX_USE_ACCUMULATOR = 2,
    };

    typedef SlangUInt32 SlangCooperativeVectorMatrixLayoutIntegral;
    enum SlangCooperativeVectorMatrixLayout : SlangCooperativeVectorMatrixLayoutIntegral
    {
        SLANG_COOPERATIVE_VECTOR_MATRIX_LAYOUT_ROW_MAJOR = 0,
        SLANG_COOPERATIVE_VECTOR_MATRIX_LAYOUT_COLUMN_MAJOR = 1,
        SLANG_COOPERATIVE_VECTOR_MATRIX_LAYOUT_INFERENCING_OPTIMAL = 2,
        SLANG_COOPERATIVE_VECTOR_MATRIX_LAYOUT_TRAINING_OPTIMAL = 3,
    };

    typedef SlangUInt32 SlangDebugInfoLevelIntegral;
    enum SlangDebugInfoLevel : SlangDebugInfoLevelIntegral
    {
        SLANG_DEBUG_INFO_LEVEL_NONE = 0,     /**< Don't emit debug information at all. */
        SLANG_DEBUG_INFO_LEVEL_MINIMAL = 1,  /**< Emit as little debug information as possible,
                                                  while still supporting stack trackers. */
        SLANG_DEBUG_INFO_LEVEL_STANDARD = 2, /**< Emit whatever is the standard level of debug
                                                  information for each target. */
        SLANG_DEBUG_INFO_LEVEL_MAXIMAL = 3,  /**< Emit as much debug information as possible for
                                                  each target. */
    };

    /* Describes the debugging information format produced during a compilation. */
    typedef SlangUInt32 SlangDebugInfoFormatIntegral;
    enum SlangDebugInfoFormat : SlangDebugInfoFormatIntegral
    {
        SLANG_DEBUG_INFO_FORMAT_DEFAULT = 0, ///< Use the default debugging format for the target
        SLANG_DEBUG_INFO_FORMAT_C7 = 1,      ///< CodeView C7 format (typically means debugging
                                             ///< information is embedded in the binary)
        SLANG_DEBUG_INFO_FORMAT_PDB = 2,     ///< Program database

        SLANG_DEBUG_INFO_FORMAT_STABS = 3, ///< Stabs
        SLANG_DEBUG_INFO_FORMAT_COFF = 4,  ///< COFF debug info
        SLANG_DEBUG_INFO_FORMAT_DWARF = 5, ///< DWARF debug info (we may want to support specifying
                                           ///< the version)

        SLANG_DEBUG_INFO_FORMAT_COUNT_OF,
    };

    typedef SlangUInt32 SlangOptimizationLevelIntegral;
    enum SlangOptimizationLevel : SlangOptimizationLevelIntegral
    {
        SLANG_OPTIMIZATION_LEVEL_NONE = 0,    /**< Don't optimize at all. */
        SLANG_OPTIMIZATION_LEVEL_DEFAULT = 1, /**< Default optimization level: balance code quality
                                                   and compilation time. */
        SLANG_OPTIMIZATION_LEVEL_HIGH = 2,    /**< Optimize aggressively. */
        SLANG_OPTIMIZATION_LEVEL_MAXIMAL = 3, /**< Include optimizations that may take a very long
                                                   time, or may involve severe space-vs-speed
                                                   tradeoffs */
    };

    enum SlangEmitSpirvMethod
    {
        SLANG_EMIT_SPIRV_DEFAULT = 0,
        SLANG_EMIT_SPIRV_VIA_GLSL = 1,
        SLANG_EMIT_SPIRV_DIRECTLY = 2,
    };

    enum SlangEmitCPUMethod
    {
        SLANG_EMIT_CPU_DEFAULT = 0,
        SLANG_EMIT_CPU_VIA_CPP = 1,
        SLANG_EMIT_CPU_VIA_LLVM = 2,
    };

    enum SlangDiagnosticColor
    {
        SLANG_DIAGNOSTIC_COLOR_AUTO = 0,   // Use color if output sink is a tty
        SLANG_DIAGNOSTIC_COLOR_ALWAYS = 1, // Always use color
        SLANG_DIAGNOSTIC_COLOR_NEVER = 2,  // Never use color
    };

    // All compiler option names supported by Slang.
    //
    // IMPORTANT: ABI STABILITY POLICY FOR CompilerOptionName
    //
    // Every enumerator except the terminal CountOf sentinel has an explicit integer value. Rules:
    //   1. NEVER insert a new enumerator in the middle of the list.
    //   2. NEVER remove an enumerator; rename to REMOVED_<Name> and keep the value.
    //   3. NEVER reuse an integer value from a removed/deprecated entry.
    //   4. NEW entries MUST be appended immediately before CountOf, assigning the next
    //      sequential integer (equal to the preceding enumerator's value + 1).
    //
    // Violation of these rules silently breaks binary compatibility for any caller
    // compiled against an older version of this header.
    namespace slang
    {
    enum class CompilerOptionName
    {
        MacroDefine = 0, // stringValue0: macro name;  stringValue1: macro value
        DepFile = 1,
        EntryPointName = 2,
        Specialize = 3,
        Help = 4,
        HelpStyle = 5,
        Include = 6, // stringValue: additional include path.
        Language = 7,
        MatrixLayoutColumn = 8,          // bool
        MatrixLayoutRow = 9,             // bool
        ZeroInitialize = 10,             // bool
        IgnoreCapabilities = 11,         // bool
        RestrictiveCapabilityCheck = 12, // bool
        ModuleName = 13,                 // stringValue0: module name.
        Output = 14,
        Profile = 15, // intValue0: profile
        Stage = 16,   // intValue0: stage
        Target = 17,  // intValue0: CodeGenTarget
        Version = 18,
        WarningsAsErrors = 19, // stringValue0: "all" or comma-separated list of
                               // warning codes or names.
        DisableWarnings = 20,  // stringValue0: comma separated list of warning codes or names.
        EnableWarning = 21,    // stringValue0: warning code or name.
        DisableWarning = 22,   // stringValue0: warning code or name.
        DumpWarningDiagnostics = 23,
        InputFilesRemain = 24,
        EmitIr = 25,                        // bool
        ReportDownstreamTime = 26,          // bool
        ReportPerfBenchmark = 27,           // bool
        ReportCheckpointIntermediates = 28, // bool
        SkipSPIRVValidation = 29,           // bool
        SourceEmbedStyle = 30,
        SourceEmbedName = 31,
        SourceEmbedLanguage = 32,
        DisableShortCircuit = 33,            // bool
        MinimumSlangOptimization = 34,       // bool
        DisableNonEssentialValidations = 35, // bool
        DisableSourceMap = 36,               // bool
        UnscopedEnum = 37,                   // bool
        PreserveParameters = 38, // bool: preserve all resource parameters in the output code.

        // Target
        Capability = 39,                // intValue0: CapabilityName
        DefaultImageFormatUnknown = 40, // bool
        DisableDynamicDispatch = 41,    // bool
        DisableSpecialization = 42,     // bool
        FloatingPointMode = 43,         // intValue0: FloatingPointMode
        DebugInformation = 44,          // intValue0: DebugInfoLevel
        LineDirectiveMode = 45,
        Optimization = 46, // intValue0: OptimizationLevel
        Obfuscate = 47,    // bool

        VulkanBindShift = 48,      // intValue0 (higher 8 bits): kind; intValue0(lower bits): set;
                                   // intValue1: shift
        VulkanBindGlobals = 49,    // intValue0: index; intValue1: set
        VulkanInvertY = 50,        // bool
        VulkanUseDxPositionW = 51, // bool
        VulkanUseEntryPointName = 52, // bool
        VulkanUseGLLayout = 53,       // bool
        VulkanEmitReflection = 54,    // bool

        GLSLForceScalarLayout = 55,   // bool
        EnableEffectAnnotations = 56, // bool

        EmitSpirvViaGLSL = 57,     // bool (will be deprecated)
        EmitSpirvDirectly = 58,    // bool (will be deprecated)
        SPIRVCoreGrammarJSON = 59, // stringValue0: json path
        IncompleteLibrary = 60, // bool, when set, will not issue an error when the linked program
                                // has unresolved extern function symbols.

        // Downstream
        CompilerPath = 61,
        DefaultDownstreamCompiler = 62,
        DownstreamArgs = 63, // stringValue0: downstream compiler name. stringValue1: argument list,
                             // one per line.
        PassThrough = 64,

        // Repro
        DumpRepro = 65,
        DumpReproOnError = 66,
        ExtractRepro = 67,
        LoadRepro = 68,
        LoadReproDirectory = 69,
        ReproFallbackDirectory = 70,

        // Debugging
        DumpAst = 71,
        DumpIntermediatePrefix = 72,
        DumpIntermediates = 73, // bool
        DumpIr = 74,            // bool
        DumpIrIds = 75,
        PreprocessorOutput = 76,
        OutputIncludes = 77,
        ReproFileSystem = 78,
        REMOVED_SerialIR = 79, // deprecated and removed; value must never be reused
        SkipCodeGen = 80,      // bool
        ValidateIr = 81,       // bool
        VerbosePaths = 82,
        VerifyDebugSerialIr = 83,
        NoCodeGen = 84, // Not used.

        // Experimental
        FileSystem = 85,
        Heterogeneous = 86,
        NoMangle = 87,
        NoHLSLBinding = 88,
        NoHLSLPackConstantBufferElements = 89,
        ValidateUniformity = 90,
        AllowGLSL = 91,
        EnableExperimentalPasses = 92,
        BindlessSpaceIndex = 93,      // int
        SPIRVResourceHeapStride = 94, // int: byte stride for SPIRV resource descriptor heap
        SPIRVSamplerHeapStride = 95,  // int: byte stride for SPIRV sampler descriptor heap

        // Internal
        ArchiveType = 96,
        CompileCoreModule = 97,
        Doc = 98,

        IrCompression = 99, // deprecated; value must never be reused

        LoadCoreModule = 100,
        ReferenceModule = 101,
        SaveCoreModule = 102,
        SaveCoreModuleBinSource = 103,
        TrackLiveness = 104,
        LoopInversion = 105, // bool, enable loop inversion optimization

        ParameterBlocksUseRegisterSpaces = 106,  // Deprecated; value must never be reused
        LanguageVersion = 107,                   // intValue0: SlangLanguageVersion
        TypeConformance = 108,                   // stringValue0: type conformance to link; format:
                                                 // "<TypeName>:<IInterfaceName>[=<sequentialId>]",
                                                 // e.g. "Impl:IFoo=3" or "Impl:IFoo".
        EnableExperimentalDynamicDispatch = 109, // bool, experimental
        EmitReflectionJSON = 110,                // bool

        CountOfParsableOptions = 111, // historical sentinel; value must not be reused

        // Options added after the original set. Most have CLI flags; a few are
        // API-only (marked below). All future additions belong after DiagnosticColor,
        // immediately before CountOf.
        DebugInformationFormat = 112,  // intValue0: DebugInfoFormat (derived from -g; no direct CLI
                                       // flag)
        VulkanBindShiftAll = 113,      // intValue0: kind; intValue1: shift (derived from
                                       // -fvk-x-shift; no direct CLI flag)
        GenerateWholeProgram = 114,    // bool
        UseUpToDateBinaryModule = 115, // bool, when set, will only load precompiled modules
                                       // if up-to-date with source. (API-only; no direct CLI flag)
        EmbedDownstreamIR = 116,       // bool
        ForceDXLayout = 117,           // bool

        // Setting of EmitSpirvDirectly or EmitSpirvViaGLSL will turn into this option internally.
        EmitSpirvMethod = 118, // enum SlangEmitSpirvMethod (derived; no direct CLI flag)

        SaveGLSLModuleBinSource = 119,

        SkipDownstreamLinking = 120, // bool, experimental (API-only; no direct CLI flag)
        DumpModule = 121,

        GetModuleInfo = 122,              // Print serialized module version and name
        GetSupportedModuleVersions = 123, // Print the min and max module versions this compiler
                                          // supports

        EmitSeparateDebug = 124, // bool

        // Floating point denormal handling modes
        DenormalModeFp16 = 125,
        DenormalModeFp32 = 126,
        DenormalModeFp64 = 127,

        // Bitfield options
        UseMSVCStyleBitfieldPacking = 128, // bool

        ForceCLayout = 129, // bool

        ExperimentalFeature = 130, // bool, enable experimental features

        ReportDetailedPerfBenchmark = 131, // bool, reports detailed compiler performance benchmark
                                           // results
        ValidateIRDetailed = 132,          // bool, enable detailed IR validation
        DumpIRBefore = 133,                // string, pass name to dump IR before
        DumpIRAfter = 134,                 // string, pass name to dump IR after

        EmitCPUMethod = 135,    // enum SlangEmitCPUMethod (derived; no direct CLI flag)
        EmitCPUViaCPP = 136,    // bool
        EmitCPUViaLLVM = 137,   // bool
        LLVMTargetTriple = 138, // string
        LLVMCPU = 139,          // string
        LLVMFeatures = 140,     // string

        EnableRichDiagnostics = 141, // bool, enable the experimental rich diagnostics

        ReportDynamicDispatchSites = 142, // bool

        EnableMachineReadableDiagnostics = 143, // bool, enable machine-readable diagnostic output
                                                // (implies EnableRichDiagnostics)

        DiagnosticColor = 144, // intValue0: SlangDiagnosticColor (always, never, auto)

        // Add new options HERE, immediately before CountOf.

        TraceCoverage = 145, // bool: insert per-statement line coverage counters
        TraceCoverageBinding =
            146, // intValue0: register index; intValue1: register space - explicit
                 //   binding for the synthesized __slang_coverage buffer. Consumed
                 //   only when any coverage mode is enabled; the slangc CLI spelling
                 //   also enables TraceCoverage.
        TraceCoverageReservedSpace =
            147, // intValue0: descriptor/register space reserved by the host when
                 //   auto-allocating the synthesized __slang_coverage buffer. This is
                 //   a repeatable hint consumed only when any coverage mode is enabled.
        TraceFunctionCoverage = 148, // bool: insert per-function-entry coverage counters
        TraceBranchCoverage = 149,   // bool: insert per-branch-arm coverage counters
        CoverageManifestOutput =
            150, // stringValue0: explicit path for the slangc coverage manifest sidecar.
                 //   When unset, slangc writes <output>.coverage-manifest.json next to
                 //   file outputs that carry coverage metadata. This option is output
                 //   policy only and is excluded from compiler cache keys. It requires
                 //   at least one coverage tracing mode, is rejected for container
                 //   outputs, and errors if the selected outputs produce no coverage
                 //   metadata. Explicit paths are valid only when exactly one compiled
                 //   artifact carries coverage metadata and must not overlap any emitted
                 //   artifact path. Query/set with the string option APIs.
        TraceCoverageCounterByteWidth =
            151, // intValue0: per-slot byte width of the synthesized __slang_coverage
                 //   buffer. Accepts 4 (uint32) or 8 (uint64). Omitting the option
                 //   yields 8 when any coverage mode is enabled. Use 4 to opt down to
                 //   uint32 when the runtime driver lacks 64-bit shader atomic support
                 //   (notably MoltenVK on Apple Silicon, where Vulkan exposes
                 //   shaderBufferInt64Atomics = false). uint32 counters wrap silently
                 //   at 2^32 hits per slot; uint64 counters effectively do not wrap
                 //   within any practical run. The corresponding CLI flag
                 //   `-trace-coverage-counter-width <bits>` takes a bit count (32/64)
                 //   and stores the matching byte width here.
        TraceCoverageBoolean =
            152, // bool: record boolean coverage (CoverageCounterMode::Boolean) instead of exact
                 //   execution counts. Each counter is written with a plain non-atomic store
                 //   of `1`, eliminating atomic contention (much faster, and avoids the GPU
                 //   watchdog timeouts heavy coverage can trigger) at the cost of exact
                 //   counts. Off by default.
        // CLI-only query option `-<compiler>-version`: prints the version of the downstream
        // <compiler> Slang would actually load for that pass-through (via
        // IGlobalSession::getDownstreamCompilerVersion). It takes no value and is never stored on
        // an option set; it only drives the print-and-continue handler in the command-line parser.
        CompilerVersion = 153,

        SPIRVUnifiedDescriptorHeapStride =
            154, // bool: when set, emit each SPIRV resource descriptor-heap runtime array's
                 //   ArrayStride as the maximum of image and buffer descriptor sizes, so a
                 //   single heap shared by buffers and images is indexed at the device's unified
                 //   stride. Opt-in; mutually exclusive with a non-zero
                 //   `-spirv-resource-heap-stride` (combining the two is an error).

        // intValue0: a SlangWarningLevel group to enable (e.g. SLANG_WARNING_LEVEL_PEDANTIC).
        // Repeatable: enabling multiple groups is additive, matching how -Wall/-Wextra/-Wpedantic
        // combine on the command line. CLI spellings: -Wall, -Wextra, -Wpedantic.
        WarningLevel = 155,

        SeparateDebugInfoOutput =
            156, // stringValue0: explicit path for the slangc separate-debug-info sidecar.
                 //   When unset, slangc derives the sidecar path from the main artifact path.
                 //   This option is output policy only and is excluded from compiler cache keys.
                 //   It requires EmitSeparateDebug and permits the main artifact to be written to
                 //   stdout. A value of "-" writes the separate debug information to stdout when
                 //   the main artifact is written to a file. Query/set with the string option APIs.

        // Do not assign an explicit value to CountOf. It must remain one past the last option,
        // which it derives implicitly from the preceding (highest-valued) enumerator.
        CountOf,
    };

    enum class CompilerOptionValueKind
    {
        Int = 0,
        String = 1,
    };

    struct CompilerOptionValue
    {
        CompilerOptionValueKind kind = CompilerOptionValueKind::Int;
        int32_t intValue0 = 0;
        int32_t intValue1 = 0;
        const char* stringValue0 = nullptr;
        const char* stringValue1 = nullptr;
    };

    struct CompilerOptionEntry
    {
        CompilerOptionName name;
        CompilerOptionValue value;
    };
    } // namespace slang

    /** A result code for a Slang API operation.

    This type is generally compatible with the Windows API `HRESULT` type. In particular, negative
    values indicate failure results, while zero or positive results indicate success.

    In general, Slang APIs always return a zero result on success, unless documented otherwise.
    Strictly speaking a negative value indicates an error, a positive (or 0) value indicates
    success. This can be tested for with the macros SLANG_SUCCEEDED(x) or SLANG_FAILED(x).

    It can represent if the call was successful or not. It can also specify in an extensible manner
    what facility produced the result (as the integral 'facility') as well as what caused it (as an
    integral 'code'). Under the covers SlangResult is represented as a int32_t.

    SlangResult is designed to be compatible with COM HRESULT.

    It's layout in bits is as follows

    Severity | Facility | Code
    ---------|----------|-----
    31       |    30-16 | 15-0

    Severity - 1 fail, 0 is success - as SlangResult is signed 32 bits, means negative number
    indicates failure. Facility is where the error originated from. Code is the code specific to the
    facility.

    Result codes have the following styles,
    1) SLANG_name
    2) SLANG_s_f_name
    3) SLANG_s_name

    where s is S for success, E for error
    f is the short version of the facility name

    Style 1 is reserved for SLANG_OK and SLANG_FAIL as they are so commonly used.

    It is acceptable to expand 'f' to a longer name to differentiate a name or drop if unique
    without it. ie for a facility 'DRIVER' it might make sense to have an error of the form
    SLANG_E_DRIVER_OUT_OF_MEMORY
    */

    typedef int32_t SlangResult;

    //! Use to test if a result was failure. Never use result != SLANG_OK to test for failure, as
    //! there may be successful codes != SLANG_OK.
#define SLANG_FAILED(status) ((status) < 0)
    //! Use to test if a result succeeded. Never use result == SLANG_OK to test for success, as will
    //! detect other successful codes as a failure.
#define SLANG_SUCCEEDED(status) ((status) >= 0)

    //! Get the facility the result is associated with
#define SLANG_GET_RESULT_FACILITY(r) ((int32_t)(((r) >> 16) & 0x7fff))
    //! Get the result code for the facility
#define SLANG_GET_RESULT_CODE(r) ((int32_t)((r) & 0xffff))

#define SLANG_MAKE_ERROR(fac, code) \
    ((((int32_t)(fac)) << 16) | ((int32_t)(code)) | int32_t(0x80000000))
#define SLANG_MAKE_SUCCESS(fac, code) ((((int32_t)(fac)) << 16) | ((int32_t)(code)))

    /*************************** Facilities ************************************/

    //! Facilities compatible with windows COM - only use if known code is compatible
#define SLANG_FACILITY_WIN_GENERAL 0
#define SLANG_FACILITY_WIN_INTERFACE 4
#define SLANG_FACILITY_WIN_API 7

    //! Base facility -> so as to not clash with HRESULT values (values in 0x200 range do not appear
    //! used)
#define SLANG_FACILITY_BASE 0x200

    /*! Facilities numbers must be unique across a project to make the resulting result a unique
    number. It can be useful to have a consistent short name for a facility, as used in the name
    prefix */
#define SLANG_FACILITY_CORE SLANG_FACILITY_BASE
    /* Facility for codes, that are not uniquely defined/protected. Can be used to pass back a
    specific error without requiring system wide facility uniqueness. Codes should never be part of
    a public API. */
#define SLANG_FACILITY_INTERNAL SLANG_FACILITY_BASE + 1

    /// Base for external facilities. Facilities should be unique across modules.
#define SLANG_FACILITY_EXTERNAL_BASE 0x210

    /* ************************ Win COM compatible Results ******************************/
    // https://msdn.microsoft.com/en-us/library/windows/desktop/aa378137(v=vs.85).aspx

    //! SLANG_OK indicates success, and is equivalent to
    //! SLANG_MAKE_SUCCESS(SLANG_FACILITY_WIN_GENERAL, 0)
#define SLANG_OK 0
    //! SLANG_FAIL is the generic failure code - meaning a serious error occurred and the call
    //! couldn't complete
#define SLANG_FAIL SLANG_MAKE_ERROR(SLANG_FACILITY_WIN_GENERAL, 0x4005)

#define SLANG_MAKE_WIN_GENERAL_ERROR(code) SLANG_MAKE_ERROR(SLANG_FACILITY_WIN_GENERAL, code)

    //! Functionality is not implemented
#define SLANG_E_NOT_IMPLEMENTED SLANG_MAKE_WIN_GENERAL_ERROR(0x4001)
    //! Interface not be found
#define SLANG_E_NO_INTERFACE SLANG_MAKE_WIN_GENERAL_ERROR(0x4002)
    //! Operation was aborted (did not correctly complete)
#define SLANG_E_ABORT SLANG_MAKE_WIN_GENERAL_ERROR(0x4004)

    //! Indicates that a handle passed in as parameter to a method is invalid.
#define SLANG_E_INVALID_HANDLE SLANG_MAKE_ERROR(SLANG_FACILITY_WIN_API, 6)
    //! Indicates that an argument passed in as parameter to a method is invalid.
#define SLANG_E_INVALID_ARG SLANG_MAKE_ERROR(SLANG_FACILITY_WIN_API, 0x57)
    //! Operation could not complete - ran out of memory
#define SLANG_E_OUT_OF_MEMORY SLANG_MAKE_ERROR(SLANG_FACILITY_WIN_API, 0xe)

    /* *************************** other Results **************************************/

#define SLANG_MAKE_CORE_ERROR(code) SLANG_MAKE_ERROR(SLANG_FACILITY_CORE, code)

    // Supplied buffer is too small to be able to complete
#define SLANG_E_BUFFER_TOO_SMALL SLANG_MAKE_CORE_ERROR(1)
    //! Used to identify a Result that has yet to be initialized.
    //! It defaults to failure such that if used incorrectly will fail, as similar in concept to
    //! using an uninitialized variable.
#define SLANG_E_UNINITIALIZED SLANG_MAKE_CORE_ERROR(2)
    //! Returned from an async method meaning the output is invalid (thus an error), but a result
    //! for the request is pending, and will be returned on a subsequent call with the async handle.
#define SLANG_E_PENDING SLANG_MAKE_CORE_ERROR(3)
    //! Indicates a file/resource could not be opened
#define SLANG_E_CANNOT_OPEN SLANG_MAKE_CORE_ERROR(4)
    //! Indicates a file/resource could not be found
#define SLANG_E_NOT_FOUND SLANG_MAKE_CORE_ERROR(5)
    //! An unhandled internal failure (typically from unhandled exception)
#define SLANG_E_INTERNAL_FAIL SLANG_MAKE_CORE_ERROR(6)
    //! Could not complete because some underlying feature (hardware or software) was not available
#define SLANG_E_NOT_AVAILABLE SLANG_MAKE_CORE_ERROR(7)
    //! Could not complete because the operation times out.
#define SLANG_E_TIME_OUT SLANG_MAKE_CORE_ERROR(8)

    /** A "Universally Unique Identifier" (UUID)

    The Slang API uses UUIDs to identify interfaces when
    using `queryInterface`.

    This type is compatible with the `GUID` type defined
    by the Component Object Model (COM), but Slang is
    not dependent on COM.
    */
    struct SlangUUID
    {
        uint32_t data1;
        uint16_t data2;
        uint16_t data3;
        uint8_t data4[8];
    };

// Place at the start of an interface with the guid.
// Guid should be specified as SLANG_COM_INTERFACE(0x00000000, 0x0000, 0x0000, { 0xC0, 0x00, 0x00,
// 0x00, 0x00, 0x00, 0x00, 0x46 }) NOTE: it's the typical guid struct definition, without the
// surrounding {} It is not necessary to use the multiple parameters (we can wrap in parens), but
// this is simple.
#define SLANG_COM_INTERFACE(a, b, c, d0, d1, d2, d3, d4, d5, d6, d7) \
public:                                                              \
    SLANG_FORCE_INLINE constexpr static SlangUUID getTypeGuid()      \
    {                                                                \
        return {a, b, c, d0, d1, d2, d3, d4, d5, d6, d7};            \
    }

// Sometimes it's useful to associate a guid with a class to identify it. This macro can used for
// this, and the guid extracted via the getTypeGuid() function defined in the type
#define SLANG_CLASS_GUID(a, b, c, d0, d1, d2, d3, d4, d5, d6, d7) \
    SLANG_FORCE_INLINE constexpr static SlangUUID getTypeGuid()   \
    {                                                             \
        return {a, b, c, d0, d1, d2, d3, d4, d5, d6, d7};         \
    }

// Helper to fill in pairs of GUIDs and return pointers. This ensures that the
// type of the GUID passed matches the pointer type, and that it is derived
// from ISlangUnknown,
// TODO(c++20): would is_derived_from be more appropriate here for private inheritance of
// ISlangUnknown?
//
// with     : void createFoo(SlangUUID, void**);
//            Slang::ComPtr<Bar> myBar;
// call with: createFoo(SLANG_IID_PPV_ARGS(myBar.writeRef()))
// to call  : createFoo(Bar::getTypeGuid(), (void**)(myBar.writeRef()))
#define SLANG_IID_PPV_ARGS(ppType)                                                         \
    std::decay_t<decltype(**(ppType))>::getTypeGuid(),                                     \
        (                                                                                  \
            (void)[] {                                                                     \
                static_assert(                                                             \
                    std::is_base_of_v<ISlangUnknown, std::decay_t<decltype(**(ppType))>>); \
            },                                                                             \
            reinterpret_cast<void**>(ppType))


    /** Base interface for components exchanged through the API.

    This interface definition is compatible with the COM `IUnknown`,
    and uses the same UUID, but Slang does not require applications
    to use or initialize COM.
    */
    struct ISlangUnknown
    {
        SLANG_COM_INTERFACE(
            0x00000000,
            0x0000,
            0x0000,
            {0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46})

        virtual SLANG_NO_THROW SlangResult SLANG_MCALL
        queryInterface(SlangUUID const& uuid, void** outObject) = 0;
        virtual SLANG_NO_THROW uint32_t SLANG_MCALL addRef() = 0;
        virtual SLANG_NO_THROW uint32_t SLANG_MCALL release() = 0;

        /*
        Inline methods are provided to allow the above operations to be called
        using their traditional COM names/signatures:
        */
        SlangResult QueryInterface(struct _GUID const& uuid, void** outObject)
        {
            return queryInterface(*(SlangUUID const*)&uuid, outObject);
        }
        uint32_t AddRef() { return addRef(); }
        uint32_t Release() { return release(); }
    };
#define SLANG_UUID_ISlangUnknown ISlangUnknown::getTypeGuid()


    /* An interface to provide a mechanism to cast, that doesn't require ref counting
    and doesn't have to return a pointer to a ISlangUnknown derived class */
    class ISlangCastable : public ISlangUnknown
    {
        SLANG_COM_INTERFACE(
            0x87ede0e1,
            0x4852,
            0x44b0,
            {0x8b, 0xf2, 0xcb, 0x31, 0x87, 0x4d, 0xe2, 0x39});

        /// Can be used to cast to interfaces without reference counting.
        /// Also provides access to internal implementations, when they provide a guid
        /// Can simulate a 'generated' interface as long as kept in scope by cast from.
        virtual SLANG_NO_THROW void* SLANG_MCALL castAs(const SlangUUID& guid) = 0;
    };

    class ISlangClonable : public ISlangCastable
    {
        SLANG_COM_INTERFACE(
            0x1ec36168,
            0xe9f4,
            0x430d,
            {0xbb, 0x17, 0x4, 0x8a, 0x80, 0x46, 0xb3, 0x1f});

        /// Note the use of guid is for the desired interface/object.
        /// The object is returned *not* ref counted. Any type that can implements the interface,
        /// derives from ICastable, and so (not withstanding some other issue) will always return
        /// an ICastable interface which other interfaces/types are accessible from via castAs
        SLANG_NO_THROW virtual void* SLANG_MCALL clone(const SlangUUID& guid) = 0;
    };

    /** A "blob" of binary data.

    This interface definition is compatible with the `ID3DBlob` and `ID3D10Blob` interfaces.
    */
    struct ISlangBlob : public ISlangUnknown
    {
        SLANG_COM_INTERFACE(
            0x8BA5FB08,
            0x5195,
            0x40e2,
            {0xAC, 0x58, 0x0D, 0x98, 0x9C, 0x3A, 0x01, 0x02})

        virtual SLANG_NO_THROW void const* SLANG_MCALL getBufferPointer() = 0;
        virtual SLANG_NO_THROW size_t SLANG_MCALL getBufferSize() = 0;
    };
#define SLANG_UUID_ISlangBlob ISlangBlob::getTypeGuid()

    /* Can be requested from ISlangCastable cast to indicate the contained chars are null
     * terminated.
     */
    struct SlangTerminatedChars
    {
        SLANG_CLASS_GUID(
            0xbe0db1a8,
            0x3594,
            0x4603,
            {0xa7, 0x8b, 0xc4, 0x86, 0x84, 0x30, 0xdf, 0xbb});
        operator const char*() const { return chars; }
        char chars[1];
    };

    /** A (real or virtual) file system.

    Slang can make use of this interface whenever it would otherwise try to load files
    from disk, allowing applications to hook and/or override filesystem access from
    the compiler.

    It is the responsibility of
    the caller of any method that returns a ISlangBlob to release the blob when it is no
    longer used (using 'release').
    */

    struct ISlangFileSystem : public ISlangCastable
    {
        SLANG_COM_INTERFACE(
            0x003A09FC,
            0x3A4D,
            0x4BA0,
            {0xAD, 0x60, 0x1F, 0xD8, 0x63, 0xA9, 0x15, 0xAB})

        /** Load a file from `path` and return a blob of its contents
        @param path The path to load from, as a null-terminated UTF-8 string.
        @param outBlob A destination pointer to receive the blob of the file contents.
        @returns A `SlangResult` to indicate success or failure in loading the file.

        NOTE! This is a *binary* load - the blob should contain the exact same bytes
        as are found in the backing file.

        If load is successful, the implementation should create a blob to hold
        the file's content, store it to `outBlob`, and return 0.
        If the load fails, the implementation should return a failure status
        (any negative value will do).
        */
        virtual SLANG_NO_THROW SlangResult SLANG_MCALL
        loadFile(char const* path, ISlangBlob** outBlob) = 0;
    };
#define SLANG_UUID_ISlangFileSystem ISlangFileSystem::getTypeGuid()


    typedef void (*SlangFuncPtr)(void);

    /**
    (DEPRECATED) ISlangSharedLibrary
    */
    struct ISlangSharedLibrary_Dep1 : public ISlangUnknown
    {
        SLANG_COM_INTERFACE(
            0x9c9d5bc5,
            0xeb61,
            0x496f,
            {0x80, 0xd7, 0xd1, 0x47, 0xc4, 0xa2, 0x37, 0x30})

        virtual SLANG_NO_THROW void* SLANG_MCALL findSymbolAddressByName(char const* name) = 0;
    };
#define SLANG_UUID_ISlangSharedLibrary_Dep1 ISlangSharedLibrary_Dep1::getTypeGuid()

    /** An interface that can be used to encapsulate access to a shared library. An implementation
    does not have to implement the library as a shared library
    */
    struct ISlangSharedLibrary : public ISlangCastable
    {
        SLANG_COM_INTERFACE(
            0x70dbc7c4,
            0xdc3b,
            0x4a07,
            {0xae, 0x7e, 0x75, 0x2a, 0xf6, 0xa8, 0x15, 0x55})

        /** Get a function by name. If the library is unloaded will only return nullptr.
        @param name The name of the function
        @return The function pointer related to the name or nullptr if not found
        */
        SLANG_FORCE_INLINE SlangFuncPtr findFuncByName(char const* name)
        {
            return (SlangFuncPtr)findSymbolAddressByName(name);
        }

        /** Get a symbol by name. If the library is unloaded will only return nullptr.
        @param name The name of the symbol
        @return The pointer related to the name or nullptr if not found
        */
        virtual SLANG_NO_THROW void* SLANG_MCALL findSymbolAddressByName(char const* name) = 0;
    };
#define SLANG_UUID_ISlangSharedLibrary ISlangSharedLibrary::getTypeGuid()

    struct ISlangSharedLibraryLoader : public ISlangUnknown
    {
        SLANG_COM_INTERFACE(
            0x6264ab2b,
            0xa3e8,
            0x4a06,
            {0x97, 0xf1, 0x49, 0xbc, 0x2d, 0x2a, 0xb1, 0x4d})

        /** Load a shared library. In typical usage the library name should *not* contain any
        platform specific elements. For example on windows a dll name should *not* be passed with a
        '.dll' extension, and similarly on linux a shared library should *not* be passed with the
        'lib' prefix and '.so' extension
        @path path The unadorned filename and/or path for the shared library
        @ param sharedLibraryOut Holds the shared library if successfully loaded */
        virtual SLANG_NO_THROW SlangResult SLANG_MCALL
        loadSharedLibrary(const char* path, ISlangSharedLibrary** sharedLibraryOut) = 0;
    };
#define SLANG_UUID_ISlangSharedLibraryLoader ISlangSharedLibraryLoader::getTypeGuid()

    /* Type that identifies how a path should be interpreted */
    typedef unsigned int SlangPathTypeIntegral;
    enum SlangPathType : SlangPathTypeIntegral
    {
        SLANG_PATH_TYPE_DIRECTORY = 0, /**< Path specified specifies a directory. */
        SLANG_PATH_TYPE_FILE = 1,      /**< Path specified is to a file. */
    };

    /* Callback to enumerate the contents of of a directory in a ISlangFileSystemExt.
    The name is the name of a file system object (directory/file) in the specified path (ie it is
    without a path) */
    typedef void (
        *FileSystemContentsCallBack)(SlangPathType pathType, const char* name, void* userData);

    /* Determines how paths map to files on the OS file system */
    enum class OSPathKind : uint8_t
    {
        None = 0,            ///< Paths do not map to the file system
        Direct = 1,          ///< Paths map directly to the file system
        OperatingSystem = 2, ///< Only paths gained via PathKind::OperatingSystem map to the
                             ///< operating system file system
    };

    /* Used to determine what kind of path is required from an input path */
    enum class PathKind
    {
        /// Given a path, returns a simplified version of that path.
        /// This typically means removing '..' and/or '.' from the path.
        /// A simplified path must point to the same object as the original.
        Simplified = 0,

        /// Given a path, returns a 'canonical path' to the item.
        /// This may be the operating system 'canonical path' that is the unique path to the item.
        ///
        /// If the item exists the returned canonical path should always be usable to access the
        /// item.
        ///
        /// If the item the path specifies doesn't exist, the canonical path may not be returnable
        /// or be a path simplification.
        /// Not all file systems support canonical paths.
        Canonical = 1,

        /// Given a path returns a path such that it is suitable to be displayed to the user.
        ///
        /// For example if the file system is a zip file - it might include the path to the zip
        /// container as well as the path to the specific file.
        ///
        /// NOTE! The display path won't necessarily work on the file system to access the item
        Display = 2,

        /// Get the path to the item on the *operating system* file system, if available.
        OperatingSystem = 3,

        CountOf,
    };

    /** An extended file system abstraction.

    Implementing and using this interface over ISlangFileSystem gives much more control over how
    paths are managed, as well as how it is determined if two files 'are the same'.

    All paths as input char*, or output as ISlangBlobs are always encoded as UTF-8 strings.
    Blobs that contain strings are always zero terminated.
    */
    struct ISlangFileSystemExt : public ISlangFileSystem
    {
        SLANG_COM_INTERFACE(
            0x5fb632d2,
            0x979d,
            0x4481,
            {0x9f, 0xee, 0x66, 0x3c, 0x3f, 0x14, 0x49, 0xe1})

        /** Get a uniqueIdentity which uniquely identifies an object of the file system.

        Given a path, returns a 'uniqueIdentity' which ideally is the same value for the same object
        on the file system.

        The uniqueIdentity is used to compare if two paths are the same - which amongst other things
        allows Slang to cache source contents internally. It is also used for #pragma once
        functionality.

        A *requirement* is for any implementation is that two paths can only return the same
        uniqueIdentity if the contents of the two files are *identical*. If an implementation breaks
        this constraint it can produce incorrect compilation. If an implementation cannot *strictly*
        identify *the same* files, this will only have an effect on #pragma once behavior.

        The string for the uniqueIdentity is held zero terminated in the ISlangBlob of
        outUniqueIdentity.

        Note that there are many ways a uniqueIdentity may be generated for a file. For example it
        could be the 'canonical path' - assuming it is available and unambiguous for a file system.
        Another possible mechanism could be to store the filename combined with the file date time
        to uniquely identify it.

        The client must ensure the blob be released when no longer used, otherwise memory will leak.

        NOTE! Ideally this method would be called 'getPathUniqueIdentity' but for historical reasons
        and backward compatibility it's name remains with 'File' even though an implementation
        should be made to work with directories too.

        @param path
        @param outUniqueIdentity
        @returns A `SlangResult` to indicate success or failure getting the uniqueIdentity.
        */
        virtual SLANG_NO_THROW SlangResult SLANG_MCALL
        getFileUniqueIdentity(const char* path, ISlangBlob** outUniqueIdentity) = 0;

        /** Calculate a path combining the 'fromPath' with 'path'

        The client must ensure the blob be released when no longer used, otherwise memory will leak.

        @param fromPathType How to interpret the from path - as a file or a directory.
        @param fromPath The from path.
        @param path Path to be determined relative to the fromPath
        @param pathOut Holds the string which is the relative path. The string is held in the blob
        zero terminated.
        @returns A `SlangResult` to indicate success or failure in loading the file.
        */
        virtual SLANG_NO_THROW SlangResult SLANG_MCALL calcCombinedPath(
            SlangPathType fromPathType,
            const char* fromPath,
            const char* path,
            ISlangBlob** pathOut) = 0;

        /** Gets the type of path that path is on the file system.
        @param path
        @param pathTypeOut
        @returns SLANG_OK if located and type is known, else an error. SLANG_E_NOT_FOUND if not
        found.
        */
        virtual SLANG_NO_THROW SlangResult SLANG_MCALL
        getPathType(const char* path, SlangPathType* pathTypeOut) = 0;

        /** Get a path based on the kind.

        @param kind The kind of path wanted
        @param path The input path
        @param outPath The output path held in a blob
        @returns SLANG_OK if successfully simplified the path (SLANG_E_NOT_IMPLEMENTED if not
        implemented, or some other error code)
        */
        virtual SLANG_NO_THROW SlangResult SLANG_MCALL
        getPath(PathKind kind, const char* path, ISlangBlob** outPath) = 0;

        /** Clears any cached information */
        virtual SLANG_NO_THROW void SLANG_MCALL clearCache() = 0;

        /** Enumerate the contents of the path

        Note that for normal Slang operation it isn't necessary to enumerate contents this can
        return SLANG_E_NOT_IMPLEMENTED.

        @param The path to enumerate
        @param callback This callback is called for each entry in the path.
        @param userData This is passed to the callback
        @returns SLANG_OK if successful
        */
        virtual SLANG_NO_THROW SlangResult SLANG_MCALL enumeratePathContents(
            const char* path,
            FileSystemContentsCallBack callback,
            void* userData) = 0;

        /** Returns how paths map to the OS file system

        @returns OSPathKind that describes how paths map to the Operating System file system
        */
        virtual SLANG_NO_THROW OSPathKind SLANG_MCALL getOSPathKind() = 0;
    };

#define SLANG_UUID_ISlangFileSystemExt ISlangFileSystemExt::getTypeGuid()

    struct ISlangMutableFileSystem : public ISlangFileSystemExt
    {
        SLANG_COM_INTERFACE(
            0xa058675c,
            0x1d65,
            0x452a,
            {0x84, 0x58, 0xcc, 0xde, 0xd1, 0x42, 0x71, 0x5})

        /** Write data to the specified path.

        @param path The path for data to be saved to
        @param data The data to be saved
        @param size The size of the data in bytes
        @returns SLANG_OK if successful (SLANG_E_NOT_IMPLEMENTED if not implemented, or some other
        error code)
        */
        virtual SLANG_NO_THROW SlangResult SLANG_MCALL
        saveFile(const char* path, const void* data, size_t size) = 0;

        /** Write data in the form of a blob to the specified path.

        Depending on the implementation writing a blob might be faster/use less memory. It is
        assumed the blob is *immutable* and that an implementation can reference count it.

        It is not guaranteed loading the same file will return the *same* blob - just a blob with
        same contents.

        @param path The path for data to be saved to
        @param dataBlob The data to be saved
        @returns SLANG_OK if successful (SLANG_E_NOT_IMPLEMENTED if not implemented, or some other
        error code)
        */
        virtual SLANG_NO_THROW SlangResult SLANG_MCALL
        saveFileBlob(const char* path, ISlangBlob* dataBlob) = 0;

        /** Remove the entry in the path (directory of file). Will only delete an empty directory,
        if not empty will return an error.

        @param path The path to remove
        @returns SLANG_OK if successful
        */
        virtual SLANG_NO_THROW SlangResult SLANG_MCALL remove(const char* path) = 0;

        /** Create a directory.

        The path to the directory must exist

        @param path To the directory to create. The parent path *must* exist otherwise will return
        an error.
        @returns SLANG_OK if successful
        */
        virtual SLANG_NO_THROW SlangResult SLANG_MCALL createDirectory(const char* path) = 0;
    };

#define SLANG_UUID_ISlangMutableFileSystem ISlangMutableFileSystem::getTypeGuid()

    /* Identifies different types of writer target*/
    typedef unsigned int SlangWriterChannelIntegral;
    enum SlangWriterChannel : SlangWriterChannelIntegral
    {
        SLANG_WRITER_CHANNEL_DIAGNOSTIC = 0,
        SLANG_WRITER_CHANNEL_STD_OUTPUT = 1,
        SLANG_WRITER_CHANNEL_STD_ERROR = 2,
        SLANG_WRITER_CHANNEL_COUNT_OF,
    };

    typedef unsigned int SlangWriterModeIntegral;
    enum SlangWriterMode : SlangWriterModeIntegral
    {
        SLANG_WRITER_MODE_TEXT = 0,
        SLANG_WRITER_MODE_BINARY = 1,
    };

    /** A stream typically of text, used for outputting diagnostic as well as other information.
     */
    struct ISlangWriter : public ISlangUnknown
    {
        SLANG_COM_INTERFACE(
            0xec457f0e,
            0x9add,
            0x4e6b,
            {0x85, 0x1c, 0xd7, 0xfa, 0x71, 0x6d, 0x15, 0xfd})

        /** Begin an append buffer.
        NOTE! Only one append buffer can be active at any time.
        @param maxNumChars The maximum of chars that will be appended
        @returns The start of the buffer for appending to. */
        virtual SLANG_NO_THROW char* SLANG_MCALL beginAppendBuffer(size_t maxNumChars) = 0;
        /** Ends the append buffer, and is equivalent to a write of the append buffer.
        NOTE! That an endAppendBuffer is not necessary if there are no characters to write.
        @param buffer is the start of the data to append and must be identical to last value
        returned from beginAppendBuffer
        @param numChars must be a value less than or equal to what was returned from last call to
        beginAppendBuffer
        @returns Result, will be SLANG_OK on success */
        virtual SLANG_NO_THROW SlangResult SLANG_MCALL
        endAppendBuffer(char* buffer, size_t numChars) = 0;
        /** Write text to the writer
        @param chars The characters to write out
        @param numChars The amount of characters
        @returns SLANG_OK on success */
        virtual SLANG_NO_THROW SlangResult SLANG_MCALL
        write(const char* chars, size_t numChars) = 0;
        /** Flushes any content to the output */
        virtual SLANG_NO_THROW void SLANG_MCALL flush() = 0;
        /** Determines if the writer stream is to the console, and can be used to alter the output
        @returns Returns true if is a console writer */
        virtual SLANG_NO_THROW SlangBool SLANG_MCALL isConsole() = 0;
        /** Set the mode for the writer to use
        @param mode The mode to use
        @returns SLANG_OK on success */
        virtual SLANG_NO_THROW SlangResult SLANG_MCALL setMode(SlangWriterMode mode) = 0;
    };

#define SLANG_UUID_ISlangWriter ISlangWriter::getTypeGuid()

    struct ISlangProfiler : public ISlangUnknown
    {
        SLANG_COM_INTERFACE(
            0x197772c7,
            0x0155,
            0x4b91,
            {0x84, 0xe8, 0x66, 0x68, 0xba, 0xff, 0x06, 0x19})
        virtual SLANG_NO_THROW size_t SLANG_MCALL getEntryCount() = 0;
        virtual SLANG_NO_THROW const char* SLANG_MCALL getEntryName(uint32_t index) = 0;
        virtual SLANG_NO_THROW long SLANG_MCALL getEntryTimeMS(uint32_t index) = 0;
        virtual SLANG_NO_THROW uint32_t SLANG_MCALL getEntryInvocationTimes(uint32_t index) = 0;
    };
#define SLANG_UUID_ISlangProfiler ISlangProfiler::getTypeGuid()

    namespace slang
    {
    struct IGlobalSession;
    struct ICompileRequest;

    } // namespace slang

    /*!
    @brief An instance of the Slang library.
    */
    typedef slang::IGlobalSession SlangSession;


    typedef struct SlangProgramLayout SlangProgramLayout;

    /*!
    @brief A request for one or more compilation actions to be performed.
    */
    typedef struct slang::ICompileRequest SlangCompileRequest;


    /*!
@brief Callback type used for diagnostic output.
*/
    typedef void (*SlangDiagnosticCallback)(char const* message, void* userData);

    /*!
    @brief Get the build version 'tag' string. The string is the same as
    produced via `git describe --tags --match v*` for the project. If such a
    version could not be determined at build time then the contents will be
    0.0.0-unknown. Any string can be set by passing
    -DSLANG_VERSION_FULL=whatever during the cmake invocation.

    This function will return exactly the same result as the method
    getBuildTagString on IGlobalSession.

    An advantage of using this function over the method is that doing so does
    not require the creation of a session, which can be a fairly costly
    operation.

    @return The build tag string
    */
    SLANG_API const char* spGetBuildTagString();

    /*
    Forward declarations of types used in the reflection interface;
    */

    typedef struct SlangProgramLayout SlangProgramLayout;
    typedef struct SlangEntryPoint SlangEntryPoint;
    typedef struct SlangEntryPointLayout SlangEntryPointLayout;

    typedef struct SlangReflectionDecl SlangReflectionDecl;
    typedef struct SlangReflectionModifier SlangReflectionModifier;
    typedef struct SlangReflectionType SlangReflectionType;
    typedef struct SlangReflectionTypeLayout SlangReflectionTypeLayout;
    typedef struct SlangReflectionVariable SlangReflectionVariable;
    typedef struct SlangReflectionVariableLayout SlangReflectionVariableLayout;
    typedef struct SlangReflectionTypeParameter SlangReflectionTypeParameter;
    typedef struct SlangReflectionUserAttribute SlangReflectionUserAttribute;
    typedef SlangReflectionUserAttribute SlangReflectionAttribute;
    typedef struct SlangReflectionFunction SlangReflectionFunction;
    typedef struct SlangReflectionGeneric SlangReflectionGeneric;

    union SlangReflectionGenericArg
    {
        SlangReflectionType* typeVal;
        int64_t intVal;
        bool boolVal;
    };

    enum SlangReflectionGenericArgType
    {
        SLANG_GENERIC_ARG_TYPE = 0,
        SLANG_GENERIC_ARG_INT = 1,
        SLANG_GENERIC_ARG_BOOL = 2
    };

    /*
    Type aliases to maintain backward compatibility.
    */
    typedef SlangProgramLayout SlangReflection;
    typedef SlangEntryPointLayout SlangReflectionEntryPoint;

    // type reflection

    typedef unsigned int SlangTypeKindIntegral;
    enum SlangTypeKind : SlangTypeKindIntegral
    {
        SLANG_TYPE_KIND_NONE = 0,
        SLANG_TYPE_KIND_STRUCT = 1,
        SLANG_TYPE_KIND_ARRAY = 2,
        SLANG_TYPE_KIND_MATRIX = 3,
        SLANG_TYPE_KIND_VECTOR = 4,
        SLANG_TYPE_KIND_SCALAR = 5,
        SLANG_TYPE_KIND_CONSTANT_BUFFER = 6,
        SLANG_TYPE_KIND_RESOURCE = 7,
        SLANG_TYPE_KIND_SAMPLER_STATE = 8,
        SLANG_TYPE_KIND_TEXTURE_BUFFER = 9,
        SLANG_TYPE_KIND_SHADER_STORAGE_BUFFER = 10,
        SLANG_TYPE_KIND_PARAMETER_BLOCK = 11,
        SLANG_TYPE_KIND_GENERIC_TYPE_PARAMETER = 12,
        SLANG_TYPE_KIND_INTERFACE = 13,
        SLANG_TYPE_KIND_OUTPUT_STREAM = 14,
        SLANG_TYPE_KIND_MESH_OUTPUT = 15,
        SLANG_TYPE_KIND_SPECIALIZED = 16,
        SLANG_TYPE_KIND_FEEDBACK = 17,
        SLANG_TYPE_KIND_POINTER = 18,
        SLANG_TYPE_KIND_DYNAMIC_RESOURCE = 19,
        SLANG_TYPE_KIND_ENUM = 20,
        SLANG_TYPE_KIND_COUNT,
    };

    typedef unsigned int SlangScalarTypeIntegral;
    enum SlangScalarType : SlangScalarTypeIntegral
    {
        SLANG_SCALAR_TYPE_NONE = 0,
        SLANG_SCALAR_TYPE_VOID = 1,
        SLANG_SCALAR_TYPE_BOOL = 2,
        SLANG_SCALAR_TYPE_INT32 = 3,
        SLANG_SCALAR_TYPE_UINT32 = 4,
        SLANG_SCALAR_TYPE_INT64 = 5,
        SLANG_SCALAR_TYPE_UINT64 = 6,
        SLANG_SCALAR_TYPE_FLOAT16 = 7,
        SLANG_SCALAR_TYPE_FLOAT32 = 8,
        SLANG_SCALAR_TYPE_FLOAT64 = 9,
        SLANG_SCALAR_TYPE_INT8 = 10,
        SLANG_SCALAR_TYPE_UINT8 = 11,
        SLANG_SCALAR_TYPE_INT16 = 12,
        SLANG_SCALAR_TYPE_UINT16 = 13,
        SLANG_SCALAR_TYPE_INTPTR = 14,
        SLANG_SCALAR_TYPE_UINTPTR = 15,
        SLANG_SCALAR_TYPE_BFLOAT16 = 16,
        SLANG_SCALAR_TYPE_FLOAT_E4M3 = 17,
        SLANG_SCALAR_TYPE_FLOAT_E5M2 = 18,
    };

    // abstract decl reflection
    typedef unsigned int SlangDeclKindIntegral;
    enum SlangDeclKind : SlangDeclKindIntegral
    {
        SLANG_DECL_KIND_UNSUPPORTED_FOR_REFLECTION = 0,
        SLANG_DECL_KIND_STRUCT = 1,
        SLANG_DECL_KIND_FUNC = 2,
        SLANG_DECL_KIND_MODULE = 3,
        SLANG_DECL_KIND_GENERIC = 4,
        SLANG_DECL_KIND_VARIABLE = 5,
        SLANG_DECL_KIND_NAMESPACE = 6,
        SLANG_DECL_KIND_ENUM = 7,
    };

#ifndef SLANG_RESOURCE_SHAPE
    #define SLANG_RESOURCE_SHAPE
    typedef unsigned int SlangResourceShapeIntegral;
    enum SlangResourceShape : SlangResourceShapeIntegral
    {
        SLANG_RESOURCE_BASE_SHAPE_MASK = 0x0F,

        SLANG_RESOURCE_NONE = 0x00,

        SLANG_TEXTURE_1D = 0x01,
        SLANG_TEXTURE_2D = 0x02,
        SLANG_TEXTURE_3D = 0x03,
        SLANG_TEXTURE_CUBE = 0x04,
        SLANG_TEXTURE_BUFFER = 0x05,

        SLANG_STRUCTURED_BUFFER = 0x06,
        SLANG_BYTE_ADDRESS_BUFFER = 0x07,
        SLANG_RESOURCE_UNKNOWN = 0x08,
        SLANG_ACCELERATION_STRUCTURE = 0x09,
        SLANG_TEXTURE_SUBPASS = 0x0A,

        SLANG_RESOURCE_EXT_SHAPE_MASK = 0x1F0,

        SLANG_TEXTURE_FEEDBACK_FLAG = 0x10,
        SLANG_TEXTURE_SHADOW_FLAG = 0x20,
        SLANG_TEXTURE_ARRAY_FLAG = 0x40,
        SLANG_TEXTURE_MULTISAMPLE_FLAG = 0x80,
        SLANG_TEXTURE_COMBINED_FLAG = 0x100,

        SLANG_TEXTURE_1D_ARRAY = SLANG_TEXTURE_1D | SLANG_TEXTURE_ARRAY_FLAG,
        SLANG_TEXTURE_2D_ARRAY = SLANG_TEXTURE_2D | SLANG_TEXTURE_ARRAY_FLAG,
        SLANG_TEXTURE_CUBE_ARRAY = SLANG_TEXTURE_CUBE | SLANG_TEXTURE_ARRAY_FLAG,

        SLANG_TEXTURE_2D_MULTISAMPLE = SLANG_TEXTURE_2D | SLANG_TEXTURE_MULTISAMPLE_FLAG,
        SLANG_TEXTURE_2D_MULTISAMPLE_ARRAY =
            SLANG_TEXTURE_2D | SLANG_TEXTURE_MULTISAMPLE_FLAG | SLANG_TEXTURE_ARRAY_FLAG,
        SLANG_TEXTURE_SUBPASS_MULTISAMPLE = SLANG_TEXTURE_SUBPASS | SLANG_TEXTURE_MULTISAMPLE_FLAG,
    };
#endif
    typedef unsigned int SlangResourceAccessIntegral;
    enum SlangResourceAccess : SlangResourceAccessIntegral
    {
        SLANG_RESOURCE_ACCESS_NONE = 0,
        SLANG_RESOURCE_ACCESS_READ = 1,
        SLANG_RESOURCE_ACCESS_READ_WRITE = 2,
        SLANG_RESOURCE_ACCESS_RASTER_ORDERED = 3,
        SLANG_RESOURCE_ACCESS_APPEND = 4,
        SLANG_RESOURCE_ACCESS_CONSUME = 5,
        SLANG_RESOURCE_ACCESS_WRITE = 6,
        SLANG_RESOURCE_ACCESS_FEEDBACK = 7,
        SLANG_RESOURCE_ACCESS_UNKNOWN = 0x7FFFFFFF,
    };

    typedef unsigned int SlangParameterCategoryIntegral;
    enum SlangParameterCategory : SlangParameterCategoryIntegral
    {
        SLANG_PARAMETER_CATEGORY_NONE = 0,
        SLANG_PARAMETER_CATEGORY_MIXED = 1,
        SLANG_PARAMETER_CATEGORY_CONSTANT_BUFFER = 2,
        SLANG_PARAMETER_CATEGORY_SHADER_RESOURCE = 3,
        SLANG_PARAMETER_CATEGORY_UNORDERED_ACCESS = 4,
        SLANG_PARAMETER_CATEGORY_VARYING_INPUT = 5,
        SLANG_PARAMETER_CATEGORY_VARYING_OUTPUT = 6,
        SLANG_PARAMETER_CATEGORY_SAMPLER_STATE = 7,
        SLANG_PARAMETER_CATEGORY_UNIFORM = 8,
        SLANG_PARAMETER_CATEGORY_DESCRIPTOR_TABLE_SLOT = 9,
        SLANG_PARAMETER_CATEGORY_SPECIALIZATION_CONSTANT = 10,
        SLANG_PARAMETER_CATEGORY_PUSH_CONSTANT_BUFFER = 11,

        // HLSL register `space`, Vulkan GLSL `set`
        SLANG_PARAMETER_CATEGORY_REGISTER_SPACE = 12,

        // TODO: Ellie, Both APIs treat mesh outputs as more or less varying output,
        // Does it deserve to be represented here??

        // A parameter whose type is to be specialized by a global generic type argument
        SLANG_PARAMETER_CATEGORY_GENERIC = 13,

        SLANG_PARAMETER_CATEGORY_RAY_PAYLOAD = 14,
        SLANG_PARAMETER_CATEGORY_HIT_ATTRIBUTES = 15,
        SLANG_PARAMETER_CATEGORY_CALLABLE_PAYLOAD = 16,
        SLANG_PARAMETER_CATEGORY_SHADER_RECORD = 17,

        // An existential type parameter represents a "hole" that
        // needs to be filled with a concrete type to enable
        // generation of specialized code.
        //
        // Consider this example:
        //
        //      struct MyParams
        //      {
        //          IMaterial material;
        //          ILight lights[3];
        //      };
        //
        // This `MyParams` type introduces two existential type parameters:
        // one for `material` and one for `lights`. Even though `lights`
        // is an array, it only introduces one type parameter, because
        // we need to have a *single* concrete type for all the array
        // elements to be able to generate specialized code.
        //
        SLANG_PARAMETER_CATEGORY_EXISTENTIAL_TYPE_PARAM = 18,

        // An existential object parameter represents a value
        // that needs to be passed in to provide data for some
        // interface-type shader parameter.
        //
        // Consider this example:
        //
        //      struct MyParams
        //      {
        //          IMaterial material;
        //          ILight lights[3];
        //      };
        //
        // This `MyParams` type introduces four existential object parameters:
        // one for `material` and three for `lights` (one for each array
        // element). This is consistent with the number of interface-type
        // "objects" that are being passed through to the shader.
        //
        SLANG_PARAMETER_CATEGORY_EXISTENTIAL_OBJECT_PARAM = 19,

        // The register space offset for the sub-elements that occupies register spaces.
        SLANG_PARAMETER_CATEGORY_SUB_ELEMENT_REGISTER_SPACE = 20,

        // The input_attachment_index subpass occupancy tracker
        SLANG_PARAMETER_CATEGORY_SUBPASS = 21,

        // Metal tier-1 argument buffer element [[id]].
        SLANG_PARAMETER_CATEGORY_METAL_ARGUMENT_BUFFER_ELEMENT = 22,

        // Metal [[attribute]] inputs.
        SLANG_PARAMETER_CATEGORY_METAL_ATTRIBUTE = 23,

        // Metal [[payload]] inputs
        SLANG_PARAMETER_CATEGORY_METAL_PAYLOAD = 24,

        //
        SLANG_PARAMETER_CATEGORY_COUNT,

        // Aliases for Metal-specific categories.
        SLANG_PARAMETER_CATEGORY_METAL_BUFFER = SLANG_PARAMETER_CATEGORY_CONSTANT_BUFFER,
        SLANG_PARAMETER_CATEGORY_METAL_TEXTURE = SLANG_PARAMETER_CATEGORY_SHADER_RESOURCE,
        SLANG_PARAMETER_CATEGORY_METAL_SAMPLER = SLANG_PARAMETER_CATEGORY_SAMPLER_STATE,

        // DEPRECATED:
        SLANG_PARAMETER_CATEGORY_VERTEX_INPUT = SLANG_PARAMETER_CATEGORY_VARYING_INPUT,
        SLANG_PARAMETER_CATEGORY_FRAGMENT_OUTPUT = SLANG_PARAMETER_CATEGORY_VARYING_OUTPUT,
        SLANG_PARAMETER_CATEGORY_COUNT_V1 = SLANG_PARAMETER_CATEGORY_SUBPASS,
    };

    /** Types of API-managed bindings that a parameter might use.

    `SlangBindingType` represents the distinct types of binding ranges that might be
    understood by an underlying graphics API or cross-API abstraction layer.
    Several of the enumeration cases here correspond to cases of `VkDescriptorType`
    defined by the Vulkan API. Note however that the values of this enumeration
    are not the same as those of any particular API.

    The `SlangBindingType` enumeration is distinct from `SlangParameterCategory`
    because `SlangParameterCategory` differentiates the types of parameters for
    the purposes of layout, where the layout rules of some targets will treat
    parameters of different types as occupying the same binding space for layout
    (e.g., in SPIR-V both a `Texture2D` and `SamplerState` use the same space of
    `binding` indices, and are not allowed to overlap), while those same types
    map to different types of bindings in the API (e.g., both textures and samplers
    use different `VkDescriptorType` values).

    When you want to answer "what register/binding did this parameter use?" you
    should use `SlangParameterCategory`.

    When you want to answer "what type of descriptor range should this parameter use?"
    you should use `SlangBindingType`.
    */
    typedef SlangUInt32 SlangBindingTypeIntegral;
    enum SlangBindingType : SlangBindingTypeIntegral
    {
        SLANG_BINDING_TYPE_UNKNOWN = 0,

        SLANG_BINDING_TYPE_SAMPLER = 1,
        SLANG_BINDING_TYPE_TEXTURE = 2,
        SLANG_BINDING_TYPE_CONSTANT_BUFFER = 3,
        SLANG_BINDING_TYPE_PARAMETER_BLOCK = 4,
        SLANG_BINDING_TYPE_TYPED_BUFFER = 5,
        SLANG_BINDING_TYPE_RAW_BUFFER = 6,
        SLANG_BINDING_TYPE_COMBINED_TEXTURE_SAMPLER = 7,
        SLANG_BINDING_TYPE_INPUT_RENDER_TARGET = 8,
        SLANG_BINDING_TYPE_INLINE_UNIFORM_DATA = 9,
        SLANG_BINDING_TYPE_RAY_TRACING_ACCELERATION_STRUCTURE = 10,

        SLANG_BINDING_TYPE_VARYING_INPUT = 11,
        SLANG_BINDING_TYPE_VARYING_OUTPUT = 12,

        SLANG_BINDING_TYPE_EXISTENTIAL_VALUE = 13,
        SLANG_BINDING_TYPE_PUSH_CONSTANT = 14,

        SLANG_BINDING_TYPE_MUTABLE_FLAG = 0x100,

        SLANG_BINDING_TYPE_MUTABLE_TETURE =
            SLANG_BINDING_TYPE_TEXTURE | SLANG_BINDING_TYPE_MUTABLE_FLAG,
        SLANG_BINDING_TYPE_MUTABLE_TYPED_BUFFER =
            SLANG_BINDING_TYPE_TYPED_BUFFER | SLANG_BINDING_TYPE_MUTABLE_FLAG,
        SLANG_BINDING_TYPE_MUTABLE_RAW_BUFFER =
            SLANG_BINDING_TYPE_RAW_BUFFER | SLANG_BINDING_TYPE_MUTABLE_FLAG,

        SLANG_BINDING_TYPE_BASE_MASK = 0x00FF,
        SLANG_BINDING_TYPE_EXT_MASK = 0xFF00,
    };

    typedef SlangUInt32 SlangLayoutRulesIntegral;
    enum SlangLayoutRules : SlangLayoutRulesIntegral
    {
        SLANG_LAYOUT_RULES_DEFAULT = 0,
        SLANG_LAYOUT_RULES_METAL_ARGUMENT_BUFFER_TIER_2 = 1,
        SLANG_LAYOUT_RULES_DEFAULT_STRUCTURED_BUFFER = 2,
        SLANG_LAYOUT_RULES_DEFAULT_CONSTANT_BUFFER = 3,
    };

    typedef SlangUInt32 SlangModifierIDIntegral;
    enum SlangModifierID : SlangModifierIDIntegral
    {
        SLANG_MODIFIER_SHARED = 0,
        SLANG_MODIFIER_NO_DIFF = 1,
        SLANG_MODIFIER_STATIC = 2,
        SLANG_MODIFIER_CONST = 3,
        SLANG_MODIFIER_EXPORT = 4,
        SLANG_MODIFIER_EXTERN = 5,
        SLANG_MODIFIER_DIFFERENTIABLE = 6,
        SLANG_MODIFIER_MUTATING = 7,
        SLANG_MODIFIER_IN = 8,
        SLANG_MODIFIER_OUT = 9,
        SLANG_MODIFIER_INOUT = 10,
    };

    typedef SlangUInt32 SlangImageFormatIntegral;
    enum SlangImageFormat : SlangImageFormatIntegral
    {
#define SLANG_FORMAT(NAME, DESC) SLANG_IMAGE_FORMAT_##NAME,
#include "slang-image-format-defs.h"
#undef SLANG_FORMAT
    };

#define SLANG_UNBOUNDED_SIZE (~size_t(0))
#define SLANG_UNKNOWN_SIZE (SLANG_UNBOUNDED_SIZE - 1)

    // Shader Parameter Reflection

    typedef SlangReflectionVariableLayout SlangReflectionParameter;

#ifdef __cplusplus
}
#endif

#ifdef __cplusplus
namespace slang
{
struct ISession;
}
#endif

#define SLANG_ERROR_INSUFFICIENT_BUFFER SLANG_E_BUFFER_TOO_SMALL
#define SLANG_ERROR_INVALID_PARAMETER SLANG_E_INVALID_ARG

#endif
