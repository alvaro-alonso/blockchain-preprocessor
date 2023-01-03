import pandas as pd
import asyncio
import aiohttp
import time
import json

from datetime import date
from tqdm.asyncio import tqdm


class PerformanceTest:
    def __init__(
        self,
        proof_id,
        name,
        dest, 
        chunk_size: int = 1,
        head: int = 0
    ):
        self.proof_id = proof_id
        self.name = name
        self.dest = dest
        self.chunk_size: int = chunk_size
        self.data = self.__load(head)

    def __load(self, first_row_num) -> list:

        def transform_args(arg_str: str) -> dict:
            elements = arg_str.split(" ")
            return { 
                "R": elements[:2],
                "S": elements[2],
                "A": elements[3:5],
                "M0": elements[5:13],
                "M1": elements[13:]
            }

        df = pd.read_json("data/test_data.json")
        rows = df.shape[0] if first_row_num < 1 else first_row_num
        rows -= rows % self.chunk_size
        df = df.loc[:rows]
        data = df.args.apply(lambda x: transform_args(x)).to_list()
        return [[data[i:i+self.chunk_size]] for i in range(0, rows, self.chunk_size)]
        

    async def __send_proof(
        self,
        session: aiohttp.ClientSession,
        url: str,
        data: dict,
        **kwargs
    ) -> dict:
        resp = await session.post(url, json=data, **kwargs)
        try: 
            # Note that this may raise an exception for non-2xx responses
            # You can either handle that here, or pass the exception through
            data = await resp.json()
            status = resp.status
            self.pbar.update(1)
            return { "payload": data, "status": status }
        except Exception as e:
            print(resp)
            print(e)
            self.pbar.update(1)

    async def run(self, **kwargs):
        url = f"{self.dest}/{self.proof_id}/compute-generate-proof"
        print(f"Requesting to {url}")
        start = time.time()
        timeout = aiohttp.ClientTimeout(total=None)
        connector = aiohttp.TCPConnector(limit=5)
        async with aiohttp.ClientSession(timeout=timeout, connector=connector) as session:
            tasks = []
            for proof in self.data:
                req_body = {"payload": proof}
                task = asyncio.ensure_future(self.__send_proof(session, url, req_body))
                tasks.append(task)
            # asyncio.gather() will wait on the entire task set to be
            # completed.  If you want to process results greedily as they come in,
            # loop over asyncio.as_completed()
            self.pbar = tqdm(total=len(tasks), desc='Scanning files')
            responses = await asyncio.gather(*tasks)
            end = time.time()
            result = { 
                "time_taken": round(end - start, 3),
                "results": responses
            }
            result_json = json.dumps(result, indent=4)
            with open(f"../results/{self.name}_{date.today()}.json", "w") as outfile:
                outfile.write(result_json)
            
            return result

