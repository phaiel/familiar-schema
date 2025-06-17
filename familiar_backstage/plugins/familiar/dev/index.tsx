import { createDevApp } from '@backstage/dev-utils';
import { familiarPlugin, FamiliarPage } from '../src/plugin';

createDevApp()
  .registerPlugin(familiarPlugin)
  .addPage({
    element: <FamiliarPage />,
    title: 'Root Page',
    path: '/familiar',
  })
  .render();
