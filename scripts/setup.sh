#!/usr/bin/bash

export DATABASE_URL="sqlite:./db.sqlite"
sqlx database setup