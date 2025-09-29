<script setup lang="ts">
import AppPage from "@/components/AppPage.vue";
import {ref} from "vue";
import {useUserSettingsStore} from "@/stores";
import {api} from "@/utils/api";
import type {AppTheme} from "@/utils/openapi";
import {showNotify} from "vant";

const store = useUserSettingsStore();
const checked = ref<AppTheme>(store.settings.app_theme);

async function onChange(app_theme: AppTheme) {
  try {
      const res = await api.updateUserSettings({
        app_theme,
      });

      store.settings = res.data.data;
  } catch {
    showNotify("Something went wrong, please try again later");
  }
}
</script>

<template>
  <AppPage :on-close="null">
    <template #title>
      Theme
    </template>

    <van-radio-group v-model="checked" @change="onChange">
      <van-cell-group inset>
        <van-cell
            title="Light"
            clickable
            @click="checked = 'Light'"
        >
          <template #right-icon>
            <van-radio name="Light" />
          </template>
        </van-cell>
        <van-cell
            title="Dark"
            clickable
            @click="checked = 'Dark'"
        >
          <template #right-icon>
            <van-radio name="Dark" />
          </template>
        </van-cell>
        <van-cell
            title="System"
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

