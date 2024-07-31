import { createApp } from "vue";
import App from "./App.vue";
import "normalize.css/normalize.css"
import router from './router';
import "/src/assets/css/view.css"


const app = createApp(App);
app.use(router);
app.mount('#app');

document.addEventListener('contextmenu', (event) => {
    event.preventDefault()
})

document.addEventListener('dragstart', (event) => {
    event.preventDefault();
});

document.addEventListener('selectstart', (event) => {
    event.preventDefault();
});

document.addEventListener('keydown', function (event) {
    if (event.ctrlKey && event.key === 'r') {
        event.preventDefault();
    }
});