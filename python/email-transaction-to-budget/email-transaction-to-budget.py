#!/usr/bin/env python

import configparser
import imaplib
import re

def main():
    config = configparser.ConfigParser()
    config.read('imap-creds.ini');

    username = config.get('creds', 'username')
    password = config.get('creds', 'password')
    hostname = config.get('server', 'hostname')

    with imaplib.IMAP4_SSL(hostname) as M:
        M.login(username, password)
        M.select('INBOX', readonly=True)
        message_ids_from_chase = M.search('NONE', 'FROM', '"chase.com"')[1][0].split()

        for mid in message_ids_from_chase:
            message = M.fetch(mid, '(BODY.PEEK[TEXT])')[1][0][1]
            amount, vendor, datestr = re.findall("\(\$USD\) ([0-9.]*) at (.*) has .* authorized on (.*) at", message.decode("UTF-8"))[0]
            print(mid, amount, vendor, datestr)

if __name__ == '__main__':
    main()
