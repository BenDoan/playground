#!/usr/bin/env python

import json

import feedparser
import praw

URL = "http://www.youtube.com/feeds/videos.xml?channel_id={}"

CHANNELS = [
    ("Bill Wurtz", "UCq6aw03lNILzV96UvEAASfQ"),
    ("CGP Grey", "UC2C_jShtL725hvbm1arSV9w"),
    ("Gus Johnson", "UCpIafFPGutTAKOBHMtGen7g"),
    ("ProZD", "UC6MFZAOHXlKK1FI7V0XQVeA"),
    (None, "UCAL3JXZSzSm8AlZyD3nQdBA"),  # Primitive Technology
    (None, "UCJHA_jMfCvEnv-3kRjTCQXw"),  # Binging with babish,
    # ("Ahoy", "UCE1jXbVAGJQEORz9nZqb5bQ"),
    # (None, "UCekQr9znsk2vWxBo3YiLq2w"),  # You Suck at Cooking
    # ("Alt Shift X", "UCveZqqGewoyPiacooywP5Ig"),
    # ("Matthias Wandel", "UCckETVOT59aYw80B36aP9vw"),
    # (None, "UCY1kMZp36IQSyNx_9h4mpCg"), # Mark Rober
]

REDDIT_CONF = {
    "client_id": "SwnMpORnJ9Ocgg",
    "client_secret": "S_TbkTaYtc2Z3L7_A-G-7jpO3GI",
    "username": "simcaster",
    "password": 'z<SwMP[w;yl@N7"p0F*y',
}


def main():
    find_new_videos()


def find_new_videos():
    try:
        with open("processed.json", "r") as f:
            processed_entries = json.load(f)
    except json.decoder.JSONDecodeError:
        processed_entries = {}

    for author, channel_id in CHANNELS:
        d = feedparser.parse(URL.format(channel_id))
        entries = {x.link: (x.title, author) for x in d.entries}
        prev = processed_entries.get(channel_id, {})

        processed_entries[channel_id] = {**entries, **prev}

        new_urls = list(set(entries.keys()) - set(prev.keys()))
        for url in new_urls:
            title, author = entries[url]
            post_video(title, url)

    with open("processed.json", "w") as f:
        json.dump(processed_entries, f, indent=4)


def post_video(title: str, url: str):
    reddit = praw.Reddit(client_id=REDDIT_CONF['client_id'],
                     client_secret=REDDIT_CONF['client_secret'],
                     user_agent=REDDIT_CONF['username'],
                     username=REDDIT_CONF['username'],
                     password=REDDIT_CONF['password'])
    subreddit = reddit.subreddit("videos")
    print("Posting {}".format(title))
    try:
        submission = subreddit.submit(title, url=url, resubmit=False)
    except praw.exceptions.APIException:
        print("Failed")
    else:
        print("Success")


if __name__ == "__main__":
    main()
