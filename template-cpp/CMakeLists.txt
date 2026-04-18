cmake_minimum_required(VERSION 3.10)

# Enable C++17. You may raise the standard if you want
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(PSEMolDyn VERSION 0.0.1 LANGUAGES CXX)

# collect all cpp files
file(GLOB_RECURSE MY_SRC
    "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp"
    # header don't need to be included but this might be necessary for some IDEs
    "${CMAKE_CURRENT_SOURCE_DIR}/src/*.h"
)

# create make target
add_executable(MolSim ${MY_SRC})

target_include_directories(MolSim
        PRIVATE
            ${CMAKE_CURRENT_SOURCE_DIR}/src
)
