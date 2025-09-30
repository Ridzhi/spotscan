<script setup lang="ts">
import {timesRange} from "@/utils/helpers";
import {useRouter} from "vue-router";
import {useIndexStore} from "@/stores";
import {ref} from "vue";
import {type PickerConfirmEventParams, showNotify} from "vant";
import {api} from "@/utils/api";

const store = useIndexStore();
const router = useRouter();
const times = timesRange(9, 23.5).map(v => ({text: v, value: v}));

const value = ref([store.ends(undefined)!]);

async function onConfirm({selectedValues}: PickerConfirmEventParams) {
  try {
    const res = await api.updateUserSettings({
      window_default_ends: `${selectedValues[0]}:00.00`,
    });

    store.user = res.data.data;

    await router.push({name: 'index'});
  } catch {
    showNotify("Что то пошло не так");
  }
}

function onCancel() {
  router.push({name: 'index'});
}

</script>

<template>
  <van-picker
      v-model="value"
      title="Время, до"
      confirm-button-text="Подтвердить"
      cancel-button-text="Отменить"
      :columns="times"
      @confirm="onConfirm"
      @cancel="onCancel"
  />
</template>
