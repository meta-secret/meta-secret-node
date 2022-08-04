import {createRouter, createWebHistory} from "vue-router";
import HomeView from "../views/HomeView.vue";
import SplitView from "../views/SplitView.vue";
import RecoverView from "../views/RecoverView.vue";
import ContactView from "../views/ContactView.vue";
import NotFoundView from "../views/404View.vue";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            name: "home",
            component: HomeView,
        },
        {
            path: "/split",
            name: "split",
            component: SplitView,
        }, {
            path: "/recover",
            name: "recover",
            component: RecoverView,
        },
        {
            path: "/contact",
            name: "contact",
            component: ContactView,
        },
        {
            path: '/404',
            name: 'PageNotExist',
            component: NotFoundView,
        },
        {
            path: "/:catchAll(.*)",
            redirect: "/404"
        }
    ],
});

export default router;
