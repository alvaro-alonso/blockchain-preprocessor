import pandas as pd
import asyncio
import aiohttp
import time

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

        df = pd.read_json("../dataset/test_data.json")
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
        start = time.time()
        resp = await session.post(url, json=data, **kwargs)
        # Note that this may raise an exception for non-2xx responses
        # You can either handle that here, or pass the exception through
        data = await resp.json()
        status = resp.status
        self.pbar.update(1)
        return (start, data, status)

    async def run(self, **kwargs):
        url = f"{self.dest}/{self.proof_id}/compute-generate-proof"
        print(f"Requesting to {url}")
        timeout = aiohttp.ClientTimeout(total=None)
        async with aiohttp.ClientSession(timeout=timeout) as session:
            tasks = []
            for proof in self.data:
                req_body = {"payload": proof}
                task = asyncio.ensure_future(self.__send_proof(session, url, req_body))
                tasks.append(task)
            # asyncio.gather() will wait on the entire task set to be
            # completed.  If you want to process results greedily as they come in,
            # loop over asyncio.as_completed()
            self.pbar = tqdm(total=len(tasks), desc='Scanning files')
            htmls = await asyncio.gather(*tasks, return_exceptions=True)
            print(htmls)
            return htmls

