#!/bin/bash

scriptdir=`dirname "$BASH_SOURCE"`

cd $scriptdir/..
rsync -a --delete --progress --exclude env email-transaction-to-budget flainted.com:~/code
