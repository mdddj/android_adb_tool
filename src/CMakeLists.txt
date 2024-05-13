# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

project(android_adb_tool_library VERSION 0.0.1 LANGUAGES C)

add_library(android_adb_tool SHARED
  "android_adb_tool.c"
)

set_target_properties(android_adb_tool PROPERTIES
  PUBLIC_HEADER android_adb_tool.h
  OUTPUT_NAME "android_adb_tool"
)

target_compile_definitions(android_adb_tool PUBLIC DART_SHARED_LIB)
