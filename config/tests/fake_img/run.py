#!/usr/bin/env python

import os
import asyncio

from viam.robot.client import RobotClient
from viam.rpc.dial import DialOptions
from viam.components.camera import Camera


async def connect():
    opts = RobotClient.Options(
        refresh_interval=0,
        dial_options=DialOptions(insecure=True)
    )
    return await RobotClient.at_address('localhost:8080', opts)

async def main():
    robot = await connect()

    print('Resources:')
    print(robot.resource_names)
    
    # test
    test = Camera.from_robot(robot, "camera1")
    image = await test.get_image(mime_type="image/png")

    print(f"test get_image return value: {image}")

    image.save(os.path.expandvars("$HTP_PERSIST/out.png"))
    

    await robot.close()

if __name__ == '__main__':
    asyncio.run(main())
