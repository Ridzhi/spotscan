<script setup lang="ts">
import AppPage from "@/components/AppPage.vue";
import {ref} from "vue";
import {api} from "@/utils/api";
import type {AppTheme} from "@/utils/openapi";
import {showNotify} from "vant";
import {useIndexStore} from "@/stores";

const store = useIndexStore();
const checked = ref<AppTheme>(store.user.settings.app_theme);

async function onChange(app_theme: AppTheme) {
  try {
    const res = await api.updateUserSettings({
      app_theme,
    });

    store.user = res.data.data;
  } catch {
    showNotify("Что то пошло не так");
  }
}
</script>

<template>
  <AppPage>
    <template #title>
      Тема
    </template>

    <van-radio-group v-model="checked" @change="onChange">
      <van-cell-group inset>
        <van-cell
            title="Светлая"
            clickable
            @click="checked = 'Light'"
        >
          <template #right-icon>
            <van-radio name="Light" />
          </template>
        </van-cell>
        <van-cell
            title="Темная"
            clickable
            @click="checked = 'Dark'"
        >
          <template #right-icon>
            <van-radio name="Dark" />
          </template>
        </van-cell>
        <van-cell
            title="Системная"
            clickable
            @click="checked = 'System'"
        >
          <template #right-icon>
            <van-radio name="System" />
          </template>
        </van-cell>
      </van-cell-group>
    </van-radio-group>
  </AppPage>
</template>

