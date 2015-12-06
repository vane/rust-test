#!/usr/bin/env python
# -*- coding: utf-8 -*-
__author__ = 'Michal Szczepanski'

from ctypes import cdll

lib = cdll.LoadLibrary('target/release/libhelloworld.dylib')
lib.process()

print('ok')