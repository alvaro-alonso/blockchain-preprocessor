import asyncio
from test import PerformanceTest
 
tests = {
    "trial_ten_signatures": {
        "proof_id": "F9E7A5265933158FA7CBE22FEF23EB787C707DAB08929C8272EB38D5F2E69AC9",
        "name": "ten_signatures",
        "dest": "http://0.0.0.0:8000",
        "chunk_size": 10,
        "head": 10,
    },
}


if __name__ == '__main__':
    test_config = tests["trial_ten_signatures"]
    test = PerformanceTest(**test_config)
    loop = asyncio.get_event_loop()
    future = asyncio.ensure_future(test.run())
    loop.run_until_complete(future)
    # asyncio.run(test.run())