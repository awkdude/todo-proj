self.addEventListener('push', (event) => {
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
