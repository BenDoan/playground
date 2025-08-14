#!/bin/bash

scriptdir=`dirname "$BASH_SOURCE"`

cd $scriptdir/..
rsync -a --delete --progress --exclude env --exclude have-processed*.json email-transaction-to-budget nuc:~
