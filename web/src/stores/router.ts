import { defineStore } from 'pinia';
import { ref } from 'vue';
import { showNotify } from 'vant';

export const useRouterStore = defineStore('router', () => {
  const successMessageAfter = ref('');

  function onRouterAfterEach() {
    if (successMessageAfter.value != '') {
      const message = successMessageAfter.value;
      successMessageAfter.value = '';

      showNotify({
        type: 'success',
        message,
      });
    }
  }

  return {
    successMessageAfter,
    onRouterAfterEach,
  };
});
