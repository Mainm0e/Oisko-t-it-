#!/bin/bash

# Configuration
# Assuming the domain from your screenshot
API_HOST="apiserver.maim0e.studio" 
API_URL="https://$API_HOST/api"

echo "🔍 Testing Backend at: $API_URL"
echo "-----------------------------------"

# 1. DNS Check
echo -n "1️⃣  DNS Resolution ($API_HOST): "
if host "$API_HOST" > /dev/null 2>&1; then
    echo "✅ Responded"
else
    echo "⚠️  Could not resolve (Check your DNS/Domain settings)"
fi

# 2. Public Applications Endpoint
echo -n "2️⃣  Public Applications (/api/public/applications): "
# We use curl to fetch. -s for silent, -o /dev/null to hide output, -w to show http code
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" "$API_URL/public/applications")

if [ "$HTTP_CODE" == "200" ]; then
    echo "✅ OK (HTTP 200)"
else
    echo "❌ Failed (HTTP $HTTP_CODE)"
    echo "   -> If this is 404, check your routes."
    echo "   -> If this is 000, check if the server is reachable via HTTPS."
fi

# 3. Visit Endpoint
echo -n "3️⃣  Visit Tracking (/api/visit): "
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$API_URL/visit")

if [ "$HTTP_CODE" == "200" ]; then
    echo "✅ OK (HTTP 200)"
else
    echo "❌ Failed (HTTP $HTTP_CODE)"
fi

echo "-----------------------------------"
echo "💡 DIAGNOSIS:"
echo "If these tests PASS but your frontend still fails:"
echo "👉 You are missing 'https://' in your frontend API_URL build argument."
echo ""
echo "Current value seen in logs: apiserver.maim0e.studio/api"
echo "Required value:             https://apiserver.maim0e.studio"
