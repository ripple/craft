cmake_minimum_required(VERSION 3.16)
project(xrpld-number)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Add include directory
include_directories(src)

# Define the source files
set(SOURCES
    src/Number.cpp
    src/number_c.cpp
)

set(HEADERS
    src/Number.h
    src/number_c.h
)

# Check for MSVC to add boost dependency conditionally
if(MSVC)
    find_package(Boost REQUIRED)
    target_link_libraries(xrpld-number PRIVATE Boost::boost)
endif()

# Create a static library
add_library(xrpld-number STATIC ${SOURCES} ${HEADERS})

# Set compiler flags
if(MSVC)
    target_compile_options(xrpld-number PRIVATE /W4)
else()
    target_compile_options(xrpld-number PRIVATE -Wall -Wextra -Wpedantic)
endif()

# Make headers available to other targets
target_include_directories(xrpld-number PUBLIC src)
