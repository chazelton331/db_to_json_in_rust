#!/bin/sh

DB_NAME="testing_rust"

dropdb   $DB_NAME
createdb $DB_NAME

psql -d $DB_NAME -f ./seeds.sql
