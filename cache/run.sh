#!/bin/bash 

g++ linked_list_class.cpp -o linked_list_class
./linked_list_class

echo "-----------------------------"

g++ linked_list_struct.cpp -o linked_list_struct
./linked_list_struct

echo "-----------------------------"

g++ lru_cache.cpp -o lru_cache
./lru_cache