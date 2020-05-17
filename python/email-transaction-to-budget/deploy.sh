#!/bin/bash

scriptdir=`dirname "$BASH_SOURCE"`

cd $scriptdir/..
scp -r email-transaction-to-budget flainted.com:~/code
