import {createRouter, createWebHistory} from 'vue-router'
import App from '../App.vue'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            name: "app",
            component: App,
            children: [
                {
                    path: "/",
                    component: () => import('../views/Start.vue')
                },
                {
                    path: "/start",
                    component: () => import('../views/Start.vue')
                },
                {
                    path: "/management",
                    component: () => import('../views/Management.vue')
                },
                {
                    path: "/issue-fix",
                    component: () => import('../views/IssueFix.vue')
                },
                {
                    path: "/setting",
                    component: () => import('../views/Setting.vue')
                },
            ]
        }
    ]
})

export default router
