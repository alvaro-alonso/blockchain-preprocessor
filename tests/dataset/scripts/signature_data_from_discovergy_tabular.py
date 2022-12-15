# %%
import pandas as pd
from multiprocesspandas import applyparallel

from tqdm import tqdm
from zokrates_pycrypto.eddsa import PrivateKey, PublicKey


tqdm.pandas()

df = pd.read_csv("discovergy_tabular.csv", sep='\t')

# convert raw values into byte arrays of 2 bytes
df['PAvg'] = df['PAvg'].fillna(0.01).apply(lambda x: min(int(abs(x) * 100), int(2 ** 16 - 1)).to_bytes(2, 'big', signed=False))

# aggregate the byte measures into 64 byte messages
agg = df.sort_values('IEnd').groupby('MId').PAvg.apply(list)
agg = agg.apply(lambda x: [b''.join(x[i:i+32]).rjust(64, b"\0") for i in range(0, len(x), 32)])
bytes = agg.explode().to_frame()

# generate priv_k und pub_k for each sensor
di = df.MId.drop_duplicates().to_frame()
di['priv_key'] = di.MId.apply(lambda x: PrivateKey.from_rand())
di['pub_key'] = di['priv_key'].apply(lambda x: PublicKey.from_private(x))

sign_df = di.set_index('MId').join(bytes)

sign_df.head(5)

# %%
sign_df['signature'] = sign_df.progress_apply(lambda x: x.priv_key.sign(x.PAvg), axis=1)

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

sign_df['args'] = sign_df.progress_apply(lambda x: signature_args(x.pub_key, x.signature, x.PAvg), axis=1)
sign_df['priv_key'] = sign_df['priv_key'].apply(lambda x: str(x.fe))
sign_df['msg'] = sign_df['PAvg'].apply(lambda x: x.hex())
sign_df['sign'] = sign_df['signature'].apply(lambda sig: { "sig_R": {"x": str(sig[0].x), "y": str(sig[0].y)}, "sig_S": str(sig[1]) })
sign_df['pub_key'] = sign_df['pub_key'].apply(lambda point: {"x": str(point.p.x), "y": str(point.p.y)})

sign_df.reset_index()[['MId', 'priv_key', 'pub_key', 'msg', 'sign', 'args']].to_json(path_or_buf='test_data.json', orient='records')

# %%



