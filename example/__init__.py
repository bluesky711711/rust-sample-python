from example._native import ffi, lib


def test():
    return lib.read_xlsx()