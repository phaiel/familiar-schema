import {
  createPlugin,
  createRoutableExtension,
} from '@backstage/core-plugin-api';

import { rootRouteRef } from './routes';

export const familiarPlugin = createPlugin({
  id: 'familiar',
  routes: {
    root: rootRouteRef,
  },
});

export const FamiliarPage = familiarPlugin.provide(
  createRoutableExtension({
    name: 'FamiliarPage',
    component: () =>
      import('./components/ExampleComponent').then(m => m.ExampleComponent),
    mountPoint: rootRouteRef,
  }),
);
