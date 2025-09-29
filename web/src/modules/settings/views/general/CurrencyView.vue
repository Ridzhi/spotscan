<script setup lang="ts">
import AppPage from "@/components/AppPage.vue";
import {ref} from "vue";
import {useUserSettingsStore} from "@/stores";
import {api} from "@/utils/api";
import type {Currency as CurrencyT} from "@/utils/openapi";
import {Currency} from "@/utils/openapi";
import {showNotify} from "vant";

const store = useUserSettingsStore();
const checked = ref<CurrencyT>(store.settings.currency);

async function onChange(currency: CurrencyT) {
  try {
    const res = await api.updateUserSettings({
      currency,
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
      Currency
    </template>

    <van-radio-group v-model="checked" @change="onChange">
      <van-cell-group inset>
        <van-cell
            :title="Currency.Usd"
            clickable
            @click="checked = Currency.Usd"
        >
          <template #right-icon>
            <van-radio :name="Currency.Usd" />
          </template>
        </van-cell>
        <van-cell
            :title="Currency.Ton"
            clickable
            @click="checked = Currency.Ton"
        >
          <template #right-icon>
            <van-radio :name="Currency.Ton" />
          </template>
        </van-cell>
      </van-cell-group>
    </van-radio-group>
  </AppPage>
</template>

