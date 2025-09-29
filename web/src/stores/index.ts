import {defineStore} from "pinia";
import {api} from "@/utils/api";
import {computed, ref} from "vue";
import type {User} from "@/utils/openapi";
import type {ConfigProviderTheme} from "vant";
import {AppTheme} from "@/utils/openapi";

export const useIndexStore = defineStore('index', () => {
    let loaded = false;
    const themeTrigger = ref(0);
    const user = ref({} as User);
    const colorScheme = document.querySelector('meta[name="color-scheme"]');
    const lightScheme = window.matchMedia('(prefers-color-scheme: light)');
    const darkScheme = window.matchMedia('(prefers-color-scheme: dark)');

    lightScheme.addEventListener('change', (e) => {
        if (e.matches && user.value.settings.app_theme === 'System') {
            themeTrigger.value++;
        }
    });

    darkScheme.addEventListener('change', (e) => {
        if (e.matches && user.value.settings.app_theme === 'System') {
            themeTrigger.value++;
        }
    });

    async function load() {
        if (loaded) {
            return;
        }

        const res = await api.getUser();
        user.value = res.data.data;
        switch (user.value.settings.app_theme) {
            case 'Light':
                colorScheme?.setAttribute('content', 'light');
                break;
            case 'Dark':
                colorScheme?.setAttribute('content', 'dark');
                break;
            case 'System':
                colorScheme?.setAttribute('content', 'light dark');
                break;
        }

        loaded = true;
    }

    const defaultDuration = computed(() => {
        return '' + parseInt(user.value.settings.defaults.duration);
    });

    const defaultStarts = computed(() => {
        return user.value.settings.defaults.starts.slice(0, -5);
    });

    const defaultEnds = computed(() => {
        return user.value.settings.defaults.ends.slice(0, -5);
    });

    const vanTheme = computed<ConfigProviderTheme>(() => {
        // use to trigger vantTheme change
        const _ = themeTrigger.value;

        if (user.value.settings.app_theme === AppTheme.System) {
            if (lightScheme.matches) {
                return 'light';
            }

            if (darkScheme.matches) {
                return 'dark';
            }

            return 'light'
        }

        if (user.value.settings.app_theme === AppTheme.Dark) {
            return 'dark';
        }

        return 'light';
    });

    const themeHuman = computed(() => {
        const lookup = {
            [AppTheme.Light]: "Светлая",
            [AppTheme.Dark]: "Темная",
            [AppTheme.System]: "Системная",
        }

        return lookup[user.value.settings.app_theme];
    })

    const defaultDurationHuman = computed(() => {
        const lookup = {
            '1800': 'Пол часа',
            '3600': 'Час',
            '5400': 'Полтора часа',
            '7200': 'Два часа',
            '9000': 'Два с половиной часа',
            '10800': 'Три часа',
        };

        return lookup[defaultDuration.value as keyof typeof lookup] || 'Непонятно';
    })

    return {
        user,
        defaultDuration,
        defaultStarts,
        defaultEnds,
        vanTheme,
        themeHuman,
        defaultDurationHuman,
        load,
    }
});
