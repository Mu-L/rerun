cmake_minimum_required(VERSION 3.16)

project(rerun_cpp_proj LANGUAGES CXX)

function(set_default_warning_settings target)
    if(MSVC)
        target_compile_options(${target} PRIVATE /W4 /WX)
    else()
        # Enabled warnings.
        target_compile_options(${target} PRIVATE
            -Wall
            -Wcast-align
            -Wcast-qual
            -Wextra
            -Wformat=2
            -Wmissing-include-dirs
            -Wnull-dereference
            -Wold-style-cast
            -Wpedantic
            -Wpointer-arith
            -Wshadow
            -Wswitch-enum
            -Wvla
        )

        if(CMAKE_CXX_COMPILER_ID MATCHES "Clang") # match both "Clang" and "AppleClang"
            target_compile_options(${target} PRIVATE
                -Wc++17-compat-pedantic
                -Wc99-extensions
                -Wgnu
                -Wnon-gcc
                -Wshadow-all
            )
        endif()

        # Disabled warnings
        # arrow has a bunch of unused parameters in its headers.
        target_compile_options(${target} PRIVATE -Wno-unused-parameter)

        # CMAKE_COMPILE_WARNING_AS_ERROR is only directly supported starting in CMake `3.24`
        # https://cmake.org/cmake/help/latest/prop_tgt/COMPILE_WARNING_AS_ERROR.html
        if(${CMAKE_COMPILE_WARNING_AS_ERROR})
            target_compile_options(${target} PRIVATE -Werror)
        endif()

        if(CMAKE_BUILD_TYPE STREQUAL "Debug")
            # Debug build with fsanatize:
            target_compile_options(${target} PRIVATE -g -fno-omit-frame-pointer -mno-omit-leaf-frame-pointer -fno-optimize-sibling-calls)
            target_compile_options(${target} PRIVATE -fsanitize=address)
            target_link_options(${target} PRIVATE -fsanitize=address)
        endif()
    endif()
endfunction()

# Use makefiles on linux, otherwise it might use Ninja which isn't installed by default.
if(NOT DEFINED CMAKE_GENERATOR AND UNIX)
    set(CMAKE_GENERATOR "Unix Makefiles")
endif()

# Arrow requires a C++17 compliant compiler.
if(NOT DEFINED CMAKE_CXX_STANDARD)
    set(CMAKE_CXX_STANDARD 17)
endif()

# Signal to all our build scripts that we're inside the Rerun repository.
set(RERUN_REPOSITORY YES)

# Set version number.
# Read it from the Rust toml with a regex so we don't have another place to maintain!
file(READ "Cargo.toml" CARGO_TOML)
string(REGEX MATCH "\nversion = \"([a-z0-9\.\+-]+)\"\n" _ ${CARGO_TOML})
set(RERUN_VERSION ${CMAKE_MATCH_1})
message(INFO "Rerun version ${RERUN_VERSION}")

# ------------------------------------------------------------------------------
# Loguru logging library (https://github.com/emilk/loguru):
set(CMAKE_DL_LIBS "dl") # Required by Loguru for backtraces

# Loguru, see https://github.com/emilk/loguru/blob/master/loguru_cmake_example/CMakeLists.txt
include(FetchContent)
FetchContent_Declare(LoguruGitRepo
    GIT_REPOSITORY "https://github.com/emilk/loguru" # can be a filesystem path
    GIT_TAG "master"
)

# set any loguru compile-time flags before calling MakeAvailable()
set(LOGURU_STACKTRACES 1)
FetchContent_MakeAvailable(LoguruGitRepo) # defines target 'loguru::loguru'

# ------------------------------------------------------------------------------
add_subdirectory(rerun_cpp) # The Rerun C++ SDK library
add_subdirectory(examples/cpp)
add_subdirectory(tests/cpp)
add_subdirectory(docs/code-examples)
