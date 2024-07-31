import { createRouter, createWebHistory } from 'vue-router';
import Ofent from '../views/Ofent.vue';
import Collect from '../views/Collect.vue';
import Setting from '../views/Setting.vue';

const routes = [
    { path: '/', redirect: '/ofent' },
    { path: '/ofent', component: Ofent },
    { path: '/collect', component: Collect },
    { path: '/setting', component: Setting },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;
