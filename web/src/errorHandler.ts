import type { AppConfig } from 'vue';
import router from '@/router';

export const errorHandler: AppConfig['errorHandler'] = (err) => {
  const error =
    err instanceof Error ? err.message : typeof err === 'string' ? err : JSON.stringify(err);

  console.log('something is wrong', error);

  router.push({ name: 'error' });
};
