<script setup lang="ts">
import {useIndexStore} from "@/stores";
import {useRoute} from "vue-router";
import {weekDay, type Weekday} from "@/utils/helpers";
import AppPage from "@/components/AppPage.vue";
import {ref} from "vue";
import {api} from "@/utils/api";
import {showNotify} from "vant";

const store = useIndexStore();
const route = useRoute();
const day = route.params.day as Weekday;

const checked = ref(store.duration(undefined));

async function onChange(duration: string) {
  try {
    const res = await api.updateUserSettings({
      slots: {
          [day]: {
            duration,
            ...store.user.settings.slots[day],
          }
      },
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
      {{weekDay(day)}}
    </template>
    <van-radio-group v-model="checked" @change="onChange">
      <van-cell-group inset title="Длина окна">
        <van-cell
            title="Пол часа"
            clickable
            @click="checked = '1800'"
        >
          <template #right-icon>
            <van-radio name="1800" />
          </template>
        </van-cell>

        <van-cell
            title="Час"
            clickable
            @click="checked = '3600'"
        >
          <template #right-icon>
            <van-radio name="3600" />
          </template>
        </van-cell>

        <van-cell
            title="Полтора часа"
            clickable
            @click="checked = '5400'"
        >
          <template #right-icon>
            <van-radio name="5400" />
          </template>
        </van-cell>

        <van-cell
            title="Два часа"
            clickable
            @click="checked = '7200'"
        >
          <template #right-icon>
            <van-radio name="7200" />
          </template>
        </van-cell>

        <van-cell
            title="Два с половиной часа"
            clickable
            @click="checked = '9000'"
        >
          <template #right-icon>
            <van-radio name="9000" />
          </template>
        </van-cell>

        <van-cell
            title="Три часа"
            clickable
            @click="checked = '10800'"
        >
          <template #right-icon>
            <van-radio name="10800" />
          </template>
        </van-cell>
      </van-cell-group>
    </van-radio-group>
  </AppPage>
</template>
