console.log('This is from the service worker1');

self.addEventListener('push', (event) => {
    console.log('Service worker push event');
    const data = event.data.json();
    self.registration.showNotification(data.title, {
        title: 'Productivity Tracker',
        options: {
            renotify: true,
        },
        body: data.body,
        icon: '/assets/favicon.svg',
    });
});
