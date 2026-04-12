import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  tutorialSidebar: [
    'intro',
    'coordinate-system',
    'tattvas',
    'animations',
    'scene-and-app',
    'latex-and-text',
    'graphs',
    'arrows-and-vectors',
    'updaters',
    'roadmap',
    {
      type: 'category',
      label: 'Internals',
      items: [
        'internals/architecture',
        'internals/dirty-flags',
        'internals/ecs',
        'internals/renderer',
        'internals/projection',
        'internals/text-and-latex',
      ],
    },
  ],
};

export default sidebars;
