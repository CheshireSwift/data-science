import pandas as pd

df = pd.read_csv("./data/scottish_epc_ratings.csv")
df['date'] = pd.to_datetime(df['date'])

print(df['date'].is_monotonic_increasing)
print(df['date'].is_monotonic_decreasing)

print(df)