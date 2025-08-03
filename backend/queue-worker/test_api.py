#!/usr/bin/env python3
"""
Test script for AI4Thai Queue Worker with YOLO integration
"""
import os

import requests


def test_pest_detection_api():
    """Test the pest detection endpoint"""

    # API endpoint
    url = "http://localhost:8001/api/v1/queue/pest-detection"

    # Test with a sample image (you'll need to provide an actual image)
    test_image_path = "test_image.jpg"

    if not os.path.exists(test_image_path):
        print("❌ Test image not found. Please add a test_image.jpg file.")
        return

    # Prepare the request
    files = {"image": ("test_image.jpg", open(test_image_path, "rb"), "image/jpeg")}
    data = {"crop_type": "rice", "description": "Test image for pest detection"}

    try:
        print("🚀 Testing pest detection API...")
        response = requests.post(url, files=files, data=data, timeout=30)

        if response.status_code == 200:
            result = response.json()
            job_id = result["job_id"]
            print(f"✅ Job created successfully: {job_id}")

            # Check job status
            status_url = f"http://localhost:8001/api/v1/jobs/{job_id}"
            status_response = requests.get(status_url)

            if status_response.status_code == 200:
                status_data = status_response.json()
                print(f"📊 Job Status: {status_data['status']}")

                if status_data.get("result"):
                    result_data = status_data["result"]
                    print(f"🔍 Detected Pests: {result_data.get('detected_pests', [])}")
                    print(f"💬 Message: {result_data.get('message', 'No message')}")
            else:
                print(f"❌ Failed to get job status: {status_response.status_code}")
        else:
            print(f"❌ API request failed: {response.status_code}")
            print(f"Response: {response.text}")

    except Exception as e:
        print(f"❌ Test failed: {e}")
    finally:
        files["image"][1].close()


def test_health_endpoint():
    """Test the health check endpoint"""
    try:
        response = requests.get("http://localhost:8001/health", timeout=5)
        if response.status_code == 200:
            print("✅ Health check passed")
            print(f"Response: {response.json()}")
        else:
            print(f"❌ Health check failed: {response.status_code}")
    except Exception as e:
        print(f"❌ Health check error: {e}")


if __name__ == "__main__":
    print("🧪 AI4Thai Queue Worker Test Suite")
    print("==================================")

    # Test health endpoint first
    test_health_endpoint()
    print()

    # Test pest detection
    test_pest_detection_api()
