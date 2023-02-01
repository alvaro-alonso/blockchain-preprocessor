import os
import asyncio
import sys
from test import PerformanceTest
 
tests = {
    "trial_ten_signatures": {
        "proof_id": "F9E7A5265933158FA7CBE22FEF23EB787C707DAB08929C8272EB38D5F2E69AC9",
        "name": "trial_ten_signatures",
        "dest": os.getenv("CLUSTER_ADDRESS"),
        "chunk_size": 10,
        "head": 10,
    },
    "trial_five_signatures": {
        "proof_id": "BDBA557991EC3F5415FC3362D9C7CBD12B78CDE47875B1CB637C4514F52AA08B",
        "name": "trial_five_signatures",
        "dest": os.getenv("CLUSTER_ADDRESS"),
        "chunk_size": 5,
        "head": 5,
    },
     "trial_one_signature": {
        "proof_id": "935C49E1AE1F05916BBD2B7053182F3D99C0D26DD6BD601145E5B6FECE872D62",
        "name": "trial_one_signature",
        "dest": os.getenv("CLUSTER_ADDRESS"),
        "chunk_size": 1,
        "head": 1,
    },
    "block_merkle_proof": {
        "proof_id": "24BD075A420C91E3FE098DA2B118CEE159FFA1C3BB547B2E9988F8D557F3602F",
        "name": "block_merkle_proof",
        "dest": os.getenv("CLUSTER_ADDRESS"),
    },
    "block_signature": {
        "proof_id": "935C49E1AE1F05916BBD2B7053182F3D99C0D26DD6BD601145E5B6FECE872D62",
        "name": "block_signature",
        "dest": os.getenv("CLUSTER_ADDRESS"),
        "chunk_size": 1,
        "head": 1000,
    },
     "block_map_func": {
        "proof_id": "87446ED898EAD75C208BCDB057A9F8EB20FF925F9D277C628F8CEA1F91F6A55E",
        "name": "block_map_func",
        "dest": os.getenv("CLUSTER_ADDRESS"),
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
    "concurrency_one_signature_five_connections": {
        "proof_id": "935C49E1AE1F05916BBD2B7053182F3D99C0D26DD6BD601145E5B6FECE872D62",
        "name": "concurrency_one_signature_five_connections",
        "dest": os.getenv("CLUSTER_ADDRESS"),
        "chunk_size": 1,
        "connections": 5,
        "head": 1000,
    },
    "concurrency_one_signature_ten_connections": {
        "proof_id": "935C49E1AE1F05916BBD2B7053182F3D99C0D26DD6BD601145E5B6FECE872D62",
        "name": "concurrency_one_signature_ten_connections",
        "dest": os.getenv("CLUSTER_ADDRESS"),
        "chunk_size": 1,
        "connections": 12,
        "head": 1000,
    },
    "concurrency_one_signature_20_connections": {
        "proof_id": "935C49E1AE1F05916BBD2B7053182F3D99C0D26DD6BD601145E5B6FECE872D62",
        "name": "concurrency_one_signature_20_connections",
        "dest": os.getenv("CLUSTER_ADDRESS"),
        "chunk_size": 1,
        "connections": 25,
        "head": 1000,
    },
        "concurrency_five_signature_five_connections": {
        "proof_id": "BDBA557991EC3F5415FC3362D9C7CBD12B78CDE47875B1CB637C4514F52AA08B",
        "name": "concurrency_five_signature_five_connections",
        "dest": os.getenv("CLUSTER_ADDRESS"),
        "chunk_size": 5,
        "connections": 5,
        "head": 1000,
    },
    "concurrency_five_signature_ten_connections": {
        "proof_id": "BDBA557991EC3F5415FC3362D9C7CBD12B78CDE47875B1CB637C4514F52AA08B",
        "name": "concurrency_five_signature_ten_connections",
        "dest": os.getenv("CLUSTER_ADDRESS"),
        "chunk_size": 5,
        "connections": 12,
        "head": 1000,
    },
    "concurrency_five_signature_20_connections": {
        "proof_id": "BDBA557991EC3F5415FC3362D9C7CBD12B78CDE47875B1CB637C4514F52AA08B",
        "name": "concurrency_five_signature_20_connections",
        "dest": os.getenv("CLUSTER_ADDRESS"),
        "chunk_size": 5,
        "connections": 25,
        "head": 1000,
    },
    "dataset_signatures_four_replicas": {
        "proof_id": "BDBA557991EC3F5415FC3362D9C7CBD12B78CDE47875B1CB637C4514F52AA08B",
        "name": "dataset_signatures_four_replicas",
        "dest": os.getenv("CLUSTER_ADDRESS"),
        "chunk_size": 5,
        "connections": 20,
    },
    "dataset_signatures_three_replicas": {
        "proof_id": "BDBA557991EC3F5415FC3362D9C7CBD12B78CDE47875B1CB637C4514F52AA08B",
        "name": "dataset_signatures_three_replicas",
        "dest": os.getenv("CLUSTER_ADDRESS"),
        "chunk_size": 5,
        "connections": 15,
    },
    "dataset_signatures_two_replicas": {
        "proof_id": "BDBA557991EC3F5415FC3362D9C7CBD12B78CDE47875B1CB637C4514F52AA08B",
        "name": "dataset_signatures_two_replicas",
        "dest": os.getenv("CLUSTER_ADDRESS"),
        "chunk_size": 5,
        "connections": 10,
    },
}

if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise Exception("Please Provide test(s) name")

    for i in range (1, len(sys.argv)):
        test_config = tests[sys.argv[i]]
        test = PerformanceTest(**test_config)
        loop = asyncio.get_event_loop()
        future = asyncio.ensure_future(test.run())
        loop.run_until_complete(future)
