import pandas as pd
import asyncio
import aiohttp
import time
import abc

class PerformanceTest(abc.ABC):
    def __init__(
        self, 
        name, 
        version, 
        description, 
        dest, 
        proof_id
    ):
        self.name = name
        self.version = version
        self.description
        self.dest = dest
        self.proof_id = proof_id
        self.data = self.__load()

    @abc.abstractclassmethod
    def __load(self): list:
        raise NotImplementedError

    async def __send_proof(
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
        end = time.time()
        status = resp.status
        return (start, end, data, status)

    async def run(self):
        print(f"Requesting to {self.dest}")
        url = f"{self.dest}/{self.proof_id}/compute-generate-proof"
        async with aiohttp.ClientSession() as session:
            tasks = []
            for proof in self.data:
                req_body = {"payload": proof}
                tasks.append(get(session, url, req_body, **kwargs))
            # asyncio.gather() will wait on the entire task set to be
            # completed.  If you want to process results greedily as they come in,
            # loop over asyncio.as_completed()
            htmls = await asyncio.gather(*tasks, return_exceptions=True)
            return htmls


if __name__ == 'main':
    Test('http://0.0.0.0:8000')