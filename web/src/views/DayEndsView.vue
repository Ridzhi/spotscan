<script setup lang="ts">
import { useIndexStore } from '@/stores';
import { useRoute, useRouter } from 'vue-router';
import { timesRange, weekDay, type Weekday } from '@/utils/helpers';
import { ref } from 'vue';
import { type PickerConfirmEventParams, showNotify } from 'vant';
import { api } from '@/utils/api';
import AppPage from '@/components/AppPage.vue';

const store = useIndexStore();
const router = useRouter();
const route = useRoute();

const day = route.params.day as Weekday;

const times = [
  { text: 'По умолчанию', value: '' },
  ...timesRange(9, 23.5).map((v) => ({ text: v, value: v })),
];

const value = ref([store.ends(day) || '']);

async function onConfirm({ selectedValues }: PickerConfirmEventParams) {
  try {
    const v = selectedValues[0] ? `${selectedValues[0]}:00.00` : null;

    const res = await api.updateUserSettings({
      slots: {
        [day]: {
          ...store.user.settings.slots[day],
          ends: v as unknown as string,
        },
      },
    });

    store.user = res.data.data;

    await router.push({ name: 'day', params: { day: day } });
  } catch {
    showNotify('Что то пошло не так');
  }
}

function onCancel() {
  router.push({ name: 'day', params: { day: day } });
}
</script>

<template>
  <AppPage>
    <template #title>
      {{ weekDay(day) }}
    </template>
    <van-picker
      v-model="value"
      title="Время, до"
      confirm-button-text="Подтвердить"
      cancel-button-text="Отменить"
      :columns="times"
      @confirm="onConfirm"
      @cancel="onCancel"
    />
  </AppPage>
</template>
