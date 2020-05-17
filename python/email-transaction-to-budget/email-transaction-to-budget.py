#!/usr/bin/env python

from googleapiclient.discovery import build
from google_auth_oauthlib.flow import InstalledAppFlow
from google.auth.transport.requests import Request
import configparser
import imaplib
import json
import os.path
import pickle
import re

HAVE_READ_FILE = 'have-processed.json'

# If modifying these scopes, delete the file token.pickle.
SCOPES = ['https://www.googleapis.com/auth/spreadsheets']

# The ID and range of a sample spreadsheet.
SPREADSHEET_ID = '1h-GBpn__5CG-jlG1LuQbooNaOm_rLQmBmz6OS1GdaeM'
RANGE_NAME = 'Transactions!A1:B'

def main():
    config = configparser.ConfigParser()
    config.read('imap-creds.ini');

    username = config.get('creds', 'username')
    password = config.get('creds', 'password')
    hostname = config.get('server', 'hostname')

    have_processed = {}
    try:
        have_processed = json.load(open(HAVE_READ_FILE))
    except:
        pass

    with imaplib.IMAP4_SSL(hostname) as M:
        M.login(username, password)
        M.select('INBOX', readonly=True)
        message_ids_from_chase = M.search('NONE', 'FROM', '"chase.com"')[1][0].split()

        entries = []
        for mid in message_ids_from_chase:
            d_mid = mid.decode("utf-8")
            if d_mid not in have_processed:
                message = M.fetch(d_mid, '(BODY.PEEK[TEXT])')[1][0][1]
                amount, vendor, datestr = re.findall("\(\$USD\) ([0-9.]*) at (.*) has .* authorized on (.*) at", message.decode("UTF-8"))[0]
                entries.append([vendor, amount])
                have_processed[d_mid] = True
                print(d_mid, amount, vendor, datestr)

        add_to_spreadsheet(entries)

    with open(HAVE_READ_FILE, "w+") as f:
        json.dump(have_processed, f)

def add_to_spreadsheet(entries):
    creds = None
    # The file token.pickle stores the user's access and refresh tokens, and is
    # created automatically when the authorization flow completes for the first
    # time.
    if os.path.exists('token.pickle'):
        with open('token.pickle', 'rb') as token:
            creds = pickle.load(token)
    # If there are no (valid) credentials available, let the user log in.
    if not creds or not creds.valid:
        if creds and creds.expired and creds.refresh_token:
            creds.refresh(Request())
        else:
            flow = InstalledAppFlow.from_client_secrets_file(
                'credentials.json', SCOPES)
            creds = flow.run_local_server(port=0)
        # Save the credentials for the next run
        with open('token.pickle', 'wb') as token:
            pickle.dump(creds, token)

    service = build('sheets', 'v4', credentials=creds)

    sheet = service.spreadsheets()
    body = {
        "range": RANGE_NAME,
        "majorDimension": "ROWS",
        "values": entries,
    }
    result = sheet.values().append(
        spreadsheetId=SPREADSHEET_ID, range=RANGE_NAME,
        valueInputOption='USER_ENTERED', body=body
    ).execute()

if __name__ == '__main__':
    main()
