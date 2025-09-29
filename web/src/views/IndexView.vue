<script setup lang="ts">
import AppPage from "@/components/AppPage.vue";
import {useIndexStore} from "@/stores";
import {ref} from "vue";
import {showNotify} from "vant";
import {api} from "@/utils/api";
import MainGap from "@/components/MainGap.vue";

const store = useIndexStore();

const enabled = ref(store.user.settings.enabled);

async function onSwitchEnabled(enabled: boolean) {
  try {
    const res = await api.updateUserSettings({
      enabled
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
      Настройки
    </template>

    <van-cell-group inset>
      <van-cell center title="Уведомления включены">
        <template #right-icon>
          <van-switch v-model="enabled" @change="onSwitchEnabled " />
        </template>
      </van-cell>
    </van-cell-group>

    <MainGap />

    <van-cell-group inset title="Фильтры по умолчанию">
      <van-cell
          title="Длина окна"
          clickable
          center
          is-link
          :value="store.defaultDurationHuman"
          :to="{name: 'defaults-duration'}"
      />
      <van-cell
          title="Время, от"
          clickable
          center
          is-link
          :value="store.defaultStarts"
          :to="{name: 'defaults-starts'}"
      />
      <van-cell
          title="Время, до"
          clickable
          center
          is-link
          :value="store.defaultEnds"
          :to="{name: 'defaults-ends'}"
      />
    </van-cell-group>

    <MainGap />

    <van-cell-group inset>
      <van-cell
          title="Тема"
          clickable
          is-link
          :value="store.themeHuman"
          :to="{name: 'theme'}"
      />
    </van-cell-group>
  </AppPage>
</template>
