document.addEventListener('click', () => {}, {
    capture: true, // catpures event
    once: true, // event handler removed after triggered
    passive: true, // doesn't cancel default action
});
