#!/bin/bash

scriptdir=`dirname "$BASH_SOURCE"`

cd $scriptdir
source env/bin/activate
python email_transaction_to_budget.py
