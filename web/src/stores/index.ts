import { defineStore } from 'pinia';
import { api } from '@/utils/api';
import { computed, ref } from 'vue';
import type { User } from '@/utils/openapi';
import type { ConfigProviderTheme } from 'vant';
import { AppTheme } from '@/utils/openapi';
import type { Weekday } from '@/utils/helpers';

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

  const duration = computed(() => {
    return (day: Weekday | undefined) => {
      const v = day
        ? user.value.settings.slots[day].duration
        : user.value.settings.defaults.duration;

      if (v) {
        return parseInt(v).toString();
      }

      return null;
    };
  });

  const starts = computed(() => {
    return (day: Weekday | undefined) => {
      const v = day ? user.value.settings.slots[day].starts : user.value.settings.defaults.starts;

      if (v) {
        return v.slice(0, -5);
      }

      return null;
    };
  });

  const ends = computed(() => {
    return (day: Weekday | undefined) => {
      const v = day ? user.value.settings.slots[day].ends : user.value.settings.defaults.ends;

      if (v) {
        return v.slice(0, -5);
      }

      return null;
    };
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

      return 'light';
    }

    if (user.value.settings.app_theme === AppTheme.Dark) {
      return 'dark';
    }

    return 'light';
  });

  const themeHuman = computed(() => {
    const lookup = {
      [AppTheme.Light]: 'Светлая',
      [AppTheme.Dark]: 'Темная',
      [AppTheme.System]: 'Системная',
    };

    return lookup[user.value.settings.app_theme];
  });

  const durationHuman = computed((): ((day: Weekday | undefined) => string) => {
    return (day: Weekday | undefined): string => {
      const lookup = {
        '1800': 'Пол часа',
        '3600': 'Час',
        '5400': 'Полтора часа',
        '7200': 'Два часа',
        '9000': 'Два с половиной часа',
        '10800': 'Три часа',
      };

      const v = duration.value(day);

      if (v) {
        return lookup[v as keyof typeof lookup] || 'Непонятно';
      }

      return 'По умолчанию';
    };
  });

  return {
    user,
    duration,
    starts,
    ends,
    vanTheme,
    themeHuman,
    durationHuman,
    load,
  };
});
