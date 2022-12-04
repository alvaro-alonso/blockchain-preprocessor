# To add a new cell, type '# %%'
# To add a new markdown cell, type '# %% [markdown]'
# %%
import pandas as pd
import random
from multiprocesspandas import applyparallel

from tqdm import tqdm
from zokrates_pycrypto.eddsa import PrivateKey, PublicKey
from zokrates_pycrypto.field import FQ
from zokrates_pycrypto.utils import write_signature_for_zokrates_cli

tqdm.pandas()

df = pd.read_csv("discovergy_tabular.csv", sep='\t')

di = df.MId.drop_duplicates().to_frame()
di['priv_key'] = di.MId.apply(lambda x: PrivateKey.from_rand())
di['pub_key'] = di['priv_key'].apply(lambda x: PublicKey.from_private(x))

dff = df.set_index('MId').join(di.set_index('MId'))

# create a msg from PAvg and EOut 
dff['PAvg'] = dff['PAvg'].fillna(0).apply(lambda x: int(abs(x)))
dff['EOut'] = dff['EOut'].fillna(0).apply(lambda x: int(x))
dff['msg'] = dff.fillna(0).apply(lambda x: x.PAvg.to_bytes(32, 'big') +  x.EOut.to_bytes(32, 'big'), axis=1)
dff.head(5)


# %%
sign_df = dff
sign_df['signature'] = sign_df.progress_apply(lambda x: x.priv_key.sign(x.msg), axis=1)


# %%
def signature_args(pk, sig, msg):
    "Writes the input arguments for verifyEddsa in the ZoKrates stdlib to file."
    sig_R, sig_S = sig
    args = [sig_R.x, sig_R.y, sig_S, pk.p.x.n, pk.p.y.n]
    args = " ".join(map(str, args))

    M0 = msg.hex()[:64]
    M1 = msg.hex()[64:]
    b0 = [str(int(M0[i:i+8], 16)) for i in range(0,len(M0), 8)]
    b1 = [str(int(M1[i:i+8], 16)) for i in range(0,len(M1), 8)]
    args = args + " " + " ".join(b0 + b1)

    return args

sign_df['args'] = sign_df.progress_apply(lambda x: signature_args(x.pub_key, x.signature, x.msg), axis=1)
sign_df['priv_key'] = sign_df['priv_key'].apply(lambda x: str(x.fe))
sign_df['pub_key'] = sign_df['pub_key'].apply(lambda point: {"x": str(point.p.x), "y": str(point.p.y)})
sign_df['msg'] = sign_df['msg'].apply(lambda x: int.from_bytes(x, byteorder='big', signed=False))
sign_df['sign'] = sign_df['signature'].apply(lambda sig: { "sig_R": {"x": str(sig[0].x), "y": str(sig[0].y)}, "sig_S": str(sig[1]) })

sign_df.reset_index()[['MId', 'priv_key', 'pub_key', 'PAvg', 'EOut','msg', 'sign', 'args']].to_json(path_or_buf='test_data.json', orient='records')

