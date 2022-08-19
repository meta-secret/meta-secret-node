#!/bin/bash

set -e

echo "register1:"
curl -X POST http://localhost:8000/register -H 'Content-Type: application/json' -d '{"vaultName":"test_vault","publicKey":"ZE+rI1+X7IsWkCbnTamDtfvvavrIp7UfAtpUVJXfBZ8=","signature":"OOshi5j4XmhxJfCtd3DiQkPIe87NxEc5TvSkqlma+0qxAEWKBpvy4HCR+yKll5p8R1ttKKL9UG9IO2rIIxm6DQ=="}'
echo

echo "register2:"
curl -X POST http://localhost:8000/register -H 'Content-Type: application/json' -d '{"vaultName":"test_vault","publicKey":"Mi6MUjlvim7r2Qz5Ug63ZnkXhaDoBWh3os/ItPzP3Aw=","signature":"haE9QJfSZyLYuKOP9dao0gI2i/bCnjFh6Zph72xgpftuTdzAOotnB5D8r8+IsPFWhqEIpKzEBGsrA59H433xBw=="}'
echo

echo "join"
curl -X POST http://localhost:8000/accept -H 'Content-Type: application/json' -d '{"member": {"vaultName":"test_vault","publicKey":"ZE+rI1+X7IsWkCbnTamDtfvvavrIp7UfAtpUVJXfBZ8=","signature":"OOshi5j4XmhxJfCtd3DiQkPIe87NxEc5TvSkqlma+0qxAEWKBpvy4HCR+yKll5p8R1ttKKL9UG9IO2rIIxm6DQ=="}, "candidate": {"vaultName":"test_vault","publicKey":"Mi6MUjlvim7r2Qz5Ug63ZnkXhaDoBWh3os/ItPzP3Aw=","signature":"haE9QJfSZyLYuKOP9dao0gI2i/bCnjFh6Zph72xgpftuTdzAOotnB5D8r8+IsPFWhqEIpKzEBGsrA59H433xBw=="}}'
echo

echo "get vault"
curl -X GET http://localhost:8000/getVault -H 'Content-Type: application/json' -d '{"vaultName":"test_vault","publicKey":"Mi6MUjlvim7r2Qz5Ug63ZnkXhaDoBWh3os/ItPzP3Aw=","signature":"haE9QJfSZyLYuKOP9dao0gI2i/bCnjFh6Zph72xgpftuTdzAOotnB5D8r8+IsPFWhqEIpKzEBGsrA59H433xBw=="}'
echo

echo finished
echo