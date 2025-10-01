<script setup lang="ts">
import AppPage from "@/components/AppPage.vue";
import {api} from "@/utils/api";
import MainGap from "@/components/MainGap.vue";

const data = (await api.getUserSlots()).data.data;
</script>

<template>
<AppPage>
  <template #title>
    Свободные слоты
  </template>

  <template v-for="(item, index) in data" :key="index">
    <van-cell-group :title="`${item.date}`" inset>
      <template v-for="(window, j) in item.windows" :key="j">
        <van-cell
            :title="`${window.window.start.slice(0, 5)} - ${window.window.end.slice(0, 5)}`"
            :value="`#${window.field}`"
        />
      </template>
    </van-cell-group>

    <MainGap />
  </template>
</AppPage>
</template>
