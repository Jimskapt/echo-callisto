const js = import('./echo_callisto.js');

js.then((js) => {
    js.greet("World");
});
