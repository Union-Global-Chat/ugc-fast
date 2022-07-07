from ugc_fast import FastProtocol
from ugc_sdk import Client
import asyncio


async def main():
    loop = asyncio.get_running_loop()
    client = Client(protocol=FastProtocol(loop), loop=loop)
    await client.connect()

asyncio.run(main())