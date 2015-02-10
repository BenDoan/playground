import requests
import time

from multiprocessing import Pool

def download_google(t):
    print("downloading cnn")

    time.sleep(1)
    page = requests.get("http://cnn.com").text
    time.sleep(1)

    return {"name": "cnn", "text": page}

pool = Pool(8)
results = pool.map(download_google, range(50))
print(results)
