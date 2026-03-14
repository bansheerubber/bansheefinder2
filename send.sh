#!/bin/env bash

rsync -ia --info=progress2 --exclude=target /home/me/Projects/bansheefinder3/ $1:/home/me/Projects/bansheefinder3/
