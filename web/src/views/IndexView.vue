<script setup lang="ts">
import AppPage from "@/components/AppPage.vue";
import {useIndexStore} from "@/stores";
import {ref} from "vue";
import {showNotify} from "vant";
import {api} from "@/utils/api";
import MainGap from "@/components/MainGap.vue";
import {type Weekday, weekDay} from "@/utils/helpers";
import type {WindowSettings} from "@/utils/openapi";

const store = useIndexStore();

const days: Weekday[] = [
  'Monday',
  'Tuesday',
  'Wednesday',
  'Thursday',
  'Friday',
  'Saturday',
  'Sunday',
];


const enabled = ref(store.user.settings.enabled);
const daysEnabled = ref(getDaysEnabled());

function getDaysEnabled(): Partial<Record<Weekday, boolean>> {
  let out: Partial<Record<Weekday, boolean>> = {};

  Object.entries(store.user.settings.slots).forEach(([key, value]) => {
    out[key as Weekday] = value.enabled;
  })

  return out;
}

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

async function onSwitchDayEnabled(day: Weekday, enabled: boolean) {
  try {
    const res = await api.updateUserSettings({
      slots: {
        [day]: {
          ...store.user.settings.slots[day],
          enabled
        }
      }
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
          :value="store.durationHuman(undefined)"
          :to="{name: 'defaults-duration'}"
      />
      <van-cell
          title="Время, от"
          clickable
          center
          is-link
          :value="store.starts(undefined)!"
          :to="{name: 'defaults-starts'}"
      />
      <van-cell
          title="Время, до"
          clickable
          center
          is-link
          :value="store.ends(undefined)!"
          :to="{name: 'defaults-ends'}"
      />
    </van-cell-group>

    <MainGap />

    <van-cell-group inset>
      <template v-for="(day, index) in days" :key="index">
        <van-cell
            center
            :title="weekDay(day)"
            clickable
            :to="{name: 'day', params: {day: day}}"
        >
          <template #right-icon>
            <van-switch
                v-model="daysEnabled[day]"
                @click.stop
                @change="onSwitchDayEnabled(day, $event) "
            />
          </template>
        </van-cell>
      </template>
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
