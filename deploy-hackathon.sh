#!/bin/bash

# AI4Thai Crop Guardian - Hackathon Deployment Script (Team 10)
# Port: 2090 (2000 + 10*9)

set -e

echo "🚀 Deploying AI4Thai Crop Guardian for Hackathon (Team 10)"
echo "📍 Port: 2090"
echo "🌐 URL: https://api.hackathon2025.ai.in.th/team10-1"

# Set environment variables for hackathon
export API_GATEWAY_PORT=2090
export FRONTEND_PORT=2090
export RUST_LOG=info

# Stop any existing containers
echo "🛑 Stopping existing containers..."
docker-compose down --remove-orphans

# Build and start services
echo "🔨 Building and starting services..."
docker-compose up --build -d

# Wait for services to be ready
echo "⏳ Waiting for services to be ready..."
sleep 10

# Check health
echo "🏥 Checking service health..."
if curl -f http://localhost:2090/health > /dev/null 2>&1; then
    echo "✅ API Gateway is healthy!"
else
    echo "❌ API Gateway health check failed"
    echo "📋 Container logs:"
    docker-compose logs api-gateway
    exit 1
fi

echo ""
echo "🎉 Deployment complete!"
echo "📊 Service Status:"
echo "   API Gateway: http://localhost:2090"
echo "   Health Check: http://localhost:2090/health"
echo "   RabbitMQ Management: http://localhost:15672 (guest/guest)"
echo "   Redis: localhost:6379"
echo ""
echo "🌐 Hackathon URL: https://api.hackathon2025.ai.in.th/team10-1"
echo ""
echo "📝 Useful commands:"
echo "   View logs: docker-compose logs -f"
echo "   Stop services: docker-compose down"
echo "   Restart: docker-compose restart" 