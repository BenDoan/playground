#!/usr/bin/env python

import argparse
from googleapiclient.discovery import build
from google_auth_oauthlib.flow import InstalledAppFlow
from google.auth.transport.requests import Request
import configparser
import datetime
import imaplib
import json
import os.path
import pickle
import re
import traceback
import enum
from bs4 import BeautifulSoup
import time
import logging

log = logging.getLogger(__name__)

HAVE_READ_MIDS_FILE = "have-processed-mids.json"
HAVE_READ_TRANSACTIONS_FILE = "have-processed-transactions.json"

# If modifying these scopes, delete the file token.pickle.
SCOPES = ["https://www.googleapis.com/auth/spreadsheets"]

# The ID and range of a sample spreadsheet.
SPREADSHEET_ID = "1h-GBpn__5CG-jlG1LuQbooNaOm_rLQmBmz6OS1GdaeM"

TEMPLATE_SHEET_ID = "740803055"

BudgetCategory = enum.Enum("BudgetCategory", "food shopping recurring")

FOOD_RANGE = "B6:C"
SHOPPING_RANGE = "E6:F"
RECURRING_RANGE = ""

# keep lowercase
food_vendors = ["hy-vee", "doordash", "chipotle", "jimmy johns", "wholefds", "trader joe", "asian market"]


def main(dry_run, proc_all):
    FORMAT = '[%(asctime)s] %(message)s'
    logging.basicConfig(filename='email_transaction_to_budget.log', level=logging.INFO, format=FORMAT)
    logging.getLogger().addHandler(logging.StreamHandler())

    config = configparser.ConfigParser()
    config.read("imap-creds.ini")

    username = config.get("creds", "username")
    password = config.get("creds", "password")
    hostname = config.get("server", "hostname")

    have_processed_mids = {}
    have_processed_transactions = {}
    try:
        if not proc_all:
            have_processed_mids = json.load(open(HAVE_READ_MIDS_FILE))
            have_processed_transactions = json.load(open(HAVE_READ_TRANSACTIONS_FILE))
    except:
        pass

    service = get_sheets_service()

    with imaplib.IMAP4_SSL(hostname) as M:
        M.login(username, password)
        M.select("Transactions", readonly=True)

        now = datetime.datetime.now()
        week_ago = datetime.timedelta(days=-7) + now
        formatted_date = week_ago.strftime("%d-%b-%Y")

        entries = [
            *process_chase(M, formatted_date, have_processed_mids, have_processed_transactions),
            *process_capital_one(M, formatted_date, have_processed_mids, have_processed_transactions),
        ]

        if not dry_run:
            add_to_spreadsheet(service, entries)

    if not dry_run:
        with open(HAVE_READ_MIDS_FILE, "w+") as f:
            json.dump(have_processed_mids, f)
        with open(HAVE_READ_TRANSACTIONS_FILE, "w+") as f:
            json.dump(have_processed_transactions, f)


def clean(s):
    t = s.replace("\r", "")
    t = re.sub(r"</?\w+>", "", t)
    t = t.strip()
    return t


def get_sheets_service():
    creds = None
    # The file token.pickle stores the user's access and refresh tokens, and is
    # created automatically when the authorization flow completes for the first
    # time.
    if os.path.exists("token.pickle"):
        with open("token.pickle", "rb") as token:
            creds = pickle.load(token)
    # If there are no (valid) credentials available, let the user log in.
    if not creds or not creds.valid:
        if creds and creds.expired and creds.refresh_token:
            creds.refresh(Request())
        else:
            flow = InstalledAppFlow.from_client_secrets_file("credentials.json", SCOPES)
            creds = flow.run_local_server(port=0)
        # Save the credentials for the next run
        with open("token.pickle", "wb") as token:
            pickle.dump(creds, token)

    service = build("sheets", "v4", credentials=creds)
    return service


def get_curr_month_str():
    return datetime.datetime.now().strftime("%Y-%m")


def get_current_month_sheet(service):
    resp = service.spreadsheets().get(spreadsheetId=SPREADSHEET_ID).execute()
    sheets = resp.get("sheets", [])
    curr_month_str = get_curr_month_str()
    for sheet in sheets:
        if sheet.get("properties", {}).get("title") == curr_month_str:
            return sheet
    return None


def add_to_spreadsheet(service, entries):
    curr_month_sheet = get_current_month_sheet(service)
    if not curr_month_sheet:
        copy_sheet(service, TEMPLATE_SHEET_ID)
    curr_month_str = get_curr_month_str()

    for entry in entries:
        cat = classify(entry[0], entry[1])
        if cat == BudgetCategory.food:
            RANGE_NAME = f"{curr_month_str}!{FOOD_RANGE}"
        else:
            RANGE_NAME = f"{curr_month_str}!{SHOPPING_RANGE}"

        body = {
            "range": RANGE_NAME,
            "majorDimension": "ROWS",
            "values": [entry],
        }
        result = (
            service.spreadsheets()
            .values()
            .append(
                spreadsheetId=SPREADSHEET_ID,
                range=RANGE_NAME,
                valueInputOption="USER_ENTERED",
                body=body,
            )
            .execute()
        )


def copy_sheet(service, old_sheet_id):
    resp = (
        service.spreadsheets()
        .sheets()
        .copyTo(
            spreadsheetId=SPREADSHEET_ID,
            sheetId=old_sheet_id,
            body={"destinationSpreadsheetId": SPREADSHEET_ID},
        )
        .execute()
    )

    sheet_id = resp["sheetId"]
    change_tab_index(service, sheet_id, 0)
    change_tab_title(service, sheet_id, get_curr_month_str())
    return sheet_id


def change_tab_index(service, sheet_id, index):
    resp = (
        service.spreadsheets()
        .batchUpdate(
            spreadsheetId=SPREADSHEET_ID,
            body={
                "requests": [
                    {
                        "updateSheetProperties": {
                            "properties": {
                                "sheetId": sheet_id,
                                "index": 0,
                            },
                            "fields": "index",
                        }
                    }
                ]
            },
        )
        .execute()
    )


def change_tab_title(service, sheet_id, title):
    resp = (
        service.spreadsheets()
        .batchUpdate(
            spreadsheetId=SPREADSHEET_ID,
            body={
                "requests": [
                    {
                        "updateSheetProperties": {
                            "properties": {
                                "sheetId": sheet_id,
                                "title": title,
                            },
                            "fields": "title",
                        }
                    }
                ]
            },
        )
        .execute()
    )


"""
TODO
- auto create tab for new month
- add transactions to category for cur month
- scrape recurring list from spreadsheet
- stretch: decorate amazon transactions
"""


def classify(merchant, amount):
    merchant_l = merchant.lower()

    if merchant.startswith("TST*"):
        return BudgetCategory.food

    for food_vendor in food_vendors:
        if food_vendor in merchant_l:
            return BudgetCategory.food


def process_chase(M, formatted_date, have_processed_mids, have_processed_transactions):
    message_ids_from_chase = M.search(
        "NONE", "FROM", '"chase.com"', "SINCE", formatted_date
    )[1][0].split()
    log.info(f"Found {len(message_ids_from_chase)} messages from Chase")
    entries = []
    for mid in message_ids_from_chase:
        d_mid = mid.decode("utf-8")
        if d_mid not in have_processed_mids:
            try:
                message = M.fetch(d_mid, "(BODY.PEEK[TEXT])")[1][0][1]
                decoded_message = message.decode("UTF-8")
                scrubbed_message = decoded_message.replace("=", "").replace(
                    "\n", ""
                )

                if "credit card statement is ready" in scrubbed_message:
                    have_processed_mids[d_mid] = True
                    continue

                soup = BeautifulSoup(scrubbed_message, "html.parser")
                tables = soup.find_all("table")
                merchant = None
                amount = None
                datestr = None
                for table in tables:
                    tds = table.find_all("td")
                    if len(tds) < 2:
                        continue
                    text1 = tds[0].text
                    text2 = tds[1].text

                    if text1 == "Merchant":
                        merchant = clean(text2)

                    if text1 == "Amount":
                        amount = clean(text2)

                    if text1 == "Date":
                        datestr = clean(text2)

                ident = f"{merchant}-{amount}-{datestr}"

                if amount is None or merchant is None:
                    log.error("failed to process")
                    log.error(soup.prettify())
                    merchant = "ERROR"

                have_processed_mids[d_mid] = True
                if ident in have_processed_transactions:
                    continue

                entries.append([merchant, amount])
                have_processed_transactions[ident] = True
                log.info(f"{d_mid}, {amount}, {merchant}, {datestr}")
            except Exception as e:
                log.error("Couldn't process: {}".format(decoded_message))
                traceback.print_exc()
    return entries

def process_capital_one(M, formatted_date, have_processed_mids, have_processed_transactions):
    message_ids_from_capone = M.search(
        "NONE", "FROM", '"capitalone.com"', "SINCE", formatted_date
    )[1][0].split()
    log.info(f"Found {len(message_ids_from_capone)} messages from Capital One")
    entries = []
    for mid in message_ids_from_capone:
        d_mid = mid.decode("utf-8")
        if d_mid not in have_processed_mids:
            try:
                message = M.fetch(d_mid, "(BODY.PEEK[TEXT])")[1][0][1]
                decoded_message = message.decode("UTF-8")

                # format:
                # notifying you that on August 14, 2025, at AMAZON MKTPLACE PMTS, a pending authorization or purchase in the amount of $45.91 was placed or charged

                amount_match = re.search(r'\$\d+\.\d{2}', decoded_message)
                if amount_match:
                    amount = amount_match.group(0)
                else:
                    amount = "AMOUNT-ERROR"

                merchant_match = re.search(r', at ([^,]+),', decoded_message)
                if merchant_match:
                    merchant = merchant_match.group(1)
                else:
                    merchant = "MERCHANT-ERROR"

                date_match = re.search(r'on (\S+ \d{1,2}, \d{4}),', decoded_message)
                if date_match:
                    transaction_date = date_match.group(1)
                else:
                    transaction_date = "DATE-ERROR"

                ident = f"{merchant}-{amount}-{transaction_date}"

                have_processed_mids[d_mid] = True
                if ident in have_processed_transactions:
                    continue

                entries.append([merchant, amount])
                have_processed_transactions[ident] = True
                log.info(f"{d_mid}, {amount}, {merchant}, {transaction_date}")
            except Exception as e:
                log.error("Couldn't process: {}".format(decoded_message))
                traceback.print_exc()
    return entries



if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("-d", "--dry-run", action="store_true")
    parser.add_argument("-a", "--all", action="store_true")
    args = parser.parse_args()

    main(dry_run=args.dry_run, proc_all=args.all)
