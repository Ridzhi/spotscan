import { createRouter, createWebHistory } from 'vue-router';
import {useRouterStore} from "@/stores/router";

export const routes = [
  {
    path: '/',
    component: () => import('@/views/MainLayout.vue'),
    children: [
      {
        path: '/slots',
        name: 'slots',
        component: () => import('@/views/SlotsView.vue'),
      },
      {
        path: '/settings',
        name: 'settings',
        component: () => import('@/views/SettingsView.vue'),
      },
      {
        path: '',
        name: 'theme',
        component: () => import('@/views/ThemeView.vue'),
      },
      {
        path: '/defaults/duration',
        name: 'defaults-duration',
        component: () => import('@/views/DefaultsDurationView.vue'),
      },
      {
        path: '/defaults/starts',
        name: 'defaults-starts',
        component: () => import('@/views/DefaultsStartsView.vue'),
      },
      {
        path: '/defaults/ends',
        name: 'defaults-ends',
        component: () => import('@/views/DefaultsEndsView.vue'),
      },
      {
        path: '/day/:day',
        name: 'day',
        component: () => import('@/views/DayView.vue'),
      },
      {
        path: '/day/:day/duration',
        name: 'day-duration',
        component: () => import('@/views/DayDurationView.vue'),
      },
      {
        path: '/day/:day/starts',
        name: 'day-starts',
        component: () => import('@/views/DayStartsView.vue'),
      },
      {
        path: '/day/:day/ends',
        name: 'day-ends',
        component: () => import('@/views/DayEndsView.vue'),
      },
      {
        path: '/error',
        name: 'error',
        component: () => import('@/views/ErrorPage.vue'),
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

router.afterEach((to, from, failure) => {
  if (!failure) {
    useRouterStore().onRouterAfterEach();
  }
});

export default router;
