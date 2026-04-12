import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  tutorialSidebar: [
    'intro',
    'coordinate-system',
    {
      type: 'category',
      label: 'Tattvas',
      link: {type: 'doc', id: 'tattvas/index'},
      items: [
        'tattvas/primitives',
        'tattvas/text',
        'tattvas/composite',
        'tattvas/graphs',
        'tattvas/math',
        'tattvas/layout',
        'tattvas/storytelling',
        'tattvas/utility',
      ],
    },
    'animations',
    'scene-and-app',
    'camera',
    'updaters',
    'roadmap',
    {
      type: 'category',
      label: 'Contributing Guide',
      items: [
        'internals/architecture',
        'internals/end-to-end-flow',
        'internals/tattva',
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
