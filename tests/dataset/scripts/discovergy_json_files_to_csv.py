
# To add a new cell, type '# %%'
# To add a new markdown cell, type '# %% [markdown]'
# %%
import re
import pandas as pd
from glob import glob

data = []
for f_name in glob('../raw/*.json'):
    date_str = re.search('(\d{4}-\d{2}-\d{2})', f_name).group(0)
    day_df = pd.read_json(f_name)
    day_df['date'] = date_str
    data.append(day_df)
    
df = pd.concat(data, ignore_index=True)
df.to_csv("../discovergy_tabular.csv", sep='\t', index=False)


# %%
df.shape[0]


