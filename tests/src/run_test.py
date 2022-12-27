import os
import asyncio
from test import PerformanceTest
 
tests = {
    "trial_ten_signatures": {
        "proof_id": "F9E7A5265933158FA7CBE22FEF23EB787C707DAB08929C8272EB38D5F2E69AC9",
        "name": "trial_ten_signatures",
        "dest": "http://0.0.0.0:8000",
        "chunk_size": 10,
        "head": 10,
    },
    "trial_five_signatures": {
        "proof_id": "BDBA557991EC3F5415FC3362D9C7CBD12B78CDE47875B1CB637C4514F52AA08B",
        "name": "trial_five_signatures",
        "dest": "http://0.0.0.0:8000",
        "chunk_size": 5,
        "head": 5,
    },
     "trial_one_signature": {
        "proof_id": "935C49E1AE1F05916BBD2B7053182F3D99C0D26DD6BD601145E5B6FECE872D62",
        "name": "trial_one_signature",
        "dest": "http://0.0.0.0:8000",
        "chunk_size": 1,
        "head": 1,
    },
    "input_num_ten_signatures": {
        "proof_id": "F9E7A5265933158FA7CBE22FEF23EB787C707DAB08929C8272EB38D5F2E69AC9",
        "name": "input_num_ten_signatures",
        "dest": os.getenv("CLUSTER_ADDRESS"),
        "chunk_size": 10,
        "head": 1000,
    },
    "input_num_five_signatures": {
        "proof_id": "BDBA557991EC3F5415FC3362D9C7CBD12B78CDE47875B1CB637C4514F52AA08B",
        "name": "input_num_five_signatures",
        "dest": os.getenv("CLUSTER_ADDRESS"),
        "chunk_size": 5,
        "head": 1000,
    },
     "input_num_one_signature": {
        "proof_id": "935C49E1AE1F05916BBD2B7053182F3D99C0D26DD6BD601145E5B6FECE872D62",
        "name": "input_num_one_signature",
        "dest": os.getenv("CLUSTER_ADDRESS"),
        "chunk_size": 1,
        "head": 1000,
    },
        "dataset_signatures": {
        "proof_id": "935C49E1AE1F05916BBD2B7053182F3D99C0D26DD6BD601145E5B6FECE872D62",
        "name": "dataset_signatures",
        "dest": os.getenv("CLUSTER_ADDRESS"),
        "chunk_size": 10,
    },
}

if __name__ == '__main__':
    test_config = tests["input_num_one_signature"]
    test = PerformanceTest(**test_config)
    loop = asyncio.get_event_loop()
    future = asyncio.ensure_future(test.run())
    loop.run_until_complete(future)