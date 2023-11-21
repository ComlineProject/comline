# Generated with Comline compiler and code generator

import typing
import ctypes


LOW_PING_RATE: typing.Final[ctypes.c_uint16] = 20

class Ping(typing.Protocol):
	def ping(self) -> ctypes.c_bool:
		...

	def ping_limit(self) -> ctypes.c_bool:
		...


