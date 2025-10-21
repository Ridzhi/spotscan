import { backButton } from '@telegram-apps/sdk-vue';
import { watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const tabRoutes = ['slots', 'settings'];

export function useBackButton() {
  let offClick: () => void = () => {};
  const route = useRoute();
  const router = useRouter();

  watch(
    () => route.name,
    () => {
      if (tabRoutes.includes(route.name as string)) {
        backButton.hide();
        offClick();
      } else if (!backButton.isVisible()) {
        backButton.show();
        offClick = backButton.onClick(onBackButtonClick);
      }
    },
  );

  async function onBackButtonClick(): Promise<void> {
    await router.go(-1);
  }
}
