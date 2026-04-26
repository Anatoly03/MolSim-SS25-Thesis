cmake_minimum_required(VERSION 3.10)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(MolDyn VERSION 0.0.1 LANGUAGES CXX)

file(GLOB_RECURSE CLI_SRC "${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-cli/*.cpp")
file(GLOB_RECURSE TEST_SRC "${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-test/*.cc")
file(GLOB_RECURSE CORE_SRC "${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-core/*.cpp")
file(GLOB_RECURSE IO_SRC "${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-io/*.cpp")

list(APPEND CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/.cmake)

# Library for the core molecular dynamics simulation logic
add_library(MolSimCore ${CORE_SRC})
target_include_directories(MolSimCore PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-core)

# Executable for the command-line interface application
add_executable(MolSim ${CLI_SRC})
target_link_libraries(MolSim PRIVATE MolSimCore)
target_include_directories(MolSim PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-cli)

# Executable for the test runner
add_executable(MolSimTest ${TEST_SRC})
target_link_libraries(MolSimTest gtest gtest_main MolSimCore)
target_include_directories(MolSimTest PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/cpp-moldyn-test)

# CMake Dependency Configurations
include(test-google)
