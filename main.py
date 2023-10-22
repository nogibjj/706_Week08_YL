import pandas as pd
import time
from mylib.lib import get_median
import psutil

start = time.time()

if __name__ == "__main__":
    file = "top25komapseriesindex.csv"
    df = pd.read_csv(file)
    print(get_median(df))
    end = time.time()

elapsed = end - start

cpu_percent = psutil.cpu_percent()
memory_info = psutil.virtual_memory()

print(f"Elapsed time: {elapsed:.4f} seconds")
print(f"CPU Usage: {cpu_percent}%")
print(f"Memory usage: {memory_info.percent}%")