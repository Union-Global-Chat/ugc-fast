from . import ugc_fast
import asyncio


class FastProtocol:
    def __init__(self, loop: asyncio.AbstractEventLoop):
        self.protocol = ugc_fast.BaseFastProtocol()
        self.loop = loop

    async def connect(self):
        await self.loop.run_in_executor(None, self.protocol.connect())

    async def send(self, data: bytes):
        await self.loop.run_in_executor(None, self.protocol.send, data)