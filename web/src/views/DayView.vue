<script setup lang="ts">
import {useRoute} from "vue-router";
import AppPage from "@/components/AppPage.vue";
import {type Weekday, weekDay} from "@/utils/helpers";
import {useIndexStore} from "@/stores";

const store = useIndexStore();
const route = useRoute();
const day = route.params.day as Weekday;
</script>

<template>
  <AppPage>
    <template #title>
        {{weekDay(route.params.day as Weekday)}}
    </template>

    <van-cell-group inset title="Фильтры">
      <van-cell
          title="Длина окна"
          clickable
          center
          is-link
          :value="store.durationHuman(day)"
          :to="{name: 'defaults-duration'}"
      />
      <van-cell
          title="Время, от"
          clickable
          center
          is-link
          :value="store.starts(day) || 'По умолчанию'"
          :to="{name: 'defaults-starts'}"
      />
      <van-cell
          title="Время, до"
          clickable
          center
          is-link
          :value="store.ends(day) || 'По умолчанию'"
          :to="{name: 'defaults-ends'}"
      />
    </van-cell-group>
  </AppPage>
</template>
