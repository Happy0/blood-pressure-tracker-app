
#!/bin/bash

if [ -z $1 ]; then
    echo "Missing machine ID argument.";
    exit 1;
fi

fly console --machine $1 -C "zip -r /tmp/files.zip /tmp/bpm-ocr"

fly sftp get /tmp/files.zip files.zip