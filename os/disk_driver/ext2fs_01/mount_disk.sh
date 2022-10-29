#!/bin/bash
# src: http://shihyu.github.io/books/ch29s02.htmls


TARGET_FOLDER=diskfs
IS_EXIST=$(ls | grep $TARGET_FOLDER | wc -l)

if [ $IS_EXIST -eq 1 ];
then
    echo "file exist: ./diskfs"
else
    mkdir diskfs
    echo "create a new file: ./diskfs"
fi

sudo mount -o loop disk.img ./diskfs