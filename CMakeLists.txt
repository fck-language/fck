cmake_minimum_required(VERSION 3.10)

#set project name
project(fck VERSION 0.1.0)

include_directories(${PROJECT_SOURCE_DIR})

add_executable(fck main.cpp bases.cpp results.cpp errors.cpp)
