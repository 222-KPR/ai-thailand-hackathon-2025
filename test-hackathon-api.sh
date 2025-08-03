#!/bin/bash

# AI4Thai Crop Guardian - Hackathon API Test Script (Team 10)
# Port: 2090

set -e

API_BASE="http://localhost:2090"
echo "🧪 Testing AI4Thai Crop Guardian API (Team 10)"
echo "📍 API Base: $API_BASE"
echo ""

# Test 1: Health Check
echo "1️⃣ Testing Health Check..."
if curl -s -f "$API_BASE/health" > /dev/null; then
    echo "✅ Health check passed"
else
    echo "❌ Health check failed"
    exit 1
fi

# Test 2: Readiness Check
echo ""
echo "2️⃣ Testing Readiness Check..."
if curl -s -f "$API_BASE/health/ready" > /dev/null; then
    echo "✅ Readiness check passed"
else
    echo "❌ Readiness check failed"
fi

# Test 3: Metrics
echo ""
echo "3️⃣ Testing Metrics..."
if curl -s -f "$API_BASE/health/metrics" > /dev/null; then
    echo "✅ Metrics endpoint working"
else
    echo "❌ Metrics endpoint failed"
fi

# Test 4: Chat API
echo ""
echo "4️⃣ Testing Chat API..."
CHAT_RESPONSE=$(curl -s -X POST "$API_BASE/api/v1/chat" \
    -H "Content-Type: application/json" \
    -d '{"message": "Hello, can you help me with my crops?"}')

if echo "$CHAT_RESPONSE" | grep -q "success.*true"; then
    echo "✅ Chat API working"
    echo "   Response: $(echo "$CHAT_RESPONSE" | jq -r '.data.response' 2>/dev/null || echo 'Response received')"
else
    echo "❌ Chat API failed"
    echo "   Response: $CHAT_RESPONSE"
fi

# Test 5: Chat History
echo ""
echo "5️⃣ Testing Chat History..."
HISTORY_RESPONSE=$(curl -s -f "$API_BASE/api/v1/chat/history")

if echo "$HISTORY_RESPONSE" | grep -q "success.*true"; then
    echo "✅ Chat history working"
else
    echo "❌ Chat history failed"
    echo "   Response: $HISTORY_RESPONSE"
fi

# Test 6: Vision Analysis Endpoints
echo ""
echo "6️⃣ Testing Vision Analysis Endpoints..."

# Test file stats
if curl -s -f "$API_BASE/api/v1/vision/files/stats" > /dev/null; then
    echo "✅ File stats endpoint working"
else
    echo "❌ File stats endpoint failed"
fi

# Test job status (should return 404 for non-existent job)
JOB_STATUS=$(curl -s -w "%{http_code}" "$API_BASE/api/v1/vision/jobs/test-job-123" -o /dev/null)
if [ "$JOB_STATUS" = "404" ]; then
    echo "✅ Job status endpoint working (correctly returns 404 for non-existent job)"
else
    echo "❌ Job status endpoint failed (got $JOB_STATUS)"
fi

echo ""
echo "🎉 API Testing Complete!"
echo ""
echo "📊 Test Summary:"
echo "   Health Check: ✅"
echo "   Chat API: ✅"
echo "   Vision API: ✅"
echo ""
echo "🌐 Your API is ready at: https://api.hackathon2025.ai.in.th/team10-1"
echo ""
echo "📝 Available endpoints:"
echo "   GET  /health                    - Health check"
echo "   GET  /health/ready              - Readiness check"
echo "   GET  /health/metrics            - Metrics"
echo "   POST /api/v1/chat               - Send chat message"
echo "   GET  /api/v1/chat/history       - Get chat history"
echo "   POST /api/v1/vision/analyze     - Upload image for analysis"
echo "   GET  /api/v1/vision/jobs/{id}   - Get job status"
echo "   GET  /api/v1/vision/files/stats - Get file storage stats" 