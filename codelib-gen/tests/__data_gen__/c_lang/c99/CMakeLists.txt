cmake_minimum_required(VERSION 3.25)

project(comline_rs C)

set(CMAKE_C_STANDARD 99)

include_directories(.)

add_executable(comline_rs
        test.c
        test.h)
