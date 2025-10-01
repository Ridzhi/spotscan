<script setup lang="ts">
import AppPage from "@/components/AppPage.vue";
import {api} from "@/utils/api";
import MainGap from "@/components/MainGap.vue";

const data = (await api.getUserSlots()).data.data;
function formatDate(date: string) {
  let lookup = {
    0: 'Вс',
    1: 'Пн',
    2: 'Вт',
    3: 'Ср',
    4: 'Чт',
    5: 'Пт',
    6: 'Сб',
  }
  const d = new Date(date.slice(0, 10));

  return `${lookup[d.getDay() as keyof typeof lookup]}, ${d.getDate()}`
}
</script>

<template>
<AppPage>
  <template #title>
    Свободные слоты
  </template>

  <template v-for="(item, index) in data" :key="index">
    <van-cell-group :title="`${formatDate(item.date)}`" inset>
      <template v-for="(slot, j) in item.slots" :key="j">
        <van-cell
            :title="`${slot.window.start.slice(0, 5)} - ${slot.window.end.slice(0, 5)}`"
            :value="`#${slot.field}`"
        />
      </template>
    </van-cell-group>

    <MainGap />
  </template>
</AppPage>
</template>
