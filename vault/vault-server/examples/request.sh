#!/bin/bash

set -e

echo "register a new vault:"
curl -X POST http://localhost:8000/register -H 'Content-Type: application/json' -d @member_user_signature.json | jq
echo
echo
echo

echo "try to register a vault which is already exists:"
curl -X POST http://localhost:8000/register -H 'Content-Type: application/json' -d @candidate_user_signature.json | jq
echo
echo
echo
echo

echo "accept join request"
curl -X POST http://localhost:8000/accept -H 'Content-Type: application/json' -d @accept.json | jq
echo
echo
echo
echo

echo "get vault"
curl -X POST http://localhost:8000/getVault -H 'Content-Type: application/json' -d @member_user_signature.json | jq
echo
echo
echo
echo

echo finished
echo