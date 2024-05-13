cmake_minimum_required(VERSION 3.10)
set(PROJECT_NAME "android_adb_tool")
project(${PROJECT_NAME} LANGUAGES CXX)
include("../cargokit/cmake/cargokit.cmake")
apply_cargokit(${PROJECT_NAME} ../rust android_adb_tool "")
set(android_adb_tool_bundled_libraries
  "${${PROJECT_NAME}_cargokit_lib}"
  PARENT_SCOPE
)