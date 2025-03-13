#!/bin/bash

set +x
set +e

echo "checking for README"
if [ ! -e "./README" ]
then
        echo "Error: No README file"
        exit 1
fi

echo "checking for Makefile"
if [ ! -e "./Makefile" ]
then
        echo "Error: No Makefile file"
        exit 1
fi

echo "Running make"
make
rc=$?
if [ $rc -ne 0 ]
then
        echo "Error when running the make command"
        exit 1
fi

echo "Running make again"
make
rc=$?
if [ $rc -ne 0 ]
then
        echo "Error when running the make command again"
        exit 1
fi

if [ ! -e "./secure_house" ]
then
        echo "Error: Running make did not create the secure_house file"
        exit 1
fi

if [ ! -x "./secure_house" ]
then
        echo "Error: secure_house is not executable"
        exit 1
fi

INPUT_CASE="WHO'S INSIDE?
INSERT KEY david key1
WHO'S INSIDE?
ENTER HOUSE david
TURN KEY david
LEAVE HOUSE david
RANDOM COMMAND
WHO'S INSIDE?
INSERT KEY pat foobar
CHANGE LOCKS pat newkey
TURN KEY pat
ENTER HOUSE pat
WHO'S INSIDE?
INSERT KEY pat key999
WHO'S INSIDE?
TURN KEY pat
ENTER HOUSE pat
INSERT KEY pat key999
WHO'S INSIDE?
TURN KEY pat
ENTER HOUSE pat
WHO'S INSIDE?
CHANGE LOCKS selina newkey1 newkey2
INSERT KEY sam someKey
TURN KEY sam
ENTER HOUSE sam
WHO'S INSIDE?
LEAVE HOUSE pat
WHO'S INSIDE?
LEAVE HOUSE selina
WHO'S INSIDE?
INSERT KEY selina newkey2
TURN KEY selina
ENTER HOUSE selina
WHO'S INSIDE?
CHANGE LOCKS selina brandNewKey
WHO'S INSIDE?
INSERT KEY sam FIREFIGHTER_SECRET_KEY
TURN KEY sam
ENTER HOUSE sam
WHO'S INSIDE?
INSERT KEY intruder brandNewKey
TURN KEY intruder
ENTER HOUSE intruder
WHO'S INSIDE?
LEAVE HOUSE sam
LEAVE HOUSE pat
LEAVE HOUSE intruder
WHO'S INSIDE?
"

CORRECT_OUTPUT="NOBODY HOME
KEY key1 INSERTED BY david
NOBODY HOME
ACCESS DENIED
FAILURE david HAD INVALID KEY key1 INSERTED
david NOT HERE
ERROR
NOBODY HOME
KEY foobar INSERTED BY pat
LOCK CHANGE DENIED
SUCCESS pat TURNS KEY foobar
ACCESS ALLOWED
pat
KEY key999 INSERTED BY pat
pat
FAILURE pat HAD INVALID KEY key999 INSERTED
ACCESS DENIED
KEY key999 INSERTED BY pat
pat
FAILURE pat HAD INVALID KEY key999 INSERTED
ACCESS DENIED
pat
LOCK CHANGE DENIED
KEY someKey INSERTED BY sam
FAILURE sam HAD INVALID KEY someKey INSERTED
ACCESS DENIED
pat
pat LEFT
NOBODY HOME
selina NOT HERE
NOBODY HOME
KEY newkey2 INSERTED BY selina
FAILURE selina HAD INVALID KEY newkey2 INSERTED
ACCESS DENIED
NOBODY HOME
LOCK CHANGE DENIED
NOBODY HOME
KEY FIREFIGHTER_SECRET_KEY INSERTED BY sam
SUCCESS sam TURNS KEY FIREFIGHTER_SECRET_KEY
ACCESS ALLOWED
sam
KEY brandNewKey INSERTED BY intruder
FAILURE intruder HAD INVALID KEY brandNewKey INSERTED
ACCESS DENIED
sam
sam LEFT
pat NOT HERE
intruder NOT HERE
NOBODY HOME
"

echo "Testing your program"
OUTPUT=$( echo -n "$INPUT_CASE" | ./secure_house selina foobar key2 key3)

DIFF=$(diff -aBw <(echo "$OUTPUT") <(echo "$CORRECT_OUTPUT"))
rc=$?
if [ $rc -ne 0 ]
then
	echo "Error: did not pass the basic test case on the website."
	echo "$DIFF"
else
	echo "SUCCESS!"
fi
