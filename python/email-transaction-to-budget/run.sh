#!/bin/bash

scriptdir=`dirname "$BASH_SOURCE"`

cd $scriptdir
source env/bin/activate
python email-transaction-to-budget.py
