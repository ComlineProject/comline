# Generated with Comline compiler and code generator

import typing
import ctypes


class HealthCheck(typing.Protocol):
	def alive(self) -> ctypes.c_bool:
		...

	def capabilities(self) -> Capabilities:
		...


