from python:3.6

COPY src /src

RUN pip install -r /src/requirements.txt

WORKDIR /src

CMD ["python", "reddit-youtube-poster.py"]
