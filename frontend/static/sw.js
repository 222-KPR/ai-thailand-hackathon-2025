// AI4Thai Crop Guardian - Service Worker
// Provides offline functionality and caching for the PWA

const CACHE_NAME = 'ai4thai-crop-guardian-v1';
const STATIC_CACHE_NAME = 'ai4thai-static-v1';
const DYNAMIC_CACHE_NAME = 'ai4thai-dynamic-v1';

// Static assets to cache
const STATIC_ASSETS = [
  '/',
  '/index.html',
  '/static/styles.css',
  '/static/manifest.json',
  '/pkg/ai_vision_frontend.js',
  '/pkg/ai_vision_frontend_bg.wasm',
  // Add other static assets as needed
];

// API endpoints that should be cached
const CACHEABLE_API_ENDPOINTS = [
  '/api/health',
  // Add other cacheable endpoints
];

// Install event - cache static assets
self.addEventListener('install', (event) => {
  console.log('Service Worker: Installing...');
  
  event.waitUntil(
    caches.open(STATIC_CACHE_NAME)
      .then((cache) => {
        console.log('Service Worker: Caching static assets');
        return cache.addAll(STATIC_ASSETS);
      })
      .then(() => {
        console.log('Service Worker: Static assets cached');
        return self.skipWaiting();
      })
      .catch((error) => {
        console.error('Service Worker: Failed to cache static assets', error);
      })
  );
});

// Activate event - clean up old caches
self.addEventListener('activate', (event) => {
  console.log('Service Worker: Activating...');
  
  event.waitUntil(
    caches.keys()
      .then((cacheNames) => {
        return Promise.all(
          cacheNames.map((cacheName) => {
            if (cacheName !== STATIC_CACHE_NAME && 
                cacheName !== DYNAMIC_CACHE_NAME &&
                cacheName !== CACHE_NAME) {
              console.log('Service Worker: Deleting old cache', cacheName);
              return caches.delete(cacheName);
            }
          })
        );
      })
      .then(() => {
        console.log('Service Worker: Activated');
        return self.clients.claim();
      })
  );
});

// Fetch event - handle requests with caching strategy
self.addEventListener('fetch', (event) => {
  const { request } = event;
  const url = new URL(request.url);
  
  // Skip non-GET requests
  if (request.method !== 'GET') {
    return;
  }
  
  // Handle static assets (cache first)
  if (STATIC_ASSETS.some(asset => url.pathname === asset || url.pathname.endsWith(asset))) {
    event.respondWith(cacheFirst(request, STATIC_CACHE_NAME));
    return;
  }
  
  // Handle API requests
  if (url.pathname.startsWith('/api/')) {
    if (CACHEABLE_API_ENDPOINTS.some(endpoint => url.pathname.startsWith(endpoint))) {
      // Network first for cacheable API endpoints
      event.respondWith(networkFirst(request, DYNAMIC_CACHE_NAME));
    } else {
      // Network only for non-cacheable API endpoints
      event.respondWith(networkOnly(request));
    }
    return;
  }
  
  // Handle other requests (stale while revalidate)
  event.respondWith(staleWhileRevalidate(request, DYNAMIC_CACHE_NAME));
});

// Background sync for failed requests
self.addEventListener('sync', (event) => {
  console.log('Service Worker: Background sync', event.tag);
  
  if (event.tag === 'background-sync-chat') {
    event.waitUntil(handleBackgroundSync());
  }
});

// Push notifications (future enhancement)
self.addEventListener('push', (event) => {
  console.log('Service Worker: Push received', event);
  
  const options = {
    body: event.data ? event.data.text() : 'New notification from AI4Thai Crop Guardian',
    icon: '/icons/icon-192x192.png',
    badge: '/icons/icon-72x72.png',
    vibrate: [200, 100, 200],
    data: {
      dateOfArrival: Date.now(),
      primaryKey: 1
    },
    actions: [
      {
        action: 'explore',
        title: 'Open App',
        icon: '/icons/action-explore.png'
      },
      {
        action: 'close',
        title: 'Close',
        icon: '/icons/action-close.png'
      }
    ]
  };
  
  event.waitUntil(
    self.registration.showNotification('AI4Thai Crop Guardian', options)
  );
});

// Notification click handler
self.addEventListener('notificationclick', (event) => {
  console.log('Service Worker: Notification click', event);
  
  event.notification.close();
  
  if (event.action === 'explore') {
    event.waitUntil(
      clients.openWindow('/')
    );
  }
});

// Caching strategies

// Cache first - for static assets
async function cacheFirst(request, cacheName) {
  try {
    const cache = await caches.open(cacheName);
    const cachedResponse = await cache.match(request);
    
    if (cachedResponse) {
      return cachedResponse;
    }
    
    const networkResponse = await fetch(request);
    
    if (networkResponse.ok) {
      cache.put(request, networkResponse.clone());
    }
    
    return networkResponse;
  } catch (error) {
    console.error('Cache first strategy failed:', error);
    return new Response('Offline - Asset not available', { 
      status: 503,
      statusText: 'Service Unavailable'
    });
  }
}

// Network first - for API endpoints
async function networkFirst(request, cacheName) {
  try {
    const networkResponse = await fetch(request);
    
    if (networkResponse.ok) {
      const cache = await caches.open(cacheName);
      cache.put(request, networkResponse.clone());
    }
    
    return networkResponse;
  } catch (error) {
    console.log('Network failed, trying cache:', error);
    
    const cache = await caches.open(cacheName);
    const cachedResponse = await cache.match(request);
    
    if (cachedResponse) {
      return cachedResponse;
    }
    
    return new Response(JSON.stringify({
      error: 'Offline - Data not available',
      cached: false
    }), {
      status: 503,
      statusText: 'Service Unavailable',
      headers: {
        'Content-Type': 'application/json'
      }
    });
  }
}

// Network only - for non-cacheable requests
async function networkOnly(request) {
  try {
    return await fetch(request);
  } catch (error) {
    console.error('Network only request failed:', error);
    return new Response(JSON.stringify({
      error: 'Network unavailable',
      message: 'Please check your internet connection'
    }), {
      status: 503,
      statusText: 'Service Unavailable',
      headers: {
        'Content-Type': 'application/json'
      }
    });
  }
}

// Stale while revalidate - for general content
async function staleWhileRevalidate(request, cacheName) {
  const cache = await caches.open(cacheName);
  const cachedResponse = await cache.match(request);
  
  // Fetch from network in background
  const networkResponsePromise = fetch(request)
    .then((networkResponse) => {
      if (networkResponse.ok) {
        cache.put(request, networkResponse.clone());
      }
      return networkResponse;
    })
    .catch((error) => {
      console.log('Network request failed:', error);
      return null;
    });
  
  // Return cached response immediately if available
  if (cachedResponse) {
    return cachedResponse;
  }
  
  // Otherwise wait for network response
  const networkResponse = await networkResponsePromise;
  
  if (networkResponse) {
    return networkResponse;
  }
  
  // Fallback response
  return new Response('Offline - Content not available', {
    status: 503,
    statusText: 'Service Unavailable'
  });
}

// Handle background sync for failed requests
async function handleBackgroundSync() {
  try {
    // Get pending requests from IndexedDB (would need to be implemented)
    const pendingRequests = await getPendingRequests();
    
    for (const request of pendingRequests) {
      try {
        const response = await fetch(request.url, request.options);
        
        if (response.ok) {
          // Remove successful request from pending queue
          await removePendingRequest(request.id);
          
          // Notify clients of successful sync
          const clients = await self.clients.matchAll();
          clients.forEach(client => {
            client.postMessage({
              type: 'BACKGROUND_SYNC_SUCCESS',
              data: { requestId: request.id }
            });
          });
        }
      } catch (error) {
        console.error('Background sync failed for request:', request.id, error);
      }
    }
  } catch (error) {
    console.error('Background sync handler failed:', error);
  }
}

// Helper functions for IndexedDB operations (placeholder)
async function getPendingRequests() {
  // Implementation would use IndexedDB to store/retrieve pending requests
  return [];
}

async function removePendingRequest(requestId) {
  // Implementation would remove request from IndexedDB
  console.log('Removing pending request:', requestId);
}

// Utility functions
function isNavigationRequest(request) {
  return request.mode === 'navigate';
}

function isStaticAsset(url) {
  return url.pathname.match(/\.(js|css|png|jpg|jpeg|gif|svg|woff|woff2|ttf|eot|ico|wasm)$/);
}

// Error handling for unhandled promise rejections
self.addEventListener('unhandledrejection', (event) => {
  console.error('Service Worker: Unhandled promise rejection:', event.reason);
  event.preventDefault();
});