let current_theme_index = 0;
function setTheme(index) {
    const theme_paths = ['themes/light.css', 'themes/dark.css'];
    let new_theme = document.createElement('link');
    new_theme.id = 'theme';
    new_theme.rel = 'stylesheet';
    new_theme.href = theme_paths[index % theme_paths.length];
    let current_theme = document.querySelector('#theme');
    if (current_theme) {
        current_theme.replaceWith(new_theme);
    } else {
        document.head.append(new_theme);
    }
}

document.body.addEventListener('keyup', function (event) {
    if (event.key === 'PageUp') {
        setTheme(++current_theme_index);
    }
});

setTheme(0);
